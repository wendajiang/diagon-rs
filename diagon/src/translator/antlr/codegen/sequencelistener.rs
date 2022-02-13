#![allow(nonstandard_style)]
// Generated from sequence.g4 by ANTLR 4.8
use super::sequenceparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait sequenceListener<'input>: ParseTreeListener<'input, sequenceParserContextType> {
    /**
     * Enter a parse tree produced by {@link sequenceParser#program}.
     * @param ctx the parse tree
     */
    fn enter_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#program}.
     * @param ctx the parse tree
     */
    fn exit_program(&mut self, _ctx: &ProgramContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#command}.
     * @param ctx the parse tree
     */
    fn enter_command(&mut self, _ctx: &CommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#command}.
     * @param ctx the parse tree
     */
    fn exit_command(&mut self, _ctx: &CommandContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#messageCommand}.
     * @param ctx the parse tree
     */
    fn enter_messageCommand(&mut self, _ctx: &MessageCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#messageCommand}.
     * @param ctx the parse tree
     */
    fn exit_messageCommand(&mut self, _ctx: &MessageCommandContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#dependencyCommand}.
     * @param ctx the parse tree
     */
    fn enter_dependencyCommand(&mut self, _ctx: &DependencyCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#dependencyCommand}.
     * @param ctx the parse tree
     */
    fn exit_dependencyCommand(&mut self, _ctx: &DependencyCommandContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#dependency}.
     * @param ctx the parse tree
     */
    fn enter_dependency(&mut self, _ctx: &DependencyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#dependency}.
     * @param ctx the parse tree
     */
    fn exit_dependency(&mut self, _ctx: &DependencyContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#dependencyID}.
     * @param ctx the parse tree
     */
    fn enter_dependencyID(&mut self, _ctx: &DependencyIDContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#dependencyID}.
     * @param ctx the parse tree
     */
    fn exit_dependencyID(&mut self, _ctx: &DependencyIDContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#dependencies}.
     * @param ctx the parse tree
     */
    fn enter_dependencies(&mut self, _ctx: &DependenciesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#dependencies}.
     * @param ctx the parse tree
     */
    fn exit_dependencies(&mut self, _ctx: &DependenciesContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#text}.
     * @param ctx the parse tree
     */
    fn enter_text(&mut self, _ctx: &TextContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#text}.
     * @param ctx the parse tree
     */
    fn exit_text(&mut self, _ctx: &TextContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#textInternal}.
     * @param ctx the parse tree
     */
    fn enter_textInternal(&mut self, _ctx: &TextInternalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#textInternal}.
     * @param ctx the parse tree
     */
    fn exit_textInternal(&mut self, _ctx: &TextInternalContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#number}.
     * @param ctx the parse tree
     */
    fn enter_number(&mut self, _ctx: &NumberContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#number}.
     * @param ctx the parse tree
     */
    fn exit_number(&mut self, _ctx: &NumberContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#comparison}.
     * @param ctx the parse tree
     */
    fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#comparison}.
     * @param ctx the parse tree
     */
    fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link sequenceParser#arrow}.
     * @param ctx the parse tree
     */
    fn enter_arrow(&mut self, _ctx: &ArrowContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link sequenceParser#arrow}.
     * @param ctx the parse tree
     */
    fn exit_arrow(&mut self, _ctx: &ArrowContext<'input>) {}
}
