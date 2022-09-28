//! Formula language parser.

// TODO: should be split into its own separate crate.

mod codegen;
mod parser;

pub use codegen::*;
pub use parser::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ExprType {
    String,
    Number,
    Boolean,
    // FIXME: ugly hack
    Counter,
}
