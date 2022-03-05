// Generated from math.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::mathlistener::*;
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
pub const STRING: isize = 3;
pub const LPAREN: isize = 4;
pub const RPAREN: isize = 5;
pub const LBRACE: isize = 6;
pub const RBRACE: isize = 7;
pub const LBRACKET: isize = 8;
pub const RBRACKET: isize = 9;
pub const PLUS: isize = 10;
pub const MINUS: isize = 11;
pub const TIMES: isize = 12;
pub const BANG: isize = 13;
pub const DIV: isize = 14;
pub const GT: isize = 15;
pub const LT: isize = 16;
pub const GE: isize = 17;
pub const LE: isize = 18;
pub const EQ: isize = 19;
pub const POW: isize = 20;
pub const SUBSCRIPT: isize = 21;
pub const EOL: isize = 22;
pub const WS: isize = 23;
pub const VARIABLE: isize = 24;
pub const RULE_equation: usize = 0;
pub const RULE_expression: usize = 1;
pub const RULE_term: usize = 2;
pub const RULE_factor: usize = 3;
pub const RULE_valueBang: usize = 4;
pub const RULE_value: usize = 5;
pub const RULE_atom: usize = 6;
pub const RULE_function: usize = 7;
pub const RULE_variable: usize = 8;
pub const RULE_matrix: usize = 9;
pub const RULE_matrixLine: usize = 10;
pub const RULE_relop: usize = 11;
pub const RULE_addop: usize = 12;
pub const RULE_mulop: usize = 13;
pub const RULE_powop: usize = 14;
pub const ruleNames: [&'static str; 15] = [
    "equation",
    "expression",
    "term",
    "factor",
    "valueBang",
    "value",
    "atom",
    "function",
    "variable",
    "matrix",
    "matrixLine",
    "relop",
    "addop",
    "mulop",
    "powop",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 22] = [
    None,
    Some("','"),
    Some("';'"),
    None,
    Some("'('"),
    Some("')'"),
    Some("'{'"),
    Some("'}'"),
    Some("'['"),
    Some("']'"),
    Some("'+'"),
    Some("'-'"),
    Some("'*'"),
    Some("'!'"),
    Some("'/'"),
    Some("'>'"),
    Some("'<'"),
    Some("'>='"),
    Some("'<='"),
    Some("'='"),
    Some("'^'"),
    Some("'_'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 25] = [
    None,
    None,
    None,
    Some("STRING"),
    Some("LPAREN"),
    Some("RPAREN"),
    Some("LBRACE"),
    Some("RBRACE"),
    Some("LBRACKET"),
    Some("RBRACKET"),
    Some("PLUS"),
    Some("MINUS"),
    Some("TIMES"),
    Some("BANG"),
    Some("DIV"),
    Some("GT"),
    Some("LT"),
    Some("GE"),
    Some("LE"),
    Some("EQ"),
    Some("POW"),
    Some("SUBSCRIPT"),
    Some("EOL"),
    Some("WS"),
    Some("VARIABLE"),
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

type BaseParserType<'input, I> =
    BaseParser<'input, mathParserExt, I, mathParserContextType, dyn mathListener<'input> + 'input>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type mathTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, mathParserContextType, dyn mathListener<'input> + 'a>;

/// Parser for math grammar
pub struct mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> mathParser<'input, I, H>
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
            base: BaseParser::new_base_parser(input, Arc::clone(&interpreter), mathParserExt {}),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> mathParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> mathParser<'input, I, DefaultErrorStrategy<'input, mathParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for mathParser
pub trait mathParserContext<'input>:
    for<'x> Listenable<dyn mathListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = mathParserContextType>
{
}

impl<'input> mathParserContext<'input> for TerminalNode<'input, mathParserContextType> {}
impl<'input> mathParserContext<'input> for ErrorNode<'input, mathParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn mathParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn mathListener<'input> + 'input {}

pub struct mathParserContextType;
antlr_rust::type_id! {mathParserContextType}

impl<'input> ParserNodeType<'input> for mathParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn mathParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct mathParserExt {}

impl mathParserExt {}

impl<'input> TokenAware<'input> for mathParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for mathParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for mathParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "math.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn mathParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            4 => mathParser::<'input, I, _>::valueBang_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> mathParser<'input, I, DefaultErrorStrategy<'input, mathParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn valueBang_sempred(
        _localctx: Option<&ValueBangContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 1),
            _ => true,
        }
    }
}
//------------------- equation ----------------
pub type EquationContextAll<'input> = EquationContext<'input>;

