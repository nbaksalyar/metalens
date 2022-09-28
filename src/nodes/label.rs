//! Generates BPF code for displaying data.

use std::{rc::Rc, time::Duration};

use futures::{future, StreamExt};
use inkwell::{
    values::{AnyValue, BasicValueEnum, IntValue, PointerValue},
    AddressSpace, AtomicOrdering, AtomicRMWBinOp, IntPredicate,
};
use serde::{Deserialize, Serialize};
use tokio::time;
use tracing::{info, warn};

use super::{CodegenCtx, Node, NodeProperties, OutputType};
use crate::{
    codegen::{gen_bpf_helper, CodegenError},
    dsl::NodeId,
    formulas::ExprType,
    runtime::{LoadedState, RuntimeError},
    ws::{Message, MsgChannelTx},
};
use crate::{formulas, nodes::uprobe::UProbeResult};

// FIXME: use consts from redbpf
const BPF_MAP_TYPE_RINGBUF: u64 = 27;
const BPF_MAP_TYPE_HASH: u64 = 1;
const BPF_MAP_TYPE_ARRAY: u64 = 2;

#[derive(Debug)]
pub struct LabelNode {
    id: NodeId,
    props: NodeProperties,
}

#[derive(Serialize, Deserialize)]
struct NodeValue {
    id: NodeId,
    value: String,
}

impl LabelNode {
    pub fn new(id: NodeId, props: NodeProperties) -> Self {
        Self { id, props }
    }

    pub fn load(
        &self,
        prog_state: &mut LoadedState,
        out_stream: MsgChannelTx,
    ) -> Result<(), RuntimeError> {
        if let Some(counters_map) = prog_state.prog.map("counters") {
            let counters_map = counters_map.clone();

            let mut counters_reader_interval = time::interval(Duration::from_millis(500));

            let node_id = self.id;
            let out_stream = out_stream.clone();
            let drop_state = prog_state.drop_state.clone();

            tokio::spawn(async move {
                let counters_array = redbpf::Array::<u32>::new(&counters_map).unwrap();
                let mut prev_counter_value = 0;

                loop {
                    counters_reader_interval.tick().await;

                    let counter_value = if let Some(counter_value) = counters_array.get(0) {
                        counter_value
                    } else {
                        warn!("failed to get a counter value");
                        continue;
                    };

                    if prev_counter_value != counter_value {
                        if let Err(e) = out_stream.unbounded_send(Message {
                            action: "value".to_owned(),
                            payload: serde_json::to_string(&NodeValue {
                                id: node_id,
                                value: counter_value.to_string(),
                            })
                            .expect("failed to construct json"),
                        }) {
                            warn!("failed to send a message: {:?}", e);
                            break;
                        }
                        prev_counter_value = counter_value;
                    }

                    // FIXME: find a more efficient solution
                    if *drop_state.lock().unwrap() {
                        info!("stopping counters - prog has been dropped");
                        break;
                    }
                }
            });
        }

        // TODO: Fixme - use better API
        if let Some(ringbuf_stream) = prog_state.prog.ringbufs.remove("ringbuf") {
            let node_id = self.id;

            let out_stream = out_stream.clone();
            let drop_state = prog_state.drop_state.clone();

            let fut = ringbuf_stream.for_each(move |events| {
                // TODO: deserialize event properly.
                // for now we assume data is u64
                for event in events {
                    let pid: Result<[u8; 8], _> = (&*event).try_into();

                    let deserialized = u64::from_ne_bytes(pid.unwrap());

                    if let Err(e) = out_stream.clone().unbounded_send(Message {
                        action: "value".to_owned(),
                        payload: serde_json::to_string(&NodeValue {
                            id: node_id,
                            value: deserialized.to_string(),
                        })
                        .expect("failed to construct json"),
                    }) {
                        warn!("failed to send a message: {:?}", e);
                    }

                    if *drop_state.lock().unwrap() {
                        info!("stopping ringbuf sender - prog has been dropped");
                        break;
                    }
                }

                future::ready(())
            });
            tokio::spawn(fut);
        }

        Ok(())
    }

