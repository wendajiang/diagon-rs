// Generated from flowchart.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::flowchartlistener::*;
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

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const T__7: isize = 8;
pub const T__8: isize = 9;
pub const T__9: isize = 10;
pub const T__10: isize = 11;
pub const T__11: isize = 12;
pub const T__12: isize = 13;
pub const T__13: isize = 14;
pub const WS: isize = 15;
pub const COMMENT: isize = 16;
pub const LINE_COMMENT: isize = 17;
pub const STRING_SIMPLE_QUOTE: isize = 18;
pub const STRING_DOUBLE_QUOTE: isize = 19;
pub const RULE_program: usize = 0;
pub const RULE_instruction: usize = 1;
pub const RULE_noop: usize = 2;
pub const RULE_element: usize = 3;
pub const RULE_string: usize = 4;
pub const RULE_condition: usize = 5;
pub const RULE_whileloop: usize = 6;
pub const RULE_doloop: usize = 7;
pub const RULE_group: usize = 8;
pub const RULE_returninstruction: usize = 9;
pub const RULE_switchinstruction: usize = 10;
pub const ruleNames: [&'static str; 11] = [
    "program",
    "instruction",
    "noop",
    "element",
    "string",
    "condition",
    "whileloop",
    "doloop",
    "group",
    "returninstruction",
    "switchinstruction",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 15] = [
    None,
    Some("'noop'"),
    Some("';'"),
    Some("'if'"),
    Some("'('"),
    Some("')'"),
    Some("'else'"),
    Some("'while'"),
    Some("'do'"),
    Some("'{'"),
    Some("'}'"),
    Some("'return'"),
    Some("'switch'"),
    Some("'case'"),
    Some("':'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 20] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("WS"),
    Some("COMMENT"),
    Some("LINE_COMMENT"),
    Some("STRING_SIMPLE_QUOTE"),
    Some("STRING_DOUBLE_QUOTE"),
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
    flowchartParserExt,
    I,
    flowchartParserContextType,
    dyn flowchartListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type flowchartTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, flowchartParserContextType, dyn flowchartListener<'input> + 'a>;

/// Parser for flowchart grammar
pub struct flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> flowchartParser<'input, I, H>
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
                flowchartParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> flowchartParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> flowchartParser<'input, I, DefaultErrorStrategy<'input, flowchartParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for flowchartParser
pub trait flowchartParserContext<'input>:
    for<'x> Listenable<dyn flowchartListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = flowchartParserContextType>
{
}

impl<'input> flowchartParserContext<'input> for TerminalNode<'input, flowchartParserContextType> {}
impl<'input> flowchartParserContext<'input> for ErrorNode<'input, flowchartParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn flowchartParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn flowchartListener<'input> + 'input {}

pub struct flowchartParserContextType;
antlr_rust::type_id! {flowchartParserContextType}

impl<'input> ParserNodeType<'input> for flowchartParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn flowchartParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct flowchartParserExt {}

impl flowchartParserExt {}

impl<'input> TokenAware<'input> for flowchartParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for flowchartParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for flowchartParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "flowchart.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;

pub type ProgramContext<'input> = BaseParserRuleContext<'input, ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for ProgramContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for ProgramContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_program(self);
    }
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_program
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::type_id! {ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ProgramContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ProgramContextExt { ph: PhantomData },
        ))
    }
}

pub trait ProgramContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<ProgramContextExt<'input>>
{
    fn instruction_all(&self) -> Vec<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn program(&mut self) -> Result<Rc<ProgramContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(25);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__0)
                            | (1usize << T__2)
                            | (1usize << T__6)
                            | (1usize << T__7)
                            | (1usize << T__8)
                            | (1usize << T__10)
                            | (1usize << T__11)
                            | (1usize << STRING_SIMPLE_QUOTE)
                            | (1usize << STRING_DOUBLE_QUOTE)))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule instruction*/
                            recog.base.set_state(22);
                            recog.instruction()?;
                        }
                    }
                    recog.base.set_state(27);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
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
//------------------- instruction ----------------
pub type InstructionContextAll<'input> = InstructionContext<'input>;

