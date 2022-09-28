//! Code generator for the formula language.

use inkwell::{
    types::AnyTypeEnum,
    values::{AnyValue, ArrayValue, PointerValue},
};

use crate::{
    codegen::{generate_string_literal, CodegenError},
    formulas::{FormulaOp, FormulaTerm},
    nodes::{CodegenCtx, ExprValue, OutputType},
};

use super::{ExprType, FormulaExpr};

pub fn generate_expr_code<'a>(
    ctx: &mut CodegenCtx<'a>,
    expr: &FormulaExpr,
    prev_node_output: Option<&OutputType>,
) -> Result<ExprValue<'a>, CodegenError> {
    match expr {
        FormulaExpr::Term(term) => {
            match term {
                FormulaTerm::Property(ref object, ref prop_name) => {
                    assert_eq!(object, "input"); // we support only one type of properties for now

                    if let Some(OutputType::Struct(prev_output_struct)) = prev_node_output {
                        return Ok(prev_output_struct.codegen_lookup(prop_name, ctx)?);
                    } else {
                        panic!("unexpected input type: {:?}", prev_node_output);
                    }
                }
                FormulaTerm::String(str_literal) => {
                    // codegen string array literal
                    Ok(ExprValue {
                        value: generate_string_literal(ctx, str_literal),
                        ty: ExprType::String,
                    })
                }
                FormulaTerm::CountCall(_ident) => Ok(ExprValue {
                    value: ctx.llvm_context.i64_type().const_int(1, false).into(),
                    ty: ExprType::Counter,
                }),
                _ => todo!(),
            }
        }
        FormulaExpr::Binary {
            lhs,
            rhs,
            binary_op,
        } => {
            let lhs_expr = generate_expr_code(ctx, lhs, prev_node_output)?;
            let rhs_expr = generate_expr_code(ctx, rhs, prev_node_output)?;

            match (lhs_expr.ty, rhs_expr.ty) {
                (ExprType::String, ExprType::String) => {
                    // compare two strings
                    if *binary_op != FormulaOp::Eq && *binary_op != FormulaOp::NotEq {
                        // TODO: fix panic
                        panic!("invalid comparison operation for strings");
                    }
                    // determine if lhs/rhs is a literal and hasn't generated any value
                    match (&lhs_expr.value.get_type(), &rhs_expr.value.get_type()) {
                        (AnyTypeEnum::ArrayType(_), AnyTypeEnum::ArrayType(_)) => {
                            // comparing two literals
                            todo!();
                        }
                        (AnyTypeEnum::PointerType(_), AnyTypeEnum::ArrayType(_))
                        | (AnyTypeEnum::ArrayType(_), AnyTypeEnum::PointerType(_)) => {
                            // comparing prop and a literal
                            // swap lhs with rhs if we're comparing literal with an expr
                            let (expr_val, literal) = if lhs_expr.value.is_array_value() {
                                (rhs_expr, lhs_expr)
                            } else {
                                (lhs_expr, rhs_expr)
                            };

                            // generate a sequence of icmps for each character
                            // TODO: check for string length
                            Ok(generate_string_cmp(
                                ctx,
                                *binary_op,
                                expr_val.value.into_pointer_value(),
                                literal.value.into_array_value(),
                            ))
                        }
                        (_, _) => {
                            // comparing two exprs
                            // TODO
                            todo!();
                        }
                    }
                }
                (ExprType::Number, ExprType::Number) => {
                    // compare two nums
                    // ctx.builder.build_int_compare();
                    todo!();
                }
                (ExprType::Boolean, ExprType::Number) | (ExprType::Number, ExprType::Boolean) => {
                    // compare bool with a number
                    todo!();
                }
                (ExprType::Boolean, ExprType::Boolean) => {
                    // compare two bools
                    todo!();
                }
                (ExprType::Counter, _) | (_, ExprType::Counter) => {
                    panic!("invalid comparison with a counter");
                }
                (ExprType::String, _) | (_, ExprType::String) => {
                    // type mismatch - comparing string with numeric/boolean value
                    // TODO: fix panic
                    panic!("type mismatch");
                }
            }
        }
    }
}

fn generate_string_cmp<'a>(
    ctx: &mut CodegenCtx<'a>,
    binary_op: FormulaOp,
    lhs: PointerValue<'a>,
    rhs: ArrayValue<'a>,
) -> ExprValue<'a> {
    dbg!(&lhs, &rhs);

    let i64_ty = ctx.llvm_context.i64_type();

    for char_num in 0..rhs.get_type().len() {
        let next_block = ctx.llvm_context.append_basic_block(ctx.func, "cmp_br");

        let lhs_elem_ptr = unsafe {
            ctx.builder.build_in_bounds_gep(
                lhs,
                &[
                    i64_ty.const_int(0, false),
                    i64_ty.const_int(char_num as u64, false),
                ],
                "v",
            )
        };

        let lhs_elem = ctx.builder.build_load(lhs_elem_ptr, "i").into_int_value();

        // extract value from the literal rhs
        let rhs_elem = ctx
            .builder
            .build_extract_value(rhs, char_num, "literal")
            .expect("could not extract value")
            .into_int_value();

        let cmp_res = ctx
            .builder
            .build_int_compare(binary_op.into(), lhs_elem, rhs_elem, "cmp");

        ctx.builder
            .build_conditional_branch(cmp_res, next_block, ctx.func_exit);

        ctx.set_current_block(next_block);
    }

    ExprValue {
        value: i64_ty.const_int(1, false).as_any_value_enum(),
        ty: ExprType::Number,
    }
}