    fn codegen_counter(&self, ctx: &mut CodegenCtx) {
        // use a BPF hash map
        let i32_ty = ctx.llvm_context.i32_type();
        let bpf_map_def = ctx.llvm_context.struct_type(
            &[
                i32_ty.into(), // type
                i32_ty.into(), // key_size
                i32_ty.into(), // value_size
                i32_ty.into(), // max_entries
                i32_ty.into(), // map_flags
                i32_ty.into(), // inner_map_idx
                i32_ty.into(), // numa_mode
            ],
            false,
        );

        let array = ctx
            .module
            .add_global(bpf_map_def, Some(AddressSpace::Global), "counters");

        array.set_initializer(&bpf_map_def.const_named_struct(&[
            i32_ty.const_int(BPF_MAP_TYPE_ARRAY, false).into(),
            i32_ty.const_int(4, false).into(),
            i32_ty.const_int(4, false).into(),  // value_size
            i32_ty.const_int(16, false).into(), // max_entries
            i32_ty.const_zero().into(),
            i32_ty.const_zero().into(),
            i32_ty.const_zero().into(),
        ]));

        array.set_section("maps/counters");

        // value = bpf_map_lookup_elem(&my_map, &index);
        ctx.builder.position_at_end(ctx.allocs_block);
        let zeroth_idx = ctx.builder.build_alloca(i32_ty, "zeroth_idx");

        ctx.builder.position_at_end(ctx.current_block);
        ctx.builder.build_store(zeroth_idx, i32_ty.const_zero());

        let array_0_ptr = self.bpf_map_lookup_elem(ctx, array.as_pointer_value(), zeroth_idx);
        // if (array_0_ptr == null) return;
        let null_cmp = ctx.builder.build_pointer_compare(
            IntPredicate::NE,
            array_0_ptr,
            i32_ty.ptr_type(AddressSpace::Generic).const_null(),
            "nullcmp",
        );

        let then_block = ctx
            .llvm_context
            .append_basic_block(ctx.func, "map_lookup_success");
        ctx.builder
            .build_conditional_branch(null_cmp, then_block, ctx.func_exit);

        ctx.set_current_block(then_block);

        let _atomic_add_res = ctx
            .builder
            .build_atomicrmw(
                AtomicRMWBinOp::Add,
                array_0_ptr,
                i32_ty.const_int(1, false).into(),
                AtomicOrdering::SequentiallyConsistent,
            )
            .expect("atomricrw failed");

        // let event = self.bpf_map_lookup_elem(ctx, array.as_pointer_value(), i32_ty.const_zero());
    }

    /// Generates LLVM IR from Metalens AST
    pub fn codegen(&self, ctx: &mut CodegenCtx, inputs: &[&Node]) -> Result<(), CodegenError> {
        // context required:
        // - previous node output type
        // - variables used in my properties

        let output_formula = &self
            .props
            .props
            .get("value")
            .ok_or_else(|| CodegenError::ExpectedProperty("value"))?;

        // generate a ring buffer map
        // TODO: move the struct/ringbuf generation to a later phase to make sure we can account for multiple sink nodes.

        // assert_eq!(inputs.len(), 1); // TODO: support multiple inputs
        // let input_node = inputs[0];
        // let output_type = input_node.output_type();

        // FIXME: filter needs to mutate its output type but we can't borrow it as mutable because
        // we need to have access to neighbours. so just hardcode its output type for now
        let prev_node_output_type = Rc::new(OutputType::Struct(Box::new(UProbeResult {})));

        let output_expr = formulas::generate_expr_code(
            ctx,
            output_formula,
            Some(prev_node_output_type.as_ref()),
        )?;

        if output_expr.ty == ExprType::Counter {
            // handle counter as a special case.
            // TODO: fixme - make this less ugly.
            self.codegen_counter(ctx);
            return Ok(());
        }

        // define an output struct
        // TODO: generate a struct depending on the output expr type.
        let i32_ty = ctx.llvm_context.i32_type();
        let event_struct = ctx
            .llvm_context
            .struct_type(&[ctx.llvm_context.i64_type().into()], false);

        let bpf_map_def = ctx.llvm_context.struct_type(
            &[
                i32_ty.into(), // type
                i32_ty.into(), // key_size
                i32_ty.into(), // value_size
                i32_ty.into(), // max_entries
                i32_ty.into(), // map_flags
                i32_ty.into(), // inner_map_idx
                i32_ty.into(), // numa_mode
            ],
            false,
        );

        // create a ringbuf map
        let ringbuf = ctx
            .module
            .add_global(bpf_map_def, Some(AddressSpace::Global), "ringbuf");
        ringbuf.set_initializer(&bpf_map_def.const_named_struct(&[
            i32_ty.const_int(BPF_MAP_TYPE_RINGBUF, false).into(),
            i32_ty.const_zero().into(),
            i32_ty.const_zero().into(),
            i32_ty.const_int(64 * 4096, false).into(), // max_entries
            i32_ty.const_zero().into(),
            i32_ty.const_zero().into(),
            i32_ty.const_zero().into(),
        ]));
        ringbuf.set_section("maps/ringbuf");

        // generate calls to fill the ring buffer
        // struct event_ty *event = bpf_ring_reserve(..);
        let event = self.bpf_ring_reserve(
            ctx,
            ringbuf.as_pointer_value(),
            event_struct.size_of().unwrap(),
        );

        // if (event == null) return;
        let null_cmp = ctx.builder.build_pointer_compare(
            IntPredicate::NE,
            event,
            event_struct.ptr_type(AddressSpace::Generic).const_null(),
            "nullcmp",
        );

        let then_block = ctx
            .llvm_context
            .append_basic_block(ctx.func, "ringbuf_success");
        ctx.builder
            .build_conditional_branch(null_cmp, then_block, ctx.func_exit);

        ctx.set_current_block(then_block);

        // reserve = bpf_ringbuf_reserve(&ringbuf, sizeof(event), 0)
        // if (!reserve)
        //   return 0;
        // struct event *e;
        // e->pid = pid_tgid >> 32;
        // bpf_ringbuf_submit(event, 0);
        // event_struct.ptr_type(AddressSpace::Generic)
        let event_0_ptr = unsafe {
            ctx.builder.build_in_bounds_gep(
                event,
                &[
                    ctx.llvm_context.i32_type().const_zero().into(),
                    // ctx.llvm_context.i32_type().const_zero().into(),
                ],
                "event_0_ptr",
            )
        };
        ctx.builder.build_store::<BasicValueEnum>(
            event_0_ptr,
            output_expr
                .value
                .try_into()
                .expect("could not convert value type"),
        );

        self.bpf_ring_submit(ctx, event);

        Ok(())

        // TODO: check the linux version and see if ringbufs are supported
    }