pub type EquationContext<'input> = BaseParserRuleContext<'input, EquationContextExt<'input>>;

#[derive(Clone)]
pub struct EquationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for EquationContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for EquationContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_equation(self);
    }
}

impl<'input> CustomRuleContext<'input> for EquationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_equation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_equation }
}
antlr_rust::type_id! {EquationContextExt<'a>}

impl<'input> EquationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<EquationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            EquationContextExt { ph: PhantomData },
        ))
    }
}

pub trait EquationContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<EquationContextExt<'input>>
{
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn relop_all(&self) -> Vec<Rc<RelopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn relop(&self, i: usize) -> Option<Rc<RelopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> EquationContextAttrs<'input> for EquationContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn equation(&mut self) -> Result<Rc<EquationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = EquationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_equation);
        let mut _localctx: Rc<EquationContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expression*/
                recog.base.set_state(30);
                recog.expression()?;

                recog.base.set_state(36);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << GT)
                            | (1usize << LT)
                            | (1usize << GE)
                            | (1usize << LE)
                            | (1usize << EQ)))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule relop*/
                            recog.base.set_state(31);
                            recog.relop()?;

                            /*InvokeRule expression*/
                            recog.base.set_state(32);
                            recog.expression()?;
                        }
                    }
                    recog.base.set_state(38);
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
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;

pub type ExpressionContext<'input> = BaseParserRuleContext<'input, ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for ExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expression(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::type_id! {ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExpressionContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>
{
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn addop_all(&self) -> Vec<Rc<AddopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn addop(&self, i: usize) -> Option<Rc<AddopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expression(&mut self) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule term*/
                recog.base.set_state(39);
                recog.term()?;

                recog.base.set_state(45);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == PLUS || _la == MINUS {
                    {
                        {
                            /*InvokeRule addop*/
                            recog.base.set_state(40);
                            recog.addop()?;

                            /*InvokeRule term*/
                            recog.base.set_state(41);
                            recog.term()?;
                        }
                    }
                    recog.base.set_state(47);
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
//------------------- term ----------------
pub type TermContextAll<'input> = TermContext<'input>;

pub type TermContext<'input> = BaseParserRuleContext<'input, TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for TermContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for TermContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term(self);
    }
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::type_id! {TermContextExt<'a>}

impl<'input> TermContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TermContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TermContextExt { ph: PhantomData },
        ))
    }
}

pub trait TermContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<TermContextExt<'input>>
{
    fn factor_all(&self) -> Vec<Rc<FactorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn factor(&self, i: usize) -> Option<Rc<FactorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn mulop_all(&self) -> Vec<Rc<MulopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn mulop(&self, i: usize) -> Option<Rc<MulopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> TermContextAttrs<'input> for TermContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn term(&mut self) -> Result<Rc<TermContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_term);
        let mut _localctx: Rc<TermContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule factor*/
                recog.base.set_state(48);
                recog.factor()?;

                recog.base.set_state(54);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == TIMES || _la == DIV {
                    {
                        {
                            /*InvokeRule mulop*/
                            recog.base.set_state(49);
                            recog.mulop()?;

                            /*InvokeRule factor*/
                            recog.base.set_state(50);
                            recog.factor()?;
                        }
                    }
                    recog.base.set_state(56);
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
//------------------- factor ----------------
pub type FactorContextAll<'input> = FactorContext<'input>;

pub type FactorContext<'input> = BaseParserRuleContext<'input, FactorContextExt<'input>>;

#[derive(Clone)]
pub struct FactorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for FactorContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for FactorContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_factor(self);
    }
}

