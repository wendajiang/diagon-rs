#![allow(nonstandard_style)]
// Generated from math.g4 by ANTLR 4.8
use super::mathparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait mathListener<'input>: ParseTreeListener<'input, mathParserContextType> {
    /**
     * Enter a parse tree produced by {@link mathParser#multilineEquation}.
     * @param ctx the parse tree
     */
    fn enter_multilineEquation(&mut self, _ctx: &MultilineEquationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#multilineEquation}.
     * @param ctx the parse tree
     */
    fn exit_multilineEquation(&mut self, _ctx: &MultilineEquationContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#newlines}.
     * @param ctx the parse tree
     */
    fn enter_newlines(&mut self, _ctx: &NewlinesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#newlines}.
     * @param ctx the parse tree
     */
    fn exit_newlines(&mut self, _ctx: &NewlinesContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#equation}.
     * @param ctx the parse tree
     */
    fn enter_equation(&mut self, _ctx: &EquationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#equation}.
     * @param ctx the parse tree
     */
    fn exit_equation(&mut self, _ctx: &EquationContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term(&mut self, _ctx: &TermContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term(&mut self, _ctx: &TermContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#factor}.
     * @param ctx the parse tree
     */
    fn enter_factor(&mut self, _ctx: &FactorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#factor}.
     * @param ctx the parse tree
     */
    fn exit_factor(&mut self, _ctx: &FactorContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#valueBang}.
     * @param ctx the parse tree
     */
    fn enter_valueBang(&mut self, _ctx: &ValueBangContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#valueBang}.
     * @param ctx the parse tree
     */
    fn exit_valueBang(&mut self, _ctx: &ValueBangContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#value}.
     * @param ctx the parse tree
     */
    fn enter_value(&mut self, _ctx: &ValueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#value}.
     * @param ctx the parse tree
     */
    fn exit_value(&mut self, _ctx: &ValueContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#atom}.
     * @param ctx the parse tree
     */
    fn enter_atom(&mut self, _ctx: &AtomContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#atom}.
     * @param ctx the parse tree
     */
    fn exit_atom(&mut self, _ctx: &AtomContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#function}.
     * @param ctx the parse tree
     */
    fn enter_function(&mut self, _ctx: &FunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#function}.
     * @param ctx the parse tree
     */
    fn exit_function(&mut self, _ctx: &FunctionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#variable}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#variable}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#matrix}.
     * @param ctx the parse tree
     */
    fn enter_matrix(&mut self, _ctx: &MatrixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#matrix}.
     * @param ctx the parse tree
     */
    fn exit_matrix(&mut self, _ctx: &MatrixContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#matrixLine}.
     * @param ctx the parse tree
     */
    fn enter_matrixLine(&mut self, _ctx: &MatrixLineContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#matrixLine}.
     * @param ctx the parse tree
     */
    fn exit_matrixLine(&mut self, _ctx: &MatrixLineContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#relop}.
     * @param ctx the parse tree
     */
    fn enter_relop(&mut self, _ctx: &RelopContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#relop}.
     * @param ctx the parse tree
     */
    fn exit_relop(&mut self, _ctx: &RelopContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#addop}.
     * @param ctx the parse tree
     */
    fn enter_addop(&mut self, _ctx: &AddopContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#addop}.
     * @param ctx the parse tree
     */
    fn exit_addop(&mut self, _ctx: &AddopContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#mulop}.
     * @param ctx the parse tree
     */
    fn enter_mulop(&mut self, _ctx: &MulopContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#mulop}.
     * @param ctx the parse tree
     */
    fn exit_mulop(&mut self, _ctx: &MulopContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link mathParser#powop}.
     * @param ctx the parse tree
     */
    fn enter_powop(&mut self, _ctx: &PowopContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link mathParser#powop}.
     * @param ctx the parse tree
     */
    fn exit_powop(&mut self, _ctx: &PowopContext<'input>) {}
}
