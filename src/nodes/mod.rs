mod filter;
mod label;
mod uprobe;

pub use filter::FilterNode;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    values::{AnyValueEnum, FunctionValue},
};
pub use label::LabelNode;
pub use uprobe::UProbe;

use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    codegen::CodegenError,
    formulas::{ExprType, FormulaExpr},
    runtime::{LoadedState, RuntimeError},
    ws::MsgChannelTx,
};

/// Contains all context necessary for the code generator.
pub struct CodegenCtx<'a> {
    pub llvm_context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
    pub func: FunctionValue<'a>,
    pub func_exit: BasicBlock<'a>,
    // FIXME: this should not be required - introduce MIR to know which allocations
    // will be needed beforehand.
    pub allocs_block: BasicBlock<'a>,
    // FIXME: see if there's an option to get the current block in the LLVM API?
    pub current_block: BasicBlock<'a>,
}

impl<'a> CodegenCtx<'a> {
    // FIXME: see if there's an option to get the current block in the LLVM API?
    pub fn set_current_block(&mut self, b: BasicBlock<'a>) {
        self.current_block = b;
        self.builder.position_at_end(b);
    }
}

/// Structured output.
pub trait OutputStruct {
    // Generate code for a property lookup call.
    fn codegen_lookup<'a>(
        &self,
        prop_name: &str,
        ctx: &mut CodegenCtx<'a>,
    ) -> Result<ExprValue<'a>, CodegenError>;
}

/// Node output type.
pub enum OutputType {
    Struct(Box<dyn OutputStruct>),
    Void,
}

impl Debug for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(..) => write!(f, "Struct"),
            Self::Void => write!(f, "Void"),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Label(LabelNode),
    UProbe(UProbe),
    Filter(FilterNode),
}

impl Node {
    pub fn codegen<'a>(
        &self,
        ctx: &mut CodegenCtx<'a>,
        inputs: &[&Node],
    ) -> Result<(), CodegenError> {
        match self {
            Node::Label(n) => n.codegen(ctx, inputs),
            Node::Filter(n) => n.codegen(ctx, inputs),
            Node::UProbe(n) => n.codegen(ctx, inputs),
        }
    }

    pub fn output_type(&self) -> Rc<OutputType> {
        match self {
            Node::Label(n) => n.output_type(),
            Node::Filter(n) => n.output_type(),
            Node::UProbe(n) => n.output_type(),
        }
    }

    pub fn load(
        &self,
        prog: &mut LoadedState,
        out_stream: MsgChannelTx,
    ) -> Result<(), RuntimeError> {
        match self {
            Node::UProbe(n) => n.load(prog, out_stream),
            Node::Label(n) => n.load(prog, out_stream),
            _ => Ok(()), // other node types don't do anything during loading
        }
    }
}

/// Value returned by a code generator.
#[derive(Clone, Debug, PartialEq)]
pub struct ExprValue<'a> {
    pub value: AnyValueEnum<'a>,
    pub ty: ExprType,
}

#[derive(Debug)]
pub struct NodeProperties {
    pub(self) props: HashMap<String, FormulaExpr>,
}

impl NodeProperties {
    pub fn new() -> Self {
        Self {
            props: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, formula: FormulaExpr) {
        self.props.insert(name.to_string(), formula);
    }

    pub fn get(&self, name: &str) -> Option<&FormulaExpr> {
        self.props.get(name)
    }
}