pub type InstructionContext<'input> = BaseParserRuleContext<'input, InstructionContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for InstructionContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for InstructionContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_instruction(self);
    }
}

impl<'input> CustomRuleContext<'input> for InstructionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_instruction
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_instruction }
}
antlr_rust::type_id! {InstructionContextExt<'a>}

impl<'input> InstructionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<InstructionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            InstructionContextExt { ph: PhantomData },
        ))
    }
}

pub trait InstructionContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<InstructionContextExt<'input>>
{
    fn noop(&self) -> Option<Rc<NoopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn element(&self) -> Option<Rc<ElementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn condition(&self) -> Option<Rc<ConditionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn group(&self) -> Option<Rc<GroupContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn whileloop(&self) -> Option<Rc<WhileloopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn doloop(&self) -> Option<Rc<DoloopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn returninstruction(&self) -> Option<Rc<ReturninstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn switchinstruction(&self) -> Option<Rc<SwitchinstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> InstructionContextAttrs<'input> for InstructionContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn instruction(&mut self) -> Result<Rc<InstructionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = InstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 2, RULE_instruction);
        let mut _localctx: Rc<InstructionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(36);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__0 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule noop*/
                        recog.base.set_state(28);
                        recog.noop()?;
                    }
                }

                STRING_SIMPLE_QUOTE | STRING_DOUBLE_QUOTE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule element*/
                        recog.base.set_state(29);
                        recog.element()?;
                    }
                }

                T__2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule condition*/
                        recog.base.set_state(30);
                        recog.condition()?;
                    }
                }

                T__8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule group*/
                        recog.base.set_state(31);
                        recog.group()?;
                    }
                }

                T__6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule whileloop*/
                        recog.base.set_state(32);
                        recog.whileloop()?;
                    }
                }

                T__7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule doloop*/
                        recog.base.set_state(33);
                        recog.doloop()?;
                    }
                }

                T__10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule returninstruction*/
                        recog.base.set_state(34);
                        recog.returninstruction()?;
                    }
                }

                T__11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule switchinstruction*/
                        recog.base.set_state(35);
                        recog.switchinstruction()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
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
//------------------- noop ----------------
pub type NoopContextAll<'input> = NoopContext<'input>;

pub type NoopContext<'input> = BaseParserRuleContext<'input, NoopContextExt<'input>>;

#[derive(Clone)]
pub struct NoopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for NoopContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for NoopContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_noop(self);
    }
}

impl<'input> CustomRuleContext<'input> for NoopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noop }
}
antlr_rust::type_id! {NoopContextExt<'a>}

impl<'input> NoopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NoopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NoopContextExt { ph: PhantomData },
        ))
    }
}

pub trait NoopContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<NoopContextExt<'input>>
{
}

impl<'input> NoopContextAttrs<'input> for NoopContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn noop(&mut self) -> Result<Rc<NoopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NoopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_noop);
        let mut _localctx: Rc<NoopContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(38);
                recog.base.match_token(T__0, &mut recog.err_handler)?;

                recog.base.set_state(40);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__1 {
                    {
                        recog.base.set_state(39);
                        recog.base.match_token(T__1, &mut recog.err_handler)?;
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
//------------------- element ----------------
pub type ElementContextAll<'input> = ElementContext<'input>;

pub type ElementContext<'input> = BaseParserRuleContext<'input, ElementContextExt<'input>>;

#[derive(Clone)]
pub struct ElementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for ElementContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for ElementContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_element(self);
    }
}

impl<'input> CustomRuleContext<'input> for ElementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_element
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_element }
}
antlr_rust::type_id! {ElementContextExt<'a>}

impl<'input> ElementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ElementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ElementContextExt { ph: PhantomData },
        ))
    }
}

pub trait ElementContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<ElementContextExt<'input>>
{
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ElementContextAttrs<'input> for ElementContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn element(&mut self) -> Result<Rc<ElementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_element);
        let mut _localctx: Rc<ElementContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule string*/
                recog.base.set_state(42);
                recog.string()?;

