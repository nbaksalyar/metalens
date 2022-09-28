//! Generates BPF code for filtering data.

use std::rc::Rc;

use crate::{codegen::CodegenError, dsl::NodeId, formulas};

use super::{CodegenCtx, Node, NodeProperties, OutputType};

#[derive(Debug)]
pub struct FilterNode {
    id: NodeId,
    props: NodeProperties,
    output_type: Rc<OutputType>,
}

impl FilterNode {
    pub fn new(id: NodeId, props: NodeProperties) -> Self {
        Self {
            id,
            props,
            output_type: Rc::new(OutputType::Void),
        }
    }

    /// Generates LLVM IR from Metalens AST
    pub fn codegen(&self, ctx: &mut CodegenCtx, inputs: &[&Node]) -> Result<(), CodegenError> {
        // context required:
        // - previous node output type
        // - my own output type
        // - variables used in my properties

        assert_eq!(inputs.len(), 1); // TODO: support multiple inputs?

        let input_node = inputs[0];
        // input_node.output_type();

        let filter_formula = &self
            .props
            .props
            .get("value")
            .ok_or(CodegenError::ExpectedProperty("value"))?;

        let output_type = input_node.output_type();

        dbg!(&output_type);
        dbg!(filter_formula);

        formulas::generate_expr_code(ctx, filter_formula, Some(output_type.as_ref()));

        // self.output_type = output_type;

        // ctx.builder
        Ok(())
    }

    pub fn output_type(&self) -> Rc<OutputType> {
        // = same output type as my input
        self.output_type.clone()
    }
}
