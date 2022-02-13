// Generated from graph_planar.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::graph_planarlistener::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::lazy_static;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const RIGHT_ARROW: isize = 1;
pub const NONE_ARROW: isize = 2;
pub const LEFT_RIGHT_ARROW: isize = 3;
pub const LEFT_ARROW: isize = 4;
pub const STRING: isize = 5;
pub const ID: isize = 6;
pub const EOL: isize = 7;
pub const WS: isize = 8;
pub const COMMENT: isize = 9;
pub const LINE_COMMENT: isize = 10;
pub const RULE_graph: usize = 0;
pub const RULE_edges: usize = 1;
pub const RULE_arrow: usize = 2;
pub const RULE_node: usize = 3;
pub const ruleNames: [&'static str; 4] = ["graph", "edges", "arrow", "node"];

pub const _LITERAL_NAMES: [Option<&'static str>; 5] = [
    None,
    Some("'->'"),
    Some("'--'"),
    Some("'<->'"),
    Some("'<-'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 11] = [
    None,
    Some("RIGHT_ARROW"),
    Some("NONE_ARROW"),
    Some("LEFT_RIGHT_ARROW"),
    Some("LEFT_ARROW"),
    Some("STRING"),
    Some("ID"),
    Some("EOL"),
    Some("WS"),
    Some("COMMENT"),
    Some("LINE_COMMENT"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    graph_planarParserExt,
    I,
    graph_planarParserContextType,
    dyn graph_planarListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type graph_planarTreeWalker<'input, 'a> = ParseTreeWalker<
    'input,
    'a,
    graph_planarParserContextType,
    dyn graph_planarListener<'input> + 'a,
>;

/// Parser for graph_planar grammar
pub struct graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                graph_planarParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> graph_planarParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I>
    graph_planarParser<'input, I, DefaultErrorStrategy<'input, graph_planarParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for graph_planarParser
pub trait graph_planarParserContext<'input>:
    for<'x> Listenable<dyn graph_planarListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = graph_planarParserContextType>
{
}

impl<'input> graph_planarParserContext<'input>
    for TerminalNode<'input, graph_planarParserContextType>
{
}
impl<'input> graph_planarParserContext<'input>
    for ErrorNode<'input, graph_planarParserContextType>
{
}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn graph_planarParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn graph_planarListener<'input> + 'input {}

pub struct graph_planarParserContextType;
antlr_rust::type_id! {graph_planarParserContextType}

impl<'input> ParserNodeType<'input> for graph_planarParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn graph_planarParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct graph_planarParserExt {}

impl graph_planarParserExt {}

impl<'input> TokenAware<'input> for graph_planarParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for graph_planarParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for graph_planarParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "graph_planar.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- graph ----------------
pub type GraphContextAll<'input> = GraphContext<'input>;

pub type GraphContext<'input> = BaseParserRuleContext<'input, GraphContextExt<'input>>;

#[derive(Clone)]
pub struct GraphContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> graph_planarParserContext<'input> for GraphContext<'input> {}

impl<'input, 'a> Listenable<dyn graph_planarListener<'input> + 'a> for GraphContext<'input> {
    fn enter(&self, listener: &mut (dyn graph_planarListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_graph(self);
    }
}

impl<'input> CustomRuleContext<'input> for GraphContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = graph_planarParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_graph
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_graph }
}
antlr_rust::type_id! {GraphContextExt<'a>}

impl<'input> GraphContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn graph_planarParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<GraphContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            GraphContextExt { ph: PhantomData },
        ))
    }
}

pub trait GraphContextAttrs<'input>:
    graph_planarParserContext<'input> + BorrowMut<GraphContextExt<'input>>
{
    fn edges_all(&self) -> Vec<Rc<EdgesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn edges(&self, i: usize) -> Option<Rc<EdgesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token EOL in current rule
    fn EOL_all(&self) -> Vec<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token EOL, starting from 0.
    /// Returns `None` if number of children corresponding to token EOL is less or equal than `i`.
    fn EOL(&self, i: usize) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOL, i)
    }
}

impl<'input> GraphContextAttrs<'input> for GraphContext<'input> {}