                recog.base.set_state(44);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__1 {
                    {
                        recog.base.set_state(43);
                        recog.base.match_token(T__1, &mut recog.err_handler)?;
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
//------------------- string ----------------
pub type StringContextAll<'input> = StringContext<'input>;

pub type StringContext<'input> = BaseParserRuleContext<'input, StringContextExt<'input>>;

#[derive(Clone)]
pub struct StringContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for StringContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for StringContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_string(self);
    }
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_string
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_string }
}
antlr_rust::type_id! {StringContextExt<'a>}

impl<'input> StringContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StringContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StringContextExt { ph: PhantomData },
        ))
    }
}

pub trait StringContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<StringContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING_SIMPLE_QUOTE
    /// Returns `None` if there is no child corresponding to token STRING_SIMPLE_QUOTE
    fn STRING_SIMPLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input, flowchartParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_SIMPLE_QUOTE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_DOUBLE_QUOTE
    /// Returns `None` if there is no child corresponding to token STRING_DOUBLE_QUOTE
    fn STRING_DOUBLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input, flowchartParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_DOUBLE_QUOTE, 0)
    }
}

impl<'input> StringContextAttrs<'input> for StringContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn string(&mut self) -> Result<Rc<StringContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(46);
                _la = recog.base.input.la(1);
                if { !(_la == STRING_SIMPLE_QUOTE || _la == STRING_DOUBLE_QUOTE) } {
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
//------------------- condition ----------------
pub type ConditionContextAll<'input> = ConditionContext<'input>;

pub type ConditionContext<'input> = BaseParserRuleContext<'input, ConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for ConditionContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for ConditionContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_condition(self);
    }
}

impl<'input> CustomRuleContext<'input> for ConditionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_condition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_condition }
}
antlr_rust::type_id! {ConditionContextExt<'a>}

impl<'input> ConditionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ConditionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ConditionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ConditionContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<ConditionContextExt<'input>>
{
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn instruction_all(&self) -> Vec<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ConditionContextAttrs<'input> for ConditionContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn condition(&mut self) -> Result<Rc<ConditionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_condition);
        let mut _localctx: Rc<ConditionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(48);
                recog.base.match_token(T__2, &mut recog.err_handler)?;

                recog.base.set_state(49);
                recog.base.match_token(T__3, &mut recog.err_handler)?;

                /*InvokeRule string*/
                recog.base.set_state(50);
                recog.string()?;

                recog.base.set_state(51);
                recog.base.match_token(T__4, &mut recog.err_handler)?;

                /*InvokeRule instruction*/
                recog.base.set_state(52);
                recog.instruction()?;

                recog.base.set_state(55);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(4, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(53);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            /*InvokeRule instruction*/
                            recog.base.set_state(54);
                            recog.instruction()?;
                        }
                    }

                    _ => {}
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
//------------------- whileloop ----------------
pub type WhileloopContextAll<'input> = WhileloopContext<'input>;

pub type WhileloopContext<'input> = BaseParserRuleContext<'input, WhileloopContextExt<'input>>;

#[derive(Clone)]
pub struct WhileloopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for WhileloopContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for WhileloopContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_whileloop(self);
    }
}

impl<'input> CustomRuleContext<'input> for WhileloopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_whileloop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_whileloop }
}
antlr_rust::type_id! {WhileloopContextExt<'a>}

impl<'input> WhileloopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<WhileloopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            WhileloopContextExt { ph: PhantomData },
        ))
    }
}

pub trait WhileloopContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<WhileloopContextExt<'input>>
{
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn instruction(&self) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> WhileloopContextAttrs<'input> for WhileloopContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn whileloop(&mut self) -> Result<Rc<WhileloopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = WhileloopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_whileloop);
        let mut _localctx: Rc<WhileloopContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(57);
                recog.base.match_token(T__6, &mut recog.err_handler)?;

                recog.base.set_state(58);
                recog.base.match_token(T__3, &mut recog.err_handler)?;

                /*InvokeRule string*/
                recog.base.set_state(59);
                recog.string()?;

                recog.base.set_state(60);
                recog.base.match_token(T__4, &mut recog.err_handler)?;

