//! Codegen utils.

use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetTriple},
    types::FunctionType,
    values::CallableValue,
    values::{AnyValue, AnyValueEnum},
    AddressSpace, OptimizationLevel,
};
use petgraph::{algo::toposort, Direction};

use crate::{formulas::FormulaError, nodes::CodegenCtx, ProgGraph};

#[derive(Debug)]
pub enum CodegenError {
    // Expected property has not been found.
    ExpectedProperty(&'static str),
    // Formula parse error
    FormulaError(FormulaError),
    // Error with a string description
    // FIXME
    Other(String),
}

impl From<FormulaError> for CodegenError {
    fn from(e: FormulaError) -> Self {
        CodegenError::FormulaError(e)
    }
}

pub struct CodegenResult {
    pub bpf: Vec<u8>,
    pub asm: String,
}

/// Generates BPF code out of a Metalens program graph.
pub fn generate(prog: &ProgGraph) -> Result<CodegenResult, CodegenError> {
    let mut exec_order = toposort(&prog, None)
        .expect("could not sort, cycles in the graph")
        .into_iter();

    let llvm_context = Context::create();

    let module = llvm_context.create_module("bpfprog");
    module.set_triple(&TargetTriple::create("bpf"));

    // TODO: introduce a "probe" abstraction instead of functions.
    // this should create a new basic block etc.

    let func = module.add_function("bpf", llvm_context.i32_type().fn_type(&[], false), None);
    let allocs_block = llvm_context.append_basic_block(func, "allocs");
    let entry_block = llvm_context.append_basic_block(func, "entry");
    let func_exit = llvm_context.append_basic_block(func, "exit");

    let builder = llvm_context.create_builder();

    builder.position_at_end(entry_block);

    let mut context = CodegenCtx {
        llvm_context: &llvm_context,
        module,
        builder,
        func,
        func_exit,
        allocs_block,
        current_block: entry_block,
    };

    while let Some(next_node_idx) = exec_order.next() {
        let next_node = &prog[next_node_idx];

        let inputs = prog
            .neighbors_directed(next_node_idx, Direction::Incoming)
            .map(|node_idx| &prog[node_idx])
            .collect::<Vec<_>>();

        // invoke codegen - generate a BPF prog
        next_node.codegen(&mut context, &inputs)?;
    }

    context
        .builder
        .build_unconditional_branch(context.func_exit);

    context.builder.position_at_end(allocs_block);
    context.builder.build_unconditional_branch(entry_block);

    context.builder.position_at_end(context.func_exit);
    context
        .builder
        .build_return(Some(&context.llvm_context.i32_type().const_zero()));

    // Create a BPF target and compile the module we have.
    Target::initialize_bpf(&InitializationConfig::default());

    let target = Target::from_name("bpf").unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetTriple::create("bpf"),
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    context.module.set_triple(&target_machine.get_triple());
    context
        .module
        .set_data_layout(&target_machine.get_target_data().get_data_layout());

    // add metainformation for the module
    context.func.set_section("uprobe/mlens");

    let char_ty = context.llvm_context.i8_type();
    let chars = b"GPL\x00"
        .iter()
        .map(|c| char_ty.const_int(*c as u64, false));
    let license_literal = char_ty.const_array(chars.collect::<Vec<_>>().as_slice());

    let global_var = context.module.add_global(
        license_literal.get_type(),
        Some(AddressSpace::Global),
        "LICENSE",
    );
    global_var.set_initializer(&license_literal);
    global_var.set_alignment(1);
    global_var.set_section("license");

    // debug llvm ir
    context.module.print_to_stderr();

    let mem_buffer_obj = target_machine
        .write_to_memory_buffer(&context.module, FileType::Object)
        .expect("failed to write to a mem buffer");

    let mem_buffer_asm = target_machine
        .write_to_memory_buffer(&context.module, FileType::Assembly)
        .expect("failed to write result to a file");

    let asm = std::str::from_utf8(&mem_buffer_asm.as_slice())
        .unwrap()
        .to_string();

    let bpf = mem_buffer_obj.as_slice().to_vec();

    Ok(CodegenResult { asm, bpf })
}

fn debug_sections(mem_buffer_obj: MemoryBuffer) {
    let obj_file = mem_buffer_obj.create_object_file().unwrap();

    let section_names = obj_file
        .get_sections()
        .map(|s| s.get_name().map(|n| n.to_owned()))
        .collect::<Vec<_>>();

    dbg!(&section_names);
}

pub fn generate_string_literal<'a>(ctx: &mut CodegenCtx<'a>, str: &str) -> AnyValueEnum<'a> {
    let char_ty = ctx.llvm_context.i8_type();

    let chars = str
        .as_bytes()
        .iter()
        .map(|c| char_ty.const_int(*c as u64, false));

    let str_const_literal = char_ty.const_array(chars.collect::<Vec<_>>().as_slice());

    str_const_literal.as_any_value_enum()
}

/// Helper to generate a function descriptor.
pub fn gen_bpf_helper<'a>(
    ctx: &mut CodegenCtx<'a>,
    helper_num: u32,
    fn_type: FunctionType<'a>,
) -> CallableValue<'a> {
    let bpf_get_current_comm = ctx
        .llvm_context
        .i64_type()
        .const_int(helper_num as u64, false)
        .const_to_pointer(fn_type.ptr_type(AddressSpace::Generic));

    CallableValue::try_from(bpf_get_current_comm)
        .expect("could not create a callable value from function ptr")
}

/// Generates a call to bpf_printk.
pub fn bpf_printk<'a>(ctx: &mut CodegenCtx<'a>, strk: &[u8]) {
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

    let char_ty = ctx.llvm_context.i8_type();
    let chars = strk.iter().map(|c| char_ty.const_int(*c as u64, false));
    let formatstr = char_ty.const_array(chars.collect::<Vec<_>>().as_slice());

    let global_var =
        ctx.module
            .add_global(formatstr.get_type(), Some(AddressSpace::Global), "format");
    global_var.set_initializer(&formatstr);
    global_var.set_alignment(1);

    ctx.builder.build_call(
        bpf_printk,
        &[
            global_var.as_pointer_value().into(),
            ctx.llvm_context
                .i32_type()
                .const_int(strk.len() as u64, false)
                .into(),
        ],
        "print",
    );
}