impl<'input, I, H> graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn graph(&mut self) -> Result<Rc<GraphContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = GraphContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_graph);
        let mut _localctx: Rc<GraphContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(11);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == EOL {
                    {
                        {
                            recog.base.set_state(8);
                            recog.base.match_token(EOL, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(13);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                /*InvokeRule edges*/
                recog.base.set_state(14);
                recog.edges()?;

                recog.base.set_state(23);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(2, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(16);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                loop {
                                    {
                                        {
                                            recog.base.set_state(15);
                                            recog.base.match_token(EOL, &mut recog.err_handler)?;
                                        }
                                    }
                                    recog.base.set_state(18);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if !(_la == EOL) {
                                        break;
                                    }
                                }
                                /*InvokeRule edges*/
                                recog.base.set_state(20);
                                recog.edges()?;
                            }
                        }
                    }
                    recog.base.set_state(25);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(2, &mut recog.base)?;
                }
                recog.base.set_state(29);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == EOL {
                    {
                        {
                            recog.base.set_state(26);
                            recog.base.match_token(EOL, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(31);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(32);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- edges ----------------
pub type EdgesContextAll<'input> = EdgesContext<'input>;

pub type EdgesContext<'input> = BaseParserRuleContext<'input, EdgesContextExt<'input>>;

#[derive(Clone)]
pub struct EdgesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> graph_planarParserContext<'input> for EdgesContext<'input> {}

impl<'input, 'a> Listenable<dyn graph_planarListener<'input> + 'a> for EdgesContext<'input> {
    fn enter(&self, listener: &mut (dyn graph_planarListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_edges(self);
    }
}

impl<'input> CustomRuleContext<'input> for EdgesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = graph_planarParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_edges
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_edges }
}
antlr_rust::type_id! {EdgesContextExt<'a>}

impl<'input> EdgesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn graph_planarParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<EdgesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            EdgesContextExt { ph: PhantomData },
        ))
    }
}

pub trait EdgesContextAttrs<'input>:
    graph_planarParserContext<'input> + BorrowMut<EdgesContextExt<'input>>
{
    fn node_all(&self) -> Vec<Rc<NodeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn node(&self, i: usize) -> Option<Rc<NodeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn arrow_all(&self) -> Vec<Rc<ArrowContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn arrow(&self, i: usize) -> Option<Rc<ArrowContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> EdgesContextAttrs<'input> for EdgesContext<'input> {}

impl<'input, I, H> graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn edges(&mut self) -> Result<Rc<EdgesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = EdgesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_edges);
        let mut _localctx: Rc<EdgesContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule node*/
                recog.base.set_state(34);
                recog.node()?;

                recog.base.set_state(38);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule arrow*/
                            recog.base.set_state(35);
                            recog.arrow()?;

                            /*InvokeRule node*/
                            recog.base.set_state(36);
                            recog.node()?;
                        }
                    }
                    recog.base.set_state(40);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << RIGHT_ARROW)
                                | (1usize << NONE_ARROW)
                                | (1usize << LEFT_RIGHT_ARROW)
                                | (1usize << LEFT_ARROW)))
                            != 0)
                    {
                        break;
                    }
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- arrow ----------------
pub type ArrowContextAll<'input> = ArrowContext<'input>;

pub type ArrowContext<'input> = BaseParserRuleContext<'input, ArrowContextExt<'input>>;

#[derive(Clone)]
pub struct ArrowContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> graph_planarParserContext<'input> for ArrowContext<'input> {}

impl<'input, 'a> Listenable<dyn graph_planarListener<'input> + 'a> for ArrowContext<'input> {
    fn enter(&self, listener: &mut (dyn graph_planarListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_arrow(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArrowContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = graph_planarParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arrow
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arrow }
}
antlr_rust::type_id! {ArrowContextExt<'a>}

impl<'input> ArrowContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn graph_planarParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArrowContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArrowContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArrowContextAttrs<'input>:
    graph_planarParserContext<'input> + BorrowMut<ArrowContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RIGHT_ARROW
    /// Returns `None` if there is no child corresponding to token RIGHT_ARROW
    fn RIGHT_ARROW(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RIGHT_ARROW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NONE_ARROW
    /// Returns `None` if there is no child corresponding to token NONE_ARROW
    fn NONE_ARROW(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONE_ARROW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEFT_RIGHT_ARROW
    /// Returns `None` if there is no child corresponding to token LEFT_RIGHT_ARROW
    fn LEFT_RIGHT_ARROW(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LEFT_RIGHT_ARROW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEFT_ARROW
    /// Returns `None` if there is no child corresponding to token LEFT_ARROW
    fn LEFT_ARROW(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LEFT_ARROW, 0)
    }
}

impl<'input> ArrowContextAttrs<'input> for ArrowContext<'input> {}

impl<'input, I, H> graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn arrow(&mut self) -> Result<Rc<ArrowContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArrowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_arrow);
        let mut _localctx: Rc<ArrowContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(42);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << RIGHT_ARROW)
                                | (1usize << NONE_ARROW)
                                | (1usize << LEFT_RIGHT_ARROW)
                                | (1usize << LEFT_ARROW)))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- node ----------------