                /*InvokeRule instruction*/
                recog.base.set_state(61);
                recog.instruction()?;
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
//------------------- doloop ----------------
pub type DoloopContextAll<'input> = DoloopContext<'input>;

pub type DoloopContext<'input> = BaseParserRuleContext<'input, DoloopContextExt<'input>>;

#[derive(Clone)]
pub struct DoloopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for DoloopContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for DoloopContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_doloop(self);
    }
}

impl<'input> CustomRuleContext<'input> for DoloopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_doloop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_doloop }
}
antlr_rust::type_id! {DoloopContextExt<'a>}

impl<'input> DoloopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DoloopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DoloopContextExt { ph: PhantomData },
        ))
    }
}

pub trait DoloopContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<DoloopContextExt<'input>>
{
    fn instruction(&self) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> DoloopContextAttrs<'input> for DoloopContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn doloop(&mut self) -> Result<Rc<DoloopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DoloopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_doloop);
        let mut _localctx: Rc<DoloopContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(63);
                recog.base.match_token(T__7, &mut recog.err_handler)?;

                /*InvokeRule instruction*/
                recog.base.set_state(64);
                recog.instruction()?;

                recog.base.set_state(65);
                recog.base.match_token(T__6, &mut recog.err_handler)?;

                recog.base.set_state(66);
                recog.base.match_token(T__3, &mut recog.err_handler)?;

                /*InvokeRule string*/
                recog.base.set_state(67);
                recog.string()?;

                recog.base.set_state(68);
                recog.base.match_token(T__4, &mut recog.err_handler)?;

                recog.base.set_state(70);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__1 {
                    {
                        recog.base.set_state(69);
                        recog.base.match_token(T__1, &mut recog.err_handler)?;
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
//------------------- group ----------------
pub type GroupContextAll<'input> = GroupContext<'input>;

pub type GroupContext<'input> = BaseParserRuleContext<'input, GroupContextExt<'input>>;

#[derive(Clone)]
pub struct GroupContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for GroupContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a> for GroupContext<'input> {
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_group(self);
    }
}

impl<'input> CustomRuleContext<'input> for GroupContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_group
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_group }
}
antlr_rust::type_id! {GroupContextExt<'a>}

impl<'input> GroupContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<GroupContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            GroupContextExt { ph: PhantomData },
        ))
    }
}

pub trait GroupContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<GroupContextExt<'input>>
{
    fn program(&self) -> Option<Rc<ProgramContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> GroupContextAttrs<'input> for GroupContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn group(&mut self) -> Result<Rc<GroupContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = GroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_group);
        let mut _localctx: Rc<GroupContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(72);
                recog.base.match_token(T__8, &mut recog.err_handler)?;

                /*InvokeRule program*/
                recog.base.set_state(73);
                recog.program()?;

                recog.base.set_state(74);
                recog.base.match_token(T__9, &mut recog.err_handler)?;
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
//------------------- returninstruction ----------------
pub type ReturninstructionContextAll<'input> = ReturninstructionContext<'input>;

pub type ReturninstructionContext<'input> =
    BaseParserRuleContext<'input, ReturninstructionContextExt<'input>>;

#[derive(Clone)]
pub struct ReturninstructionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for ReturninstructionContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a>
    for ReturninstructionContext<'input>
{
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_returninstruction(self);
    }
}

impl<'input> CustomRuleContext<'input> for ReturninstructionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_returninstruction
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_returninstruction }
}
antlr_rust::type_id! {ReturninstructionContextExt<'a>}

impl<'input> ReturninstructionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ReturninstructionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ReturninstructionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ReturninstructionContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<ReturninstructionContextExt<'input>>
{
    fn instruction(&self) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ReturninstructionContextAttrs<'input> for ReturninstructionContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn returninstruction(
        &mut self,
    ) -> Result<Rc<ReturninstructionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ReturninstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_returninstruction);
        let mut _localctx: Rc<ReturninstructionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(76);
                recog.base.match_token(T__10, &mut recog.err_handler)?;

                /*InvokeRule instruction*/
                recog.base.set_state(77);
                recog.instruction()?;
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
//------------------- switchinstruction ----------------
pub type SwitchinstructionContextAll<'input> = SwitchinstructionContext<'input>;