impl<'input> CustomRuleContext<'input> for FactorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_factor
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_factor }
}
antlr_rust::type_id! {FactorContextExt<'a>}

impl<'input> FactorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FactorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FactorContextExt { ph: PhantomData },
        ))
    }
}

pub trait FactorContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<FactorContextExt<'input>>
{
    fn valueBang_all(&self) -> Vec<Rc<ValueBangContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn valueBang(&self, i: usize) -> Option<Rc<ValueBangContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn powop_all(&self) -> Vec<Rc<PowopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn powop(&self, i: usize) -> Option<Rc<PowopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> FactorContextAttrs<'input> for FactorContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn factor(&mut self) -> Result<Rc<FactorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FactorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_factor);
        let mut _localctx: Rc<FactorContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule valueBang*/
                recog.base.set_state(57);
                recog.valueBang_rec(0)?;

                recog.base.set_state(63);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == POW || _la == SUBSCRIPT {
                    {
                        {
                            /*InvokeRule powop*/
                            recog.base.set_state(58);
                            recog.powop()?;

                            /*InvokeRule valueBang*/
                            recog.base.set_state(59);
                            recog.valueBang_rec(0)?;
                        }
                    }
                    recog.base.set_state(65);
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
//------------------- valueBang ----------------
pub type ValueBangContextAll<'input> = ValueBangContext<'input>;

pub type ValueBangContext<'input> = BaseParserRuleContext<'input, ValueBangContextExt<'input>>;

#[derive(Clone)]
pub struct ValueBangContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for ValueBangContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for ValueBangContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueBang(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueBangContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_valueBang
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_valueBang }
}
antlr_rust::type_id! {ValueBangContextExt<'a>}

impl<'input> ValueBangContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ValueBangContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ValueBangContextExt { ph: PhantomData },
        ))
    }
}

pub trait ValueBangContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<ValueBangContextExt<'input>>
{
    fn value(&self) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn valueBang(&self) -> Option<Rc<ValueBangContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BANG
    /// Returns `None` if there is no child corresponding to token BANG
    fn BANG(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BANG, 0)
    }
}

impl<'input> ValueBangContextAttrs<'input> for ValueBangContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn valueBang(&mut self) -> Result<Rc<ValueBangContextAll<'input>>, ANTLRError> {
        self.valueBang_rec(0)
    }

    fn valueBang_rec(&mut self, _p: isize) -> Result<Rc<ValueBangContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = ValueBangContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 8, RULE_valueBang, _p);
        let mut _localctx: Rc<ValueBangContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 8;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    /*InvokeRule value*/
                    recog.base.set_state(67);
                    recog.value()?;
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(73);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                /*recRuleAltStartAction*/
                                let mut tmp =
                                    ValueBangContextExt::new(_parentctx.clone(), _parentState);
                                recog.push_new_recursion_context(
                                    tmp.clone(),
                                    _startState,
                                    RULE_valueBang,
                                );
                                _localctx = tmp;
                                recog.base.set_state(69);
                                if !({ recog.precpred(None, 1) }) {
                                    Err(FailedPredicateError::new(
                                        &mut recog.base,
                                        Some("recog.precpred(None, 1)".to_owned()),
                                        None,
                                    ))?;
                                }
                                recog.base.set_state(70);
                                recog.base.match_token(BANG, &mut recog.err_handler)?;
                            }
                        }
                    }
                    recog.base.set_state(75);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
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
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;

pub type ValueContext<'input> = BaseParserRuleContext<'input, ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for ValueContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for ValueContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::type_id! {ValueContextExt<'a>}

impl<'input> ValueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ValueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ValueContextExt { ph: PhantomData },
        ))
    }
}

