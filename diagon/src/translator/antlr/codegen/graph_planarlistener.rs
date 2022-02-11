#![allow(nonstandard_style)]
// Generated from graph_planar.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::graph_planarparser::*;

pub trait graph_planarListener<'input> : ParseTreeListener<'input,graph_planarParserContextType>{

/**
 * Enter a parse tree produced by {@link graph_planarParser#graph}.
 * @param ctx the parse tree
 */
fn enter_graph(&mut self, _ctx: &GraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link graph_planarParser#graph}.
 * @param ctx the parse tree
 */
fn exit_graph(&mut self, _ctx: &GraphContext<'input>) { }

/**
 * Enter a parse tree produced by {@link graph_planarParser#edges}.
 * @param ctx the parse tree
 */
fn enter_edges(&mut self, _ctx: &EdgesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link graph_planarParser#edges}.
 * @param ctx the parse tree
 */
fn exit_edges(&mut self, _ctx: &EdgesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link graph_planarParser#arrow}.
 * @param ctx the parse tree
 */
fn enter_arrow(&mut self, _ctx: &ArrowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link graph_planarParser#arrow}.
 * @param ctx the parse tree
 */
fn exit_arrow(&mut self, _ctx: &ArrowContext<'input>) { }

/**
 * Enter a parse tree produced by {@link graph_planarParser#node}.
 * @param ctx the parse tree
 */
fn enter_node(&mut self, _ctx: &NodeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link graph_planarParser#node}.
 * @param ctx the parse tree
 */
fn exit_node(&mut self, _ctx: &NodeContext<'input>) { }

}
