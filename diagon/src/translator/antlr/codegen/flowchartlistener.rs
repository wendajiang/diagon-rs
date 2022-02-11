#![allow(nonstandard_style)]
// Generated from flowchart.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::flowchartparser::*;

pub trait flowchartListener<'input> : ParseTreeListener<'input,flowchartParserContextType>{

/**
 * Enter a parse tree produced by {@link flowchartParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#instruction}.
 * @param ctx the parse tree
 */
fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#instruction}.
 * @param ctx the parse tree
 */
fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#noop}.
 * @param ctx the parse tree
 */
fn enter_noop(&mut self, _ctx: &NoopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#noop}.
 * @param ctx the parse tree
 */
fn exit_noop(&mut self, _ctx: &NoopContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#element}.
 * @param ctx the parse tree
 */
fn enter_element(&mut self, _ctx: &ElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#element}.
 * @param ctx the parse tree
 */
fn exit_element(&mut self, _ctx: &ElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#string}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#string}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#condition}.
 * @param ctx the parse tree
 */
fn enter_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#condition}.
 * @param ctx the parse tree
 */
fn exit_condition(&mut self, _ctx: &ConditionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#whileloop}.
 * @param ctx the parse tree
 */
fn enter_whileloop(&mut self, _ctx: &WhileloopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#whileloop}.
 * @param ctx the parse tree
 */
fn exit_whileloop(&mut self, _ctx: &WhileloopContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#doloop}.
 * @param ctx the parse tree
 */
fn enter_doloop(&mut self, _ctx: &DoloopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#doloop}.
 * @param ctx the parse tree
 */
fn exit_doloop(&mut self, _ctx: &DoloopContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#group}.
 * @param ctx the parse tree
 */
fn enter_group(&mut self, _ctx: &GroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#group}.
 * @param ctx the parse tree
 */
fn exit_group(&mut self, _ctx: &GroupContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#returninstruction}.
 * @param ctx the parse tree
 */
fn enter_returninstruction(&mut self, _ctx: &ReturninstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#returninstruction}.
 * @param ctx the parse tree
 */
fn exit_returninstruction(&mut self, _ctx: &ReturninstructionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link flowchartParser#switchinstruction}.
 * @param ctx the parse tree
 */
fn enter_switchinstruction(&mut self, _ctx: &SwitchinstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link flowchartParser#switchinstruction}.
 * @param ctx the parse tree
 */
fn exit_switchinstruction(&mut self, _ctx: &SwitchinstructionContext<'input>) { }

}