pub trait ValueContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<ValueContextExt<'input>>
{
    fn atom(&self) -> Option<Rc<AtomContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value(&mut self) -> Result<Rc<ValueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(77);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == PLUS || _la == MINUS {
                    {
                        recog.base.set_state(76);
                        _la = recog.base.input.la(1);
                        if { !(_la == PLUS || _la == MINUS) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                /*InvokeRule atom*/
                recog.base.set_state(79);
                recog.atom()?;
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
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;

pub type AtomContext<'input> = BaseParserRuleContext<'input, AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for AtomContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for AtomContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_atom(self);
    }
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atom
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr_rust::type_id! {AtomContextExt<'a>}

impl<'input> AtomContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AtomContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AtomContextExt { ph: PhantomData },
        ))
    }
}

pub trait AtomContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<AtomContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    fn variable(&self) -> Option<Rc<VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn function(&self) -> Option<Rc<FunctionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn matrix(&self) -> Option<Rc<MatrixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn atom(&mut self) -> Result<Rc<AtomContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(93);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(6, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(81);
                        recog.base.match_token(STRING, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule variable*/
                        recog.base.set_state(82);
                        recog.variable()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule function*/
                        recog.base.set_state(83);
                        recog.function()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule matrix*/
                        recog.base.set_state(84);
                        recog.matrix()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(85);
                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(86);
                        recog.expression()?;

                        recog.base.set_state(87);
                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(89);
                        recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(90);
                        recog.expression()?;

                        recog.base.set_state(91);
                        recog.base.match_token(RPAREN, &mut recog.err_handler)?;
                    }
                }

                _ => {}
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
//------------------- function ----------------
pub type FunctionContextAll<'input> = FunctionContext<'input>;

pub type FunctionContext<'input> = BaseParserRuleContext<'input, FunctionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for FunctionContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for FunctionContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_function(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_function
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_function }
}
antlr_rust::type_id! {FunctionContextExt<'a>}

impl<'input> FunctionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FunctionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FunctionContextExt { ph: PhantomData },
        ))
    }
}

pub trait FunctionContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<FunctionContextExt<'input>>
{
    fn variable(&self) -> Option<Rc<VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn equation_all(&self) -> Vec<Rc<EquationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn equation(&self, i: usize) -> Option<Rc<EquationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn function(&mut self) -> Result<Rc<FunctionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_function);
        let mut _localctx: Rc<FunctionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule variable*/
                recog.base.set_state(95);
                recog.variable()?;

                recog.base.set_state(96);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                /*InvokeRule equation*/
                recog.base.set_state(97);
                recog.equation()?;

                recog.base.set_state(102);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__0 {
                    {
                        {
                            recog.base.set_state(98);
                            recog.base.match_token(T__0, &mut recog.err_handler)?;

                            /*InvokeRule equation*/
                            recog.base.set_state(99);
                            recog.equation()?;
                        }
                    }
                    recog.base.set_state(104);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(105);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;
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
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;

pub type VariableContext<'input> = BaseParserRuleContext<'input, VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for VariableContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for VariableContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_variable(self);
    }
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_variable
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::type_id! {VariableContextExt<'a>}

impl<'input> VariableContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VariableContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VariableContextExt { ph: PhantomData },
        ))
    }
}

pub trait VariableContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<VariableContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token VARIABLE
    /// Returns `None` if there is no child corresponding to token VARIABLE
    fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARIABLE, 0)
    }
}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn variable(&mut self) -> Result<Rc<VariableContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(107);
                recog.base.match_token(VARIABLE, &mut recog.err_handler)?;
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
//------------------- matrix ----------------
pub type MatrixContextAll<'input> = MatrixContext<'input>;

pub type MatrixContext<'input> = BaseParserRuleContext<'input, MatrixContextExt<'input>>;

#[derive(Clone)]
pub struct MatrixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for MatrixContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for MatrixContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_matrix(self);
    }
}

impl<'input> CustomRuleContext<'input> for MatrixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_matrix
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_matrix }
}
antlr_rust::type_id! {MatrixContextExt<'a>}

impl<'input> MatrixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MatrixContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MatrixContextExt { ph: PhantomData },
        ))
    }
}