    pub fn output_type(&self) -> Rc<OutputType> {
        // = same output type as my input
        // self.output_type
        Rc::new(OutputType::Void)
    }

    pub fn bpf_ring_reserve<'a>(
        &self,
        ctx: &mut CodegenCtx<'a>,
        ringbuf: PointerValue<'a>,
        size: IntValue<'a>,
    ) -> PointerValue<'a> {
        let bpf_ring_reserve = gen_bpf_helper(
            ctx,
            131,
            ctx.llvm_context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .fn_type(
                    &[
                        ctx.llvm_context
                            .i8_type()
                            .ptr_type(AddressSpace::Generic)
                            .into(), // ringbuf
                        ctx.llvm_context.i64_type().into(), // size
                        ctx.llvm_context.i64_type().into(), // flags
                    ],
                    false,
                ),
        );

        ctx.builder
            .build_call(
                bpf_ring_reserve,
                &[
                    ringbuf.into(),
                    size.into(),
                    ctx.llvm_context.i64_type().const_zero().into(),
                ],
                "reserve_result",
            )
            .as_any_value_enum()
            .into_pointer_value()
    }

    // TODO: move to the helpers module
    pub fn bpf_map_lookup_elem<'a>(
        &self,
        ctx: &mut CodegenCtx<'a>,
        array_map: PointerValue<'a>,
        index: PointerValue<'a>,
    ) -> PointerValue<'a> {
        let bpf_ring_reserve = gen_bpf_helper(
            ctx,
            1, // BPF_MAP_LOOKUP_ELEM
            ctx.llvm_context
                .i32_type()
                .ptr_type(AddressSpace::Generic)
                .fn_type(
                    &[
                        ctx.llvm_context
                            .i8_type()
                            .ptr_type(AddressSpace::Generic)
                            .into(), // map
                        ctx.llvm_context
                            .i8_type()
                            .ptr_type(AddressSpace::Generic)
                            .into(), // index
                    ],
                    false,
                ),
        );

        ctx.builder
            .build_call(
                bpf_ring_reserve,
                &[
                    array_map.into(),
                    index.into(),
                    ctx.llvm_context.i64_type().const_zero().into(),
                ],
                "map_lookup_result",
            )
            .as_any_value_enum()
            .into_pointer_value()
    }

    pub fn bpf_ring_submit<'a>(&self, ctx: &mut CodegenCtx<'a>, value: PointerValue<'a>) {
        let bpf_ring_submit = gen_bpf_helper(
            ctx,
            132,
            ctx.llvm_context.void_type().fn_type(
                &[
                    ctx.llvm_context
                        .i8_type()
                        .ptr_type(AddressSpace::Generic)
                        .into(), // data
                    ctx.llvm_context.i64_type().into(), // flags
                ],
                false,
            ),
        );

        ctx.builder.build_call(
            bpf_ring_submit,
            &[
                value.into(),
                ctx.llvm_context.i64_type().const_zero().into(),
            ],
            "submit_res",
        );
    }
}