pub type SwitchinstructionContext<'input> =
    BaseParserRuleContext<'input, SwitchinstructionContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchinstructionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> flowchartParserContext<'input> for SwitchinstructionContext<'input> {}

impl<'input, 'a> Listenable<dyn flowchartListener<'input> + 'a>
    for SwitchinstructionContext<'input>
{
    fn enter(&self, listener: &mut (dyn flowchartListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_switchinstruction(self);
    }
}

impl<'input> CustomRuleContext<'input> for SwitchinstructionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = flowchartParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_switchinstruction
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_switchinstruction }
}
antlr_rust::type_id! {SwitchinstructionContextExt<'a>}

impl<'input> SwitchinstructionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn flowchartParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SwitchinstructionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SwitchinstructionContextExt { ph: PhantomData },
        ))
    }
}

pub trait SwitchinstructionContextAttrs<'input>:
    flowchartParserContext<'input> + BorrowMut<SwitchinstructionContextExt<'input>>
{
    fn string_all(&self) -> Vec<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn string(&self, i: usize) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn instruction_all(&self) -> Vec<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> SwitchinstructionContextAttrs<'input> for SwitchinstructionContext<'input> {}

impl<'input, I, H> flowchartParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn switchinstruction(
        &mut self,
    ) -> Result<Rc<SwitchinstructionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SwitchinstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_switchinstruction);
        let mut _localctx: Rc<SwitchinstructionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(79);
                recog.base.match_token(T__11, &mut recog.err_handler)?;

                recog.base.set_state(80);
                recog.base.match_token(T__3, &mut recog.err_handler)?;

                /*InvokeRule string*/
                recog.base.set_state(81);
                recog.string()?;

                recog.base.set_state(82);
                recog.base.match_token(T__4, &mut recog.err_handler)?;

                recog.base.set_state(83);
                recog.base.match_token(T__8, &mut recog.err_handler)?;

                recog.base.set_state(91);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__12 {
                    {
                        {
                            recog.base.set_state(84);
                            recog.base.match_token(T__12, &mut recog.err_handler)?;

                            /*InvokeRule string*/
                            recog.base.set_state(85);
                            recog.string()?;

                            recog.base.set_state(86);
                            recog.base.match_token(T__13, &mut recog.err_handler)?;

                            /*InvokeRule instruction*/
                            recog.base.set_state(87);
                            recog.instruction()?;
                        }
                    }
                    recog.base.set_state(93);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(94);
                recog.base.match_token(T__9, &mut recog.err_handler)?;
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
	\x15\x63\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x03\x02\x07\x02\x1a\x0a\x02\x0c\
	\x02\x0e\x02\x1d\x0b\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x05\x03\x27\x0a\x03\x03\x04\x03\x04\x05\x04\x2b\x0a\x04\
	\x03\x05\x03\x05\x05\x05\x2f\x0a\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\x3a\x0a\x07\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\
	\x03\x09\x03\x09\x05\x09\x49\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\x5c\x0a\x0c\x0c\x0c\x0e\x0c\x5f\x0b\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x02\x02\x0d\x02\x04\x06\x08\x0a\x0c\x0e\x10\
	\x12\x14\x16\x02\x03\x03\x02\x14\x15\x02\x64\x02\x1b\x03\x02\x02\x02\x04\
	\x26\x03\x02\x02\x02\x06\x28\x03\x02\x02\x02\x08\x2c\x03\x02\x02\x02\x0a\
	\x30\x03\x02\x02\x02\x0c\x32\x03\x02\x02\x02\x0e\x3b\x03\x02\x02\x02\x10\
	\x41\x03\x02\x02\x02\x12\x4a\x03\x02\x02\x02\x14\x4e\x03\x02\x02\x02\x16\
	\x51\x03\x02\x02\x02\x18\x1a\x05\x04\x03\x02\x19\x18\x03\x02\x02\x02\x1a\
	\x1d\x03\x02\x02\x02\x1b\x19\x03\x02\x02\x02\x1b\x1c\x03\x02\x02\x02\x1c\
	\x03\x03\x02\x02\x02\x1d\x1b\x03\x02\x02\x02\x1e\x27\x05\x06\x04\x02\x1f\
	\x27\x05\x08\x05\x02\x20\x27\x05\x0c\x07\x02\x21\x27\x05\x12\x0a\x02\x22\
	\x27\x05\x0e\x08\x02\x23\x27\x05\x10\x09\x02\x24\x27\x05\x14\x0b\x02\x25\
	\x27\x05\x16\x0c\x02\x26\x1e\x03\x02\x02\x02\x26\x1f\x03\x02\x02\x02\x26\
	\x20\x03\x02\x02\x02\x26\x21\x03\x02\x02\x02\x26\x22\x03\x02\x02\x02\x26\
	\x23\x03\x02\x02\x02\x26\x24\x03\x02\x02\x02\x26\x25\x03\x02\x02\x02\x27\
	\x05\x03\x02\x02\x02\x28\x2a\x07\x03\x02\x02\x29\x2b\x07\x04\x02\x02\x2a\
	\x29\x03\x02\x02\x02\x2a\x2b\x03\x02\x02\x02\x2b\x07\x03\x02\x02\x02\x2c\
	\x2e\x05\x0a\x06\x02\x2d\x2f\x07\x04\x02\x02\x2e\x2d\x03\x02\x02\x02\x2e\
	\x2f\x03\x02\x02\x02\x2f\x09\x03\x02\x02\x02\x30\x31\x09\x02\x02\x02\x31\
	\x0b\x03\x02\x02\x02\x32\x33\x07\x05\x02\x02\x33\x34\x07\x06\x02\x02\x34\
	\x35\x05\x0a\x06\x02\x35\x36\x07\x07\x02\x02\x36\x39\x05\x04\x03\x02\x37\
	\x38\x07\x08\x02\x02\x38\x3a\x05\x04\x03\x02\x39\x37\x03\x02\x02\x02\x39\
	\x3a\x03\x02\x02\x02\x3a\x0d\x03\x02\x02\x02\x3b\x3c\x07\x09\x02\x02\x3c\
	\x3d\x07\x06\x02\x02\x3d\x3e\x05\x0a\x06\x02\x3e\x3f\x07\x07\x02\x02\x3f\
	\x40\x05\x04\x03\x02\x40\x0f\x03\x02\x02\x02\x41\x42\x07\x0a\x02\x02\x42\
	\x43\x05\x04\x03\x02\x43\x44\x07\x09\x02\x02\x44\x45\x07\x06\x02\x02\x45\
	\x46\x05\x0a\x06\x02\x46\x48\x07\x07\x02\x02\x47\x49\x07\x04\x02\x02\x48\
	\x47\x03\x02\x02\x02\x48\x49\x03\x02\x02\x02\x49\x11\x03\x02\x02\x02\x4a\
	\x4b\x07\x0b\x02\x02\x4b\x4c\x05\x02\x02\x02\x4c\x4d\x07\x0c\x02\x02\x4d\
	\x13\x03\x02\x02\x02\x4e\x4f\x07\x0d\x02\x02\x4f\x50\x05\x04\x03\x02\x50\
	\x15\x03\x02\x02\x02\x51\x52\x07\x0e\x02\x02\x52\x53\x07\x06\x02\x02\x53\
	\x54\x05\x0a\x06\x02\x54\x55\x07\x07\x02\x02\x55\x5d\x07\x0b\x02\x02\x56\
	\x57\x07\x0f\x02\x02\x57\x58\x05\x0a\x06\x02\x58\x59\x07\x10\x02\x02\x59\
	\x5a\x05\x04\x03\x02\x5a\x5c\x03\x02\x02\x02\x5b\x56\x03\x02\x02\x02\x5c\
	\x5f\x03\x02\x02\x02\x5d\x5b\x03\x02\x02\x02\x5d\x5e\x03\x02\x02\x02\x5e\
	\x60\x03\x02\x02\x02\x5f\x5d\x03\x02\x02\x02\x60\x61\x07\x0c\x02\x02\x61\
	\x17\x03\x02\x02\x02\x09\x1b\x26\x2a\x2e\x39\x48\x5d";