pub trait MatrixContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<MatrixContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACKET
    /// Returns `None` if there is no child corresponding to token LBRACKET
    fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACKET, 0)
    }
    fn matrixLine_all(&self) -> Vec<Rc<MatrixLineContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn matrixLine(&self, i: usize) -> Option<Rc<MatrixLineContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACKET
    /// Returns `None` if there is no child corresponding to token RBRACKET
    fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACKET, 0)
    }
}

impl<'input> MatrixContextAttrs<'input> for MatrixContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn matrix(&mut self) -> Result<Rc<MatrixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MatrixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_matrix);
        let mut _localctx: Rc<MatrixContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(109);
                recog.base.match_token(LBRACKET, &mut recog.err_handler)?;

                /*InvokeRule matrixLine*/
                recog.base.set_state(110);
                recog.matrixLine()?;

                recog.base.set_state(115);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__1 {
                    {
                        {
                            recog.base.set_state(111);
                            recog.base.match_token(T__1, &mut recog.err_handler)?;

                            /*InvokeRule matrixLine*/
                            recog.base.set_state(112);
                            recog.matrixLine()?;
                        }
                    }
                    recog.base.set_state(117);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(118);
                recog.base.match_token(RBRACKET, &mut recog.err_handler)?;
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
//------------------- matrixLine ----------------
pub type MatrixLineContextAll<'input> = MatrixLineContext<'input>;

pub type MatrixLineContext<'input> = BaseParserRuleContext<'input, MatrixLineContextExt<'input>>;

#[derive(Clone)]
pub struct MatrixLineContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for MatrixLineContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for MatrixLineContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_matrixLine(self);
    }
}

impl<'input> CustomRuleContext<'input> for MatrixLineContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_matrixLine
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_matrixLine }
}
antlr_rust::type_id! {MatrixLineContextExt<'a>}

impl<'input> MatrixLineContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MatrixLineContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MatrixLineContextExt { ph: PhantomData },
        ))
    }
}

pub trait MatrixLineContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<MatrixLineContextExt<'input>>
{
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> MatrixLineContextAttrs<'input> for MatrixLineContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn matrixLine(&mut self) -> Result<Rc<MatrixLineContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MatrixLineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_matrixLine);
        let mut _localctx: Rc<MatrixLineContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expression*/
                recog.base.set_state(120);
                recog.expression()?;

                recog.base.set_state(125);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__0 {
                    {
                        {
                            recog.base.set_state(121);
                            recog.base.match_token(T__0, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(122);
                            recog.expression()?;
                        }
                    }
                    recog.base.set_state(127);
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
//------------------- relop ----------------
pub type RelopContextAll<'input> = RelopContext<'input>;

pub type RelopContext<'input> = BaseParserRuleContext<'input, RelopContextExt<'input>>;

#[derive(Clone)]
pub struct RelopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for RelopContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for RelopContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_relop(self);
    }
}

impl<'input> CustomRuleContext<'input> for RelopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_relop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_relop }
}
antlr_rust::type_id! {RelopContextExt<'a>}

impl<'input> RelopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RelopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            RelopContextExt { ph: PhantomData },
        ))
    }
}

pub trait RelopContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<RelopContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GE
    /// Returns `None` if there is no child corresponding to token GE
    fn GE(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LE
    /// Returns `None` if there is no child corresponding to token LE
    fn LE(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LE, 0)
    }
}

impl<'input> RelopContextAttrs<'input> for RelopContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn relop(&mut self) -> Result<Rc<RelopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = RelopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_relop);
        let mut _localctx: Rc<RelopContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(128);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << GT)
                                | (1usize << LT)
                                | (1usize << GE)
                                | (1usize << LE)
                                | (1usize << EQ)))
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
//------------------- addop ----------------
pub type AddopContextAll<'input> = AddopContext<'input>;

