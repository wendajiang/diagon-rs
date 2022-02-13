#![allow(nonstandard_style)]
// Generated from sequence.g4 by ANTLR 4.8
use super::sequenceparser::*;
use antlr_rust::tree::ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link sequenceParser}.
 */
pub trait sequenceVisitor<'input>: ParseTreeVisitor<'input, sequenceParserContextType> {
    /**
     * Visit a parse tree produced by {@link sequenceParser#program}.
     * @param ctx the parse tree
     */
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#command}.
     * @param ctx the parse tree
     */
    fn visit_command(&mut self, ctx: &CommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#messageCommand}.
     * @param ctx the parse tree
     */
    fn visit_messageCommand(&mut self, ctx: &MessageCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#dependencyCommand}.
     * @param ctx the parse tree
     */
    fn visit_dependencyCommand(&mut self, ctx: &DependencyCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#dependency}.
     * @param ctx the parse tree
     */
    fn visit_dependency(&mut self, ctx: &DependencyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#dependencyID}.
     * @param ctx the parse tree
     */
    fn visit_dependencyID(&mut self, ctx: &DependencyIDContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#dependencies}.
     * @param ctx the parse tree
     */
    fn visit_dependencies(&mut self, ctx: &DependenciesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#text}.
     * @param ctx the parse tree
     */
    fn visit_text(&mut self, ctx: &TextContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#textInternal}.
     * @param ctx the parse tree
     */
    fn visit_textInternal(&mut self, ctx: &TextInternalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#number}.
     * @param ctx the parse tree
     */
    fn visit_number(&mut self, ctx: &NumberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#comparison}.
     * @param ctx the parse tree
     */
    fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link sequenceParser#arrow}.
     * @param ctx the parse tree
     */
    fn visit_arrow(&mut self, ctx: &ArrowContext<'input>) {
        self.visit_children(ctx)
    }
}