pub type NodeContextAll<'input> = NodeContext<'input>;

pub type NodeContext<'input> = BaseParserRuleContext<'input, NodeContextExt<'input>>;

#[derive(Clone)]
pub struct NodeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> graph_planarParserContext<'input> for NodeContext<'input> {}

impl<'input, 'a> Listenable<dyn graph_planarListener<'input> + 'a> for NodeContext<'input> {
    fn enter(&self, listener: &mut (dyn graph_planarListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_node(self);
    }
}

impl<'input> CustomRuleContext<'input> for NodeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = graph_planarParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_node
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_node }
}
antlr_rust::type_id! {NodeContextExt<'a>}

impl<'input> NodeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn graph_planarParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NodeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NodeContextExt { ph: PhantomData },
        ))
    }
}

pub trait NodeContextAttrs<'input>:
    graph_planarParserContext<'input> + BorrowMut<NodeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, graph_planarParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> NodeContextAttrs<'input> for NodeContext<'input> {}

impl<'input, I, H> graph_planarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn node(&mut self) -> Result<Rc<NodeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NodeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_node);
        let mut _localctx: Rc<NodeContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(44);
                _la = recog.base.input.la(1);
                if { !(_la == STRING || _la == ID) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x0c\x31\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x03\x02\x07\x02\x0c\x0a\x02\x0c\x02\x0e\x02\x0f\x0b\x02\x03\x02\x03\x02\
	\x06\x02\x13\x0a\x02\x0d\x02\x0e\x02\x14\x03\x02\x07\x02\x18\x0a\x02\x0c\
	\x02\x0e\x02\x1b\x0b\x02\x03\x02\x07\x02\x1e\x0a\x02\x0c\x02\x0e\x02\x21\
	\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x06\x03\x29\x0a\
	\x03\x0d\x03\x0e\x03\x2a\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x02\x02\
	\x06\x02\x04\x06\x08\x02\x04\x03\x02\x03\x06\x03\x02\x07\x08\x02\x31\x02\
	\x0d\x03\x02\x02\x02\x04\x24\x03\x02\x02\x02\x06\x2c\x03\x02\x02\x02\x08\
	\x2e\x03\x02\x02\x02\x0a\x0c\x07\x09\x02\x02\x0b\x0a\x03\x02\x02\x02\x0c\
	\x0f\x03\x02\x02\x02\x0d\x0b\x03\x02\x02\x02\x0d\x0e\x03\x02\x02\x02\x0e\
	\x10\x03\x02\x02\x02\x0f\x0d\x03\x02\x02\x02\x10\x19\x05\x04\x03\x02\x11\
	\x13\x07\x09\x02\x02\x12\x11\x03\x02\x02\x02\x13\x14\x03\x02\x02\x02\x14\
	\x12\x03\x02\x02\x02\x14\x15\x03\x02\x02\x02\x15\x16\x03\x02\x02\x02\x16\
	\x18\x05\x04\x03\x02\x17\x12\x03\x02\x02\x02\x18\x1b\x03\x02\x02\x02\x19\
	\x17\x03\x02\x02\x02\x19\x1a\x03\x02\x02\x02\x1a\x1f\x03\x02\x02\x02\x1b\
	\x19\x03\x02\x02\x02\x1c\x1e\x07\x09\x02\x02\x1d\x1c\x03\x02\x02\x02\x1e\
	\x21\x03\x02\x02\x02\x1f\x1d\x03\x02\x02\x02\x1f\x20\x03\x02\x02\x02\x20\
	\x22\x03\x02\x02\x02\x21\x1f\x03\x02\x02\x02\x22\x23\x07\x02\x02\x03\x23\
	\x03\x03\x02\x02\x02\x24\x28\x05\x08\x05\x02\x25\x26\x05\x06\x04\x02\x26\
	\x27\x05\x08\x05\x02\x27\x29\x03\x02\x02\x02\x28\x25\x03\x02\x02\x02\x29\
	\x2a\x03\x02\x02\x02\x2a\x28\x03\x02\x02\x02\x2a\x2b\x03\x02\x02\x02\x2b\
	\x05\x03\x02\x02\x02\x2c\x2d\x09\x02\x02\x02\x2d\x07\x03\x02\x02\x02\x2e\
	\x2f\x09\x03\x02\x02\x2f\x09\x03\x02\x02\x02\x07\x0d\x14\x19\x1f\x2a";