pub type AddopContext<'input> = BaseParserRuleContext<'input, AddopContextExt<'input>>;

#[derive(Clone)]
pub struct AddopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for AddopContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for AddopContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_addop(self);
    }
}

impl<'input> CustomRuleContext<'input> for AddopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_addop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_addop }
}
antlr_rust::type_id! {AddopContextExt<'a>}

impl<'input> AddopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AddopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AddopContextExt { ph: PhantomData },
        ))
    }
}

pub trait AddopContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<AddopContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> AddopContextAttrs<'input> for AddopContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn addop(&mut self) -> Result<Rc<AddopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AddopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_addop);
        let mut _localctx: Rc<AddopContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(130);
                _la = recog.base.input.la(1);
                if { !(_la == PLUS || _la == MINUS) } {
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
//------------------- mulop ----------------
pub type MulopContextAll<'input> = MulopContext<'input>;

pub type MulopContext<'input> = BaseParserRuleContext<'input, MulopContextExt<'input>>;

#[derive(Clone)]
pub struct MulopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for MulopContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for MulopContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mulop(self);
    }
}

impl<'input> CustomRuleContext<'input> for MulopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mulop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mulop }
}
antlr_rust::type_id! {MulopContextExt<'a>}

impl<'input> MulopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MulopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MulopContextExt { ph: PhantomData },
        ))
    }
}

pub trait MulopContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<MulopContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token TIMES
    /// Returns `None` if there is no child corresponding to token TIMES
    fn TIMES(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TIMES, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIV, 0)
    }
}

impl<'input> MulopContextAttrs<'input> for MulopContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mulop(&mut self) -> Result<Rc<MulopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MulopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_mulop);
        let mut _localctx: Rc<MulopContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(132);
                _la = recog.base.input.la(1);
                if { !(_la == TIMES || _la == DIV) } {
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
//------------------- powop ----------------
pub type PowopContextAll<'input> = PowopContext<'input>;

pub type PowopContext<'input> = BaseParserRuleContext<'input, PowopContextExt<'input>>;

#[derive(Clone)]
pub struct PowopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> mathParserContext<'input> for PowopContext<'input> {}

impl<'input, 'a> Listenable<dyn mathListener<'input> + 'a> for PowopContext<'input> {
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_powop(self);
    }
}

impl<'input> CustomRuleContext<'input> for PowopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = mathParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_powop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_powop }
}
antlr_rust::type_id! {PowopContextExt<'a>}

impl<'input> PowopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn mathParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PowopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PowopContextExt { ph: PhantomData },
        ))
    }
}

pub trait PowopContextAttrs<'input>:
    mathParserContext<'input> + BorrowMut<PowopContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token POW
    /// Returns `None` if there is no child corresponding to token POW
    fn POW(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SUBSCRIPT
    /// Returns `None` if there is no child corresponding to token SUBSCRIPT
    fn SUBSCRIPT(&self) -> Option<Rc<TerminalNode<'input, mathParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUBSCRIPT, 0)
    }
}

