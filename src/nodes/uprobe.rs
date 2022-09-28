//! Implements eBPF UProbe feature.

use std::rc::Rc;

use inkwell::{values::AnyValue, AddressSpace};
use tracing::info;
use usdt_reader::Context as UsdtContext;

use super::{CodegenCtx, ExprValue, Node, NodeProperties, OutputStruct, OutputType};
use crate::codegen::{gen_bpf_helper, CodegenError};
use crate::dsl::NodeId;
use crate::formulas::ExprType;
use crate::runtime::{LoadedState, RuntimeError};
use crate::ws::MsgChannelTx;

#[derive(Debug)]
struct UProbePoint {
    fn_name: Option<String>,
    offset: u64,
    semaphore_offset: u64,
}

#[derive(Debug)]
pub struct UProbe {
    id: NodeId,
    props: NodeProperties,
    output_type: Rc<OutputType>,
    trace_point: Option<UProbePoint>,
}

impl UProbe {
    pub fn new(id: NodeId, props: NodeProperties) -> Self {
        Self {
            id,
            props,
            output_type: Rc::new(OutputType::Struct(Box::new(UProbeResult {}))),
            trace_point: None,
        }
    }

    /// Loads UProbes from the generated program.
    pub fn load(
        &self,
        state: &mut LoadedState,
        _out_stream: MsgChannelTx,
    ) -> Result<(), RuntimeError> {
        let prog_prop = self
            .props
            .get("program")
            .ok_or(RuntimeError::ExpectedProperty("program"))?;

        // TODO: use formulas evaluator
        let prog_name = prog_prop
            .term()
            .ok_or_else(|| {
                RuntimeError::Other("non-term value used for the `probe` property".to_string())
            })?
            .str_value()
            .ok_or_else(|| {
                RuntimeError::Other("expected a string value for `probe`".to_string())
            })?;

        let uprobe = state.prog.uprobe_mut("mlens").ok_or_else(|| {
            RuntimeError::Other("expected uprobe/mlens. no compiled bpf program?".to_string())
        })?;

        let tracepoints = self.generate_trace_points()?;

        for tp in tracepoints {
            info!("Attaching trace point to [{}]: {:?}", prog_name, tp);

            uprobe
                .attach_uprobe_with_semaphore(
                    tp.fn_name.as_ref().map(|s| s.as_str()),
                    tp.offset,
                    tp.semaphore_offset as u32,
                    &prog_name,
                    None,
                )
                .map_err(|e| RuntimeError::Other(format!("failed to attach uprobe: {:?}", e)))?;
        }

        Ok(())
    }

    fn generate_trace_points(&self) -> Result<Vec<UProbePoint>, RuntimeError> {
        let prog_prop = self
            .props
            .get("program")
            .ok_or(RuntimeError::ExpectedProperty("program"))?;

        // TODO: use formulas evaluator
        let prog_name = prog_prop
            .term()
            .ok_or_else(|| {
                RuntimeError::Other("non-term value used for the `program` property".to_string())
            })?
            .str_value()
            .ok_or_else(|| {
                RuntimeError::Other("expected a string value for `program`".to_string())
            })?;

        if let Some(probe_formula) = self.props.get("probe") {
            // TODO: use formulas evaluator
            let probe_name = probe_formula
                .term()
                .ok_or_else(|| {
                    RuntimeError::Other("non-term value used for the `probe` property".to_string())
                })?
                .str_value()
                .ok_or_else(|| {
                    RuntimeError::Other("expected a string value for `probe`".to_string())
                })?;

            let bin_data = std::fs::read(prog_name)?;
            let context = UsdtContext::new(&bin_data)
                .map_err(|e| RuntimeError::Other(format!("usdt reader error: {:?}", e)))?;

            let uprobes = context
                .probes()
                .map_err(|e| RuntimeError::Other(format!("usdt reader error: {:?}", e)))?
                .map(|probe| probe.unwrap()) // FIXME: proper error handling
                .filter(|probe| probe.probe_name == probe_name)
                .map(|probe| UProbePoint {
                    fn_name: None,
                    offset: probe.sh_addr,
                    semaphore_offset: probe.semaphore_offset,
                })
                .collect::<Vec<_>>();

            if uprobes.is_empty() {
                return Err(RuntimeError::Other(format!(
                    "USDT probe {} not found",
                    probe_name
                )));
            }

            info!("Attaching to static probe {}", probe_name);

            Ok(uprobes)
        } else {
            let func_prop = self
                .props
                .get("function")
                .ok_or(RuntimeError::ExpectedProperty("function"))?;

            // TODO: use formulas evaluator
            let func_name = func_prop
                .term()
                .ok_or_else(|| {
                    RuntimeError::Other(
                        "non-term value used for the `function` property".to_string(),
                    )
                })?
                .str_value()
                .ok_or_else(|| {
                    RuntimeError::Other("expected a string value for `function`".to_string())
                })?;

            Ok(vec![UProbePoint {
                offset: 0,
                semaphore_offset: 0,
                fn_name: Some(func_name.to_owned()),
            }])
        }
    }