impl<'input> PowopContextAttrs<'input> for PowopContext<'input> {}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn powop(&mut self) -> Result<Rc<PowopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PowopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_powop);
        let mut _localctx: Rc<PowopContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(134);
                _la = recog.base.input.la(1);
                if { !(_la == POW || _la == SUBSCRIPT) } {
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
	\x1a\u{8b}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x03\x02\x03\x02\x03\x02\x03\x02\x07\x02\
	\x25\x0a\x02\x0c\x02\x0e\x02\x28\x0b\x02\x03\x03\x03\x03\x03\x03\x03\x03\
	\x07\x03\x2e\x0a\x03\x0c\x03\x0e\x03\x31\x0b\x03\x03\x04\x03\x04\x03\x04\
	\x03\x04\x07\x04\x37\x0a\x04\x0c\x04\x0e\x04\x3a\x0b\x04\x03\x05\x03\x05\
	\x03\x05\x03\x05\x07\x05\x40\x0a\x05\x0c\x05\x0e\x05\x43\x0b\x05\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x07\x06\x4a\x0a\x06\x0c\x06\x0e\x06\x4d\
	\x0b\x06\x03\x07\x05\x07\x50\x0a\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\
	\x08\x05\x08\x60\x0a\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x07\x09\
	\x67\x0a\x09\x0c\x09\x0e\x09\x6a\x0b\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x07\x0b\x74\x0a\x0b\x0c\x0b\x0e\x0b\x77\
	\x0b\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x07\x0c\x7e\x0a\x0c\x0c\
	\x0c\x0e\x0c\u{81}\x0b\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\
	\x03\x10\x03\x10\x03\x10\x02\x03\x0a\x11\x02\x04\x06\x08\x0a\x0c\x0e\x10\
	\x12\x14\x16\x18\x1a\x1c\x1e\x02\x06\x03\x02\x0c\x0d\x03\x02\x11\x15\x04\
	\x02\x0e\x0e\x10\x10\x03\x02\x16\x17\x02\u{89}\x02\x20\x03\x02\x02\x02\x04\
	\x29\x03\x02\x02\x02\x06\x32\x03\x02\x02\x02\x08\x3b\x03\x02\x02\x02\x0a\
	\x44\x03\x02\x02\x02\x0c\x4f\x03\x02\x02\x02\x0e\x5f\x03\x02\x02\x02\x10\
	\x61\x03\x02\x02\x02\x12\x6d\x03\x02\x02\x02\x14\x6f\x03\x02\x02\x02\x16\
	\x7a\x03\x02\x02\x02\x18\u{82}\x03\x02\x02\x02\x1a\u{84}\x03\x02\x02\x02\
	\x1c\u{86}\x03\x02\x02\x02\x1e\u{88}\x03\x02\x02\x02\x20\x26\x05\x04\x03\
	\x02\x21\x22\x05\x18\x0d\x02\x22\x23\x05\x04\x03\x02\x23\x25\x03\x02\x02\
	\x02\x24\x21\x03\x02\x02\x02\x25\x28\x03\x02\x02\x02\x26\x24\x03\x02\x02\
	\x02\x26\x27\x03\x02\x02\x02\x27\x03\x03\x02\x02\x02\x28\x26\x03\x02\x02\
	\x02\x29\x2f\x05\x06\x04\x02\x2a\x2b\x05\x1a\x0e\x02\x2b\x2c\x05\x06\x04\
	\x02\x2c\x2e\x03\x02\x02\x02\x2d\x2a\x03\x02\x02\x02\x2e\x31\x03\x02\x02\
	\x02\x2f\x2d\x03\x02\x02\x02\x2f\x30\x03\x02\x02\x02\x30\x05\x03\x02\x02\
	\x02\x31\x2f\x03\x02\x02\x02\x32\x38\x05\x08\x05\x02\x33\x34\x05\x1c\x0f\
	\x02\x34\x35\x05\x08\x05\x02\x35\x37\x03\x02\x02\x02\x36\x33\x03\x02\x02\
	\x02\x37\x3a\x03\x02\x02\x02\x38\x36\x03\x02\x02\x02\x38\x39\x03\x02\x02\
	\x02\x39\x07\x03\x02\x02\x02\x3a\x38\x03\x02\x02\x02\x3b\x41\x05\x0a\x06\
	\x02\x3c\x3d\x05\x1e\x10\x02\x3d\x3e\x05\x0a\x06\x02\x3e\x40\x03\x02\x02\
	\x02\x3f\x3c\x03\x02\x02\x02\x40\x43\x03\x02\x02\x02\x41\x3f\x03\x02\x02\
	\x02\x41\x42\x03\x02\x02\x02\x42\x09\x03\x02\x02\x02\x43\x41\x03\x02\x02\
	\x02\x44\x45\x08\x06\x01\x02\x45\x46\x05\x0c\x07\x02\x46\x4b\x03\x02\x02\
	\x02\x47\x48\x0c\x03\x02\x02\x48\x4a\x07\x0f\x02\x02\x49\x47\x03\x02\x02\
	\x02\x4a\x4d\x03\x02\x02\x02\x4b\x49\x03\x02\x02\x02\x4b\x4c\x03\x02\x02\
	\x02\x4c\x0b\x03\x02\x02\x02\x4d\x4b\x03\x02\x02\x02\x4e\x50\x09\x02\x02\
	\x02\x4f\x4e\x03\x02\x02\x02\x4f\x50\x03\x02\x02\x02\x50\x51\x03\x02\x02\
	\x02\x51\x52\x05\x0e\x08\x02\x52\x0d\x03\x02\x02\x02\x53\x60\x07\x05\x02\
	\x02\x54\x60\x05\x12\x0a\x02\x55\x60\x05\x10\x09\x02\x56\x60\x05\x14\x0b\
	\x02\x57\x58\x07\x08\x02\x02\x58\x59\x05\x04\x03\x02\x59\x5a\x07\x09\x02\
	\x02\x5a\x60\x03\x02\x02\x02\x5b\x5c\x07\x06\x02\x02\x5c\x5d\x05\x04\x03\
	\x02\x5d\x5e\x07\x07\x02\x02\x5e\x60\x03\x02\x02\x02\x5f\x53\x03\x02\x02\
	\x02\x5f\x54\x03\x02\x02\x02\x5f\x55\x03\x02\x02\x02\x5f\x56\x03\x02\x02\
	\x02\x5f\x57\x03\x02\x02\x02\x5f\x5b\x03\x02\x02\x02\x60\x0f\x03\x02\x02\
	\x02\x61\x62\x05\x12\x0a\x02\x62\x63\x07\x06\x02\x02\x63\x68\x05\x02\x02\
	\x02\x64\x65\x07\x03\x02\x02\x65\x67\x05\x02\x02\x02\x66\x64\x03\x02\x02\
	\x02\x67\x6a\x03\x02\x02\x02\x68\x66\x03\x02\x02\x02\x68\x69\x03\x02\x02\
	\x02\x69\x6b\x03\x02\x02\x02\x6a\x68\x03\x02\x02\x02\x6b\x6c\x07\x07\x02\
	\x02\x6c\x11\x03\x02\x02\x02\x6d\x6e\x07\x1a\x02\x02\x6e\x13\x03\x02\x02\
	\x02\x6f\x70\x07\x0a\x02\x02\x70\x75\x05\x16\x0c\x02\x71\x72\x07\x04\x02\
	\x02\x72\x74\x05\x16\x0c\x02\x73\x71\x03\x02\x02\x02\x74\x77\x03\x02\x02\
	\x02\x75\x73\x03\x02\x02\x02\x75\x76\x03\x02\x02\x02\x76\x78\x03\x02\x02\
	\x02\x77\x75\x03\x02\x02\x02\x78\x79\x07\x0b\x02\x02\x79\x15\x03\x02\x02\
	\x02\x7a\x7f\x05\x04\x03\x02\x7b\x7c\x07\x03\x02\x02\x7c\x7e\x05\x04\x03\
	\x02\x7d\x7b\x03\x02\x02\x02\x7e\u{81}\x03\x02\x02\x02\x7f\x7d\x03\x02\x02\
	\x02\x7f\u{80}\x03\x02\x02\x02\u{80}\x17\x03\x02\x02\x02\u{81}\x7f\x03\x02\
	\x02\x02\u{82}\u{83}\x09\x03\x02\x02\u{83}\x19\x03\x02\x02\x02\u{84}\u{85}\
	\x09\x02\x02\x02\u{85}\x1b\x03\x02\x02\x02\u{86}\u{87}\x09\x04\x02\x02\u{87}\
	\x1d\x03\x02\x02\x02\u{88}\u{89}\x09\x05\x02\x02\u{89}\x1f\x03\x02\x02\x02\
	\x0c\x26\x2f\x38\x41\x4b\x4f\x5f\x68\x75\x7f";