    /// Generates LLVM IR from Metalens AST
    pub fn codegen(&self, ctx: &mut CodegenCtx, _inputs: &[&Node]) -> Result<(), CodegenError> {
        // context required:
        // - my own output type
        // - uprobe type used in my property

        // ctx.builder
        Ok(())
    }

    /// Returns the node output type.
    pub fn output_type(&self) -> Rc<OutputType> {
        // = same output type as my input
        self.output_type.clone()
    }
}

#[derive(Clone)]
pub struct UProbeResult {}

impl UProbeResult {
    pub fn codegen_proc_name<'a>(&self, ctx: &mut CodegenCtx<'a>) -> ExprValue<'a> {
        // char comm[TASK_COMM_LEN];
        // bpf_get_current_comm(&comm, sizeof(comm));

        // bpf_get_current_comm signature:
        // let f: unsafe extern "C" fn(buf: *mut ::cty::c_void, size_of_buf: __u32) -> ::cty::c_long =
        //      ::core::mem::transmute(16usize);

        // TODO: use ptr_sized_int_type instead
        // TODO: use enum with helper names
        // TODO: move to helper function
        let bpf_get_current_comm = gen_bpf_helper(
            ctx,
            16,
            ctx.llvm_context.i64_type().fn_type(
                &[
                    ctx.llvm_context
                        .i8_type()
                        .ptr_type(AddressSpace::Generic)
                        .into(),
                    ctx.llvm_context.i32_type().into(),
                ],
                false,
            ),
        );

        ctx.builder.position_at_end(ctx.allocs_block);
        let alloca = ctx
            .builder
            .build_alloca(ctx.llvm_context.i8_type().array_type(16), "comm_name");

        ctx.builder.position_at_end(ctx.current_block);

        let _call_ret_val = ctx
            .builder
            .build_call(
                bpf_get_current_comm,
                &[
                    alloca.into(),
                    ctx.llvm_context.i32_type().const_int(16, false).into(),
                ],
                "bpf_get_current_comm",
            )
            .as_any_value_enum();

        //-------------
        /*
        let bpf_printk = gen_bpf_helper(
            ctx,
            6,
            ctx.llvm_context.void_type().fn_type(
                &[
                    ctx.llvm_context
                        .i8_type()
                        .ptr_type(AddressSpace::Generic)
                        .into(),
                    ctx.llvm_context.i32_type().into(),
                ],
                false,
            ),
        );

        ctx.builder.build_call(
            bpf_printk,
            &[
                alloca.into(),
                ctx.llvm_context
                    .i32_type()
                    .const_int(16 as u64, false)
                    .into(),
            ],
            "print",
        );
         */
        //-------------

        ExprValue {
            value: alloca.as_any_value_enum(),
            ty: ExprType::String,
        }
    }

    pub fn codegen_pid<'a>(&self, ctx: &mut CodegenCtx<'a>) -> ExprValue<'a> {
        // u32 pid;
        // pid = bpf_get_current_pid_tgid() >> 32;
        // bpf_printk(ctx, b"get_current_pid\0");

        let bpf_get_current_pid_tgid =
            gen_bpf_helper(ctx, 14, ctx.llvm_context.i64_type().fn_type(&[], false));

        let call_ret_val = ctx
            .builder
            .build_call(bpf_get_current_pid_tgid, &[], "pid_tgid");

        let value = ctx.builder.build_right_shift(
            call_ret_val.as_any_value_enum().into_int_value(),
            ctx.llvm_context.i64_type().const_int(32, false),
            false,
            "pid",
        );

        ExprValue {
            value: value.as_any_value_enum(),
            ty: ExprType::Number,
        }
    }
}

impl OutputStruct for UProbeResult {
    fn codegen_lookup<'a>(
        &self,
        prop_name: &str,
        ctx: &mut CodegenCtx<'a>,
    ) -> Result<ExprValue<'a>, CodegenError> {
        Ok(match prop_name {
            "process_name" => self.codegen_proc_name(ctx),
            "pid" => self.codegen_pid(ctx),
            prop => return Err(CodegenError::Other(format!("unknown property {}", prop))),
        })
    }
}
