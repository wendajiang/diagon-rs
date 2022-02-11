// Generated from sequence.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::sequencelistener::*;
use super::sequencevisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const NormalRightArrow:isize=2; 
		pub const NormalLeftArrow:isize=3; 
		pub const Comma:isize=4; 
		pub const Less:isize=5; 
		pub const More:isize=6; 
		pub const Colon:isize=7; 
		pub const EOL:isize=8; 
		pub const Number:isize=9; 
		pub const Space:isize=10; 
		pub const Other:isize=11;
	pub const RULE_program:usize = 0; 
	pub const RULE_command:usize = 1; 
	pub const RULE_messageCommand:usize = 2; 
	pub const RULE_dependencyCommand:usize = 3; 
	pub const RULE_dependency:usize = 4; 
	pub const RULE_dependencyID:usize = 5; 
	pub const RULE_dependencies:usize = 6; 
	pub const RULE_text:usize = 7; 
	pub const RULE_textInternal:usize = 8; 
	pub const RULE_number:usize = 9; 
	pub const RULE_comparison:usize = 10; 
	pub const RULE_arrow:usize = 11;
	pub const ruleNames: [&'static str; 12] =  [
		"program", "command", "messageCommand", "dependencyCommand", "dependency", 
		"dependencyID", "dependencies", "text", "textInternal", "number", "comparison", 
		"arrow"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("')'"), Some("'->'"), Some("'<-'"), Some("':'"), Some("'<'"), 
		Some("'>'"), Some("','")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
		None, None, Some("NormalRightArrow"), Some("NormalLeftArrow"), Some("Comma"), 
		Some("Less"), Some("More"), Some("Colon"), Some("EOL"), Some("Number"), 
		Some("Space"), Some("Other")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,sequenceParserExt, I, sequenceParserContextType , dyn sequenceListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type sequenceTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, sequenceParserContextType , dyn sequenceListener<'input> + 'a>;

/// Parser for sequence grammar
pub struct sequenceParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				sequenceParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> sequenceParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> sequenceParser<'input, I, DefaultErrorStrategy<'input,sequenceParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for sequenceParser
pub trait sequenceParserContext<'input>:
	for<'x> Listenable<dyn sequenceListener<'input> + 'x > + 
	for<'x> Visitable<dyn sequenceVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=sequenceParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn sequenceParserContext<'input> + 'input
where
    T: sequenceVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn sequenceVisitor<'input> + 'x))
    }
}

impl<'input> sequenceParserContext<'input> for TerminalNode<'input,sequenceParserContextType> {}
impl<'input> sequenceParserContext<'input> for ErrorNode<'input,sequenceParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn sequenceParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn sequenceListener<'input> + 'input{}

pub struct sequenceParserContextType;
antlr_rust::type_id!{sequenceParserContextType}

impl<'input> ParserNodeType<'input> for sequenceParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn sequenceParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct sequenceParserExt{
}

impl sequenceParserExt{
}


impl<'input> TokenAware<'input> for sequenceParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for sequenceParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for sequenceParserExt{
	fn get_grammar_file_name(&self) -> & str{ "sequence.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for ProgramContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_program(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_program(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::type_id!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn command_all(&self) ->  Vec<Rc<CommandContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn command(&self, i: usize) -> Option<Rc<CommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token EOL in current rule
fn EOL_all(&self) -> Vec<Rc<TerminalNode<'input,sequenceParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EOL, starting from 0.
/// Returns `None` if number of children corresponding to token EOL is less or equal than `i`.
fn EOL(&self, i: usize) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(EOL, i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
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
			{
			recog.base.set_state(25);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << NormalRightArrow) | (1usize << NormalLeftArrow) | (1usize << Comma) | (1usize << Less) | (1usize << More) | (1usize << Colon) | (1usize << Number) | (1usize << Space) | (1usize << Other))) != 0) {
				{
				/*InvokeRule command*/
				recog.base.set_state(24);
				recog.command()?;

				}
			}

			recog.base.set_state(33);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==EOL {
				{
				{
				recog.base.set_state(27);
				recog.base.match_token(EOL,&mut recog.err_handler)?;

				recog.base.set_state(29);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << NormalRightArrow) | (1usize << NormalLeftArrow) | (1usize << Comma) | (1usize << Less) | (1usize << More) | (1usize << Colon) | (1usize << Number) | (1usize << Space) | (1usize << Other))) != 0) {
					{
					/*InvokeRule command*/
					recog.base.set_state(28);
					recog.command()?;

					}
				}

				}
				}
				recog.base.set_state(35);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			recog.base.set_state(36);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- command ----------------
pub type CommandContextAll<'input> = CommandContext<'input>;


pub type CommandContext<'input> = BaseParserRuleContext<'input,CommandContextExt<'input>>;

#[derive(Clone)]
pub struct CommandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for CommandContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for CommandContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_command(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_command(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for CommandContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_command(self);
	}
}

impl<'input> CustomRuleContext<'input> for CommandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_command }
	//fn type_rule_index() -> usize where Self: Sized { RULE_command }
}
antlr_rust::type_id!{CommandContextExt<'a>}

impl<'input> CommandContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommandContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<CommandContextExt<'input>>{

fn messageCommand(&self) -> Option<Rc<MessageCommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dependencyCommand(&self) -> Option<Rc<DependencyCommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CommandContextAttrs<'input> for CommandContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn command(&mut self,)
	-> Result<Rc<CommandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_command);
        let mut _localctx: Rc<CommandContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(40);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule messageCommand*/
					recog.base.set_state(38);
					recog.messageCommand()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule dependencyCommand*/
					recog.base.set_state(39);
					recog.dependencyCommand()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- messageCommand ----------------
pub type MessageCommandContextAll<'input> = MessageCommandContext<'input>;


pub type MessageCommandContext<'input> = BaseParserRuleContext<'input,MessageCommandContextExt<'input>>;

#[derive(Clone)]
pub struct MessageCommandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for MessageCommandContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for MessageCommandContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_messageCommand(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_messageCommand(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for MessageCommandContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_messageCommand(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageCommandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_messageCommand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_messageCommand }
}
antlr_rust::type_id!{MessageCommandContextExt<'a>}

impl<'input> MessageCommandContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageCommandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageCommandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageCommandContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<MessageCommandContextExt<'input>>{

fn text_all(&self) ->  Vec<Rc<TextContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn text(&self, i: usize) -> Option<Rc<TextContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn arrow(&self) -> Option<Rc<ArrowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}
fn dependencyID(&self) -> Option<Rc<DependencyIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MessageCommandContextAttrs<'input> for MessageCommandContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn messageCommand(&mut self,)
	-> Result<Rc<MessageCommandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageCommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_messageCommand);
        let mut _localctx: Rc<MessageCommandContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(43);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule dependencyID*/
					recog.base.set_state(42);
					recog.dependencyID()?;

					}
				}

				_ => {}
			}
			/*InvokeRule text*/
			recog.base.set_state(45);
			recog.text()?;

			/*InvokeRule arrow*/
			recog.base.set_state(46);
			recog.arrow()?;

			/*InvokeRule text*/
			recog.base.set_state(47);
			recog.text()?;

			recog.base.set_state(48);
			recog.base.match_token(Comma,&mut recog.err_handler)?;

			/*InvokeRule text*/
			recog.base.set_state(49);
			recog.text()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- dependencyCommand ----------------
pub type DependencyCommandContextAll<'input> = DependencyCommandContext<'input>;


pub type DependencyCommandContext<'input> = BaseParserRuleContext<'input,DependencyCommandContextExt<'input>>;

#[derive(Clone)]
pub struct DependencyCommandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for DependencyCommandContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for DependencyCommandContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dependencyCommand(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_dependencyCommand(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for DependencyCommandContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_dependencyCommand(self);
	}
}

impl<'input> CustomRuleContext<'input> for DependencyCommandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependencyCommand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependencyCommand }
}
antlr_rust::type_id!{DependencyCommandContextExt<'a>}

impl<'input> DependencyCommandContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DependencyCommandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependencyCommandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DependencyCommandContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<DependencyCommandContextExt<'input>>{

fn text(&self) -> Option<Rc<TextContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Comma, 0)
}
fn dependencies(&self) -> Option<Rc<DependenciesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DependencyCommandContextAttrs<'input> for DependencyCommandContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dependencyCommand(&mut self,)
	-> Result<Rc<DependencyCommandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependencyCommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_dependencyCommand);
        let mut _localctx: Rc<DependencyCommandContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule text*/
			recog.base.set_state(51);
			recog.text()?;

			recog.base.set_state(52);
			recog.base.match_token(Comma,&mut recog.err_handler)?;

			/*InvokeRule dependencies*/
			recog.base.set_state(53);
			recog.dependencies()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- dependency ----------------
pub type DependencyContextAll<'input> = DependencyContext<'input>;


pub type DependencyContext<'input> = BaseParserRuleContext<'input,DependencyContextExt<'input>>;

#[derive(Clone)]
pub struct DependencyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for DependencyContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for DependencyContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dependency(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_dependency(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for DependencyContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_dependency(self);
	}
}

impl<'input> CustomRuleContext<'input> for DependencyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependency }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependency }
}
antlr_rust::type_id!{DependencyContextExt<'a>}

impl<'input> DependencyContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DependencyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependencyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DependencyContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<DependencyContextExt<'input>>{

fn number_all(&self) ->  Vec<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn number(&self, i: usize) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn comparison_all(&self) ->  Vec<Rc<ComparisonContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comparison(&self, i: usize) -> Option<Rc<ComparisonContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DependencyContextAttrs<'input> for DependencyContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dependency(&mut self,)
	-> Result<Rc<DependencyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependencyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_dependency);
        let mut _localctx: Rc<DependencyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule number*/
			recog.base.set_state(55);
			recog.number()?;

			recog.base.set_state(59); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule comparison*/
				recog.base.set_state(56);
				recog.comparison()?;

				/*InvokeRule number*/
				recog.base.set_state(57);
				recog.number()?;

				}
				}
				recog.base.set_state(61); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==Less || _la==More) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- dependencyID ----------------
pub type DependencyIDContextAll<'input> = DependencyIDContext<'input>;


pub type DependencyIDContext<'input> = BaseParserRuleContext<'input,DependencyIDContextExt<'input>>;

#[derive(Clone)]
pub struct DependencyIDContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for DependencyIDContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for DependencyIDContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dependencyID(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_dependencyID(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for DependencyIDContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_dependencyID(self);
	}
}

impl<'input> CustomRuleContext<'input> for DependencyIDContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependencyID }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependencyID }
}
antlr_rust::type_id!{DependencyIDContextExt<'a>}

impl<'input> DependencyIDContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DependencyIDContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependencyIDContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DependencyIDContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<DependencyIDContextExt<'input>>{

fn number(&self) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DependencyIDContextAttrs<'input> for DependencyIDContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dependencyID(&mut self,)
	-> Result<Rc<DependencyIDContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependencyIDContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_dependencyID);
        let mut _localctx: Rc<DependencyIDContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule number*/
			recog.base.set_state(63);
			recog.number()?;

			recog.base.set_state(64);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- dependencies ----------------
pub type DependenciesContextAll<'input> = DependenciesContext<'input>;


pub type DependenciesContext<'input> = BaseParserRuleContext<'input,DependenciesContextExt<'input>>;

#[derive(Clone)]
pub struct DependenciesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for DependenciesContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for DependenciesContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dependencies(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_dependencies(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for DependenciesContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_dependencies(self);
	}
}

impl<'input> CustomRuleContext<'input> for DependenciesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependencies }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependencies }
}
antlr_rust::type_id!{DependenciesContextExt<'a>}

impl<'input> DependenciesContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DependenciesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependenciesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DependenciesContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<DependenciesContextExt<'input>>{

fn dependency_all(&self) ->  Vec<Rc<DependencyContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dependency(&self, i: usize) -> Option<Rc<DependencyContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Colon in current rule
fn Colon_all(&self) -> Vec<Rc<TerminalNode<'input,sequenceParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Colon, starting from 0.
/// Returns `None` if number of children corresponding to token Colon is less or equal than `i`.
fn Colon(&self, i: usize) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Colon, i)
}

}

impl<'input> DependenciesContextAttrs<'input> for DependenciesContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dependencies(&mut self,)
	-> Result<Rc<DependenciesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependenciesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_dependencies);
        let mut _localctx: Rc<DependenciesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(74);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Number || _la==Space {
				{
				/*InvokeRule dependency*/
				recog.base.set_state(66);
				recog.dependency()?;

				recog.base.set_state(71);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==Colon {
					{
					{
					recog.base.set_state(67);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					/*InvokeRule dependency*/
					recog.base.set_state(68);
					recog.dependency()?;

					}
					}
					recog.base.set_state(73);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- text ----------------
pub type TextContextAll<'input> = TextContext<'input>;


pub type TextContext<'input> = BaseParserRuleContext<'input,TextContextExt<'input>>;

#[derive(Clone)]
pub struct TextContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for TextContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for TextContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_text(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_text(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for TextContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_text(self);
	}
}

impl<'input> CustomRuleContext<'input> for TextContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_text }
	//fn type_rule_index() -> usize where Self: Sized { RULE_text }
}
antlr_rust::type_id!{TextContextExt<'a>}

impl<'input> TextContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TextContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TextContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TextContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<TextContextExt<'input>>{

fn textInternal(&self) -> Option<Rc<TextInternalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token Space in current rule
fn Space_all(&self) -> Vec<Rc<TerminalNode<'input,sequenceParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Space, starting from 0.
/// Returns `None` if number of children corresponding to token Space is less or equal than `i`.
fn Space(&self, i: usize) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Space, i)
}

}

impl<'input> TextContextAttrs<'input> for TextContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn text(&mut self,)
	-> Result<Rc<TextContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TextContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_text);
        let mut _localctx: Rc<TextContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(79);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Space {
				{
				{
				recog.base.set_state(76);
				recog.base.match_token(Space,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(81);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule textInternal*/
			recog.base.set_state(82);
			recog.textInternal()?;

			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Space {
				{
				{
				recog.base.set_state(83);
				recog.base.match_token(Space,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(88);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- textInternal ----------------
pub type TextInternalContextAll<'input> = TextInternalContext<'input>;


pub type TextInternalContext<'input> = BaseParserRuleContext<'input,TextInternalContextExt<'input>>;

#[derive(Clone)]
pub struct TextInternalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for TextInternalContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for TextInternalContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_textInternal(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_textInternal(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for TextInternalContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_textInternal(self);
	}
}

impl<'input> CustomRuleContext<'input> for TextInternalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_textInternal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_textInternal }
}
antlr_rust::type_id!{TextInternalContextExt<'a>}

impl<'input> TextInternalContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TextInternalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TextInternalContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TextInternalContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<TextInternalContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Space in current rule
fn Space_all(&self) -> Vec<Rc<TerminalNode<'input,sequenceParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Space, starting from 0.
/// Returns `None` if number of children corresponding to token Space is less or equal than `i`.
fn Space(&self, i: usize) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Space, i)
}
/// Retrieves all `TerminalNode`s corresponding to token EOL in current rule
fn EOL_all(&self) -> Vec<Rc<TerminalNode<'input,sequenceParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EOL, starting from 0.
/// Returns `None` if number of children corresponding to token EOL is less or equal than `i`.
fn EOL(&self, i: usize) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(EOL, i)
}

}

impl<'input> TextInternalContextAttrs<'input> for TextInternalContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn textInternal(&mut self,)
	-> Result<Rc<TextInternalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TextInternalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_textInternal);
        let mut _localctx: Rc<TextInternalContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(89);
			_la = recog.base.input.la(1);
			if { _la <= 0 || (_la==EOL || _la==Space) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(97);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(93);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(90);
							_la = recog.base.input.la(1);
							if { _la <= 0 || (_la==EOL) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							}
							} 
						}
						recog.base.set_state(95);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
					}
					recog.base.set_state(96);
					_la = recog.base.input.la(1);
					if { _la <= 0 || (_la==EOL || _la==Space) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- number ----------------
pub type NumberContextAll<'input> = NumberContext<'input>;


pub type NumberContext<'input> = BaseParserRuleContext<'input,NumberContextExt<'input>>;

#[derive(Clone)]
pub struct NumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for NumberContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_number(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_number(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for NumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_number(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr_rust::type_id!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<NumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Space in current rule
fn Space_all(&self) -> Vec<Rc<TerminalNode<'input,sequenceParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Space, starting from 0.
/// Returns `None` if number of children corresponding to token Space is less or equal than `i`.
fn Space(&self, i: usize) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Space, i)
}

}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn number(&mut self,)
	-> Result<Rc<NumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(102);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Space {
				{
				{
				recog.base.set_state(99);
				recog.base.match_token(Space,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(104);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(105);
			recog.base.match_token(Number,&mut recog.err_handler)?;

			recog.base.set_state(109);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Space {
				{
				{
				recog.base.set_state(106);
				recog.base.match_token(Space,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(111);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- comparison ----------------
pub type ComparisonContextAll<'input> = ComparisonContext<'input>;


pub type ComparisonContext<'input> = BaseParserRuleContext<'input,ComparisonContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for ComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for ComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_comparison(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_comparison(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for ComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_comparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparison }
}
antlr_rust::type_id!{ComparisonContextExt<'a>}

impl<'input> ComparisonContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<ComparisonContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Less
/// Returns `None` if there is no child corresponding to token Less
fn Less(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(Less, 0)
}
/// Retrieves first TerminalNode corresponding to token More
/// Returns `None` if there is no child corresponding to token More
fn More(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(More, 0)
}

}

impl<'input> ComparisonContextAttrs<'input> for ComparisonContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparison(&mut self,)
	-> Result<Rc<ComparisonContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_comparison);
        let mut _localctx: Rc<ComparisonContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(112);
			_la = recog.base.input.la(1);
			if { !(_la==Less || _la==More) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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


pub type ArrowContext<'input> = BaseParserRuleContext<'input,ArrowContextExt<'input>>;

#[derive(Clone)]
pub struct ArrowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> sequenceParserContext<'input> for ArrowContext<'input>{}

impl<'input,'a> Listenable<dyn sequenceListener<'input> + 'a> for ArrowContext<'input>{
	fn enter(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_arrow(self);
	}
	fn exit(&self,listener: &mut (dyn sequenceListener<'input> + 'a)) {
		listener.exit_arrow(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn sequenceVisitor<'input> + 'a> for ArrowContext<'input>{
	fn accept(&self,visitor: &mut (dyn sequenceVisitor<'input> + 'a)) {
		visitor.visit_arrow(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = sequenceParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrow }
}
antlr_rust::type_id!{ArrowContextExt<'a>}

impl<'input> ArrowContextExt<'input>{
	fn new(parent: Option<Rc<dyn sequenceParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrowContextAttrs<'input>: sequenceParserContext<'input> + BorrowMut<ArrowContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NormalLeftArrow
/// Returns `None` if there is no child corresponding to token NormalLeftArrow
fn NormalLeftArrow(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(NormalLeftArrow, 0)
}
/// Retrieves first TerminalNode corresponding to token NormalRightArrow
/// Returns `None` if there is no child corresponding to token NormalRightArrow
fn NormalRightArrow(&self) -> Option<Rc<TerminalNode<'input,sequenceParserContextType>>> where Self:Sized{
	self.get_token(NormalRightArrow, 0)
}

}

impl<'input> ArrowContextAttrs<'input> for ArrowContext<'input>{}

impl<'input, I, H> sequenceParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrow(&mut self,)
	-> Result<Rc<ArrowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_arrow);
        let mut _localctx: Rc<ArrowContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(114);
			_la = recog.base.input.la(1);
			if { !(_la==NormalRightArrow || _la==NormalLeftArrow) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x0d\x77\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x03\x02\x05\x02\
	\x1c\x0a\x02\x03\x02\x03\x02\x05\x02\x20\x0a\x02\x07\x02\x22\x0a\x02\x0c\
	\x02\x0e\x02\x25\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x05\x03\x2b\x0a\
	\x03\x03\x04\x05\x04\x2e\x0a\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\
	\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\
	\x06\x06\x3e\x0a\x06\x0d\x06\x0e\x06\x3f\x03\x07\x03\x07\x03\x07\x03\x08\
	\x03\x08\x03\x08\x07\x08\x48\x0a\x08\x0c\x08\x0e\x08\x4b\x0b\x08\x05\x08\
	\x4d\x0a\x08\x03\x09\x07\x09\x50\x0a\x09\x0c\x09\x0e\x09\x53\x0b\x09\x03\
	\x09\x03\x09\x07\x09\x57\x0a\x09\x0c\x09\x0e\x09\x5a\x0b\x09\x03\x0a\x03\
	\x0a\x07\x0a\x5e\x0a\x0a\x0c\x0a\x0e\x0a\x61\x0b\x0a\x03\x0a\x05\x0a\x64\
	\x0a\x0a\x03\x0b\x07\x0b\x67\x0a\x0b\x0c\x0b\x0e\x0b\x6a\x0b\x0b\x03\x0b\
	\x03\x0b\x07\x0b\x6e\x0a\x0b\x0c\x0b\x0e\x0b\x71\x0b\x0b\x03\x0c\x03\x0c\
	\x03\x0d\x03\x0d\x03\x0d\x02\x02\x0e\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\
	\x14\x16\x18\x02\x06\x04\x02\x0a\x0a\x0c\x0c\x03\x02\x0a\x0a\x03\x02\x07\
	\x08\x03\x02\x04\x05\x02\x78\x02\x1b\x03\x02\x02\x02\x04\x2a\x03\x02\x02\
	\x02\x06\x2d\x03\x02\x02\x02\x08\x35\x03\x02\x02\x02\x0a\x39\x03\x02\x02\
	\x02\x0c\x41\x03\x02\x02\x02\x0e\x4c\x03\x02\x02\x02\x10\x51\x03\x02\x02\
	\x02\x12\x5b\x03\x02\x02\x02\x14\x68\x03\x02\x02\x02\x16\x72\x03\x02\x02\
	\x02\x18\x74\x03\x02\x02\x02\x1a\x1c\x05\x04\x03\x02\x1b\x1a\x03\x02\x02\
	\x02\x1b\x1c\x03\x02\x02\x02\x1c\x23\x03\x02\x02\x02\x1d\x1f\x07\x0a\x02\
	\x02\x1e\x20\x05\x04\x03\x02\x1f\x1e\x03\x02\x02\x02\x1f\x20\x03\x02\x02\
	\x02\x20\x22\x03\x02\x02\x02\x21\x1d\x03\x02\x02\x02\x22\x25\x03\x02\x02\
	\x02\x23\x21\x03\x02\x02\x02\x23\x24\x03\x02\x02\x02\x24\x26\x03\x02\x02\
	\x02\x25\x23\x03\x02\x02\x02\x26\x27\x07\x02\x02\x03\x27\x03\x03\x02\x02\
	\x02\x28\x2b\x05\x06\x04\x02\x29\x2b\x05\x08\x05\x02\x2a\x28\x03\x02\x02\
	\x02\x2a\x29\x03\x02\x02\x02\x2b\x05\x03\x02\x02\x02\x2c\x2e\x05\x0c\x07\
	\x02\x2d\x2c\x03\x02\x02\x02\x2d\x2e\x03\x02\x02\x02\x2e\x2f\x03\x02\x02\
	\x02\x2f\x30\x05\x10\x09\x02\x30\x31\x05\x18\x0d\x02\x31\x32\x05\x10\x09\
	\x02\x32\x33\x07\x06\x02\x02\x33\x34\x05\x10\x09\x02\x34\x07\x03\x02\x02\
	\x02\x35\x36\x05\x10\x09\x02\x36\x37\x07\x06\x02\x02\x37\x38\x05\x0e\x08\
	\x02\x38\x09\x03\x02\x02\x02\x39\x3d\x05\x14\x0b\x02\x3a\x3b\x05\x16\x0c\
	\x02\x3b\x3c\x05\x14\x0b\x02\x3c\x3e\x03\x02\x02\x02\x3d\x3a\x03\x02\x02\
	\x02\x3e\x3f\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x3f\x40\x03\x02\x02\
	\x02\x40\x0b\x03\x02\x02\x02\x41\x42\x05\x14\x0b\x02\x42\x43\x07\x03\x02\
	\x02\x43\x0d\x03\x02\x02\x02\x44\x49\x05\x0a\x06\x02\x45\x46\x07\x09\x02\
	\x02\x46\x48\x05\x0a\x06\x02\x47\x45\x03\x02\x02\x02\x48\x4b\x03\x02\x02\
	\x02\x49\x47\x03\x02\x02\x02\x49\x4a\x03\x02\x02\x02\x4a\x4d\x03\x02\x02\
	\x02\x4b\x49\x03\x02\x02\x02\x4c\x44\x03\x02\x02\x02\x4c\x4d\x03\x02\x02\
	\x02\x4d\x0f\x03\x02\x02\x02\x4e\x50\x07\x0c\x02\x02\x4f\x4e\x03\x02\x02\
	\x02\x50\x53\x03\x02\x02\x02\x51\x4f\x03\x02\x02\x02\x51\x52\x03\x02\x02\
	\x02\x52\x54\x03\x02\x02\x02\x53\x51\x03\x02\x02\x02\x54\x58\x05\x12\x0a\
	\x02\x55\x57\x07\x0c\x02\x02\x56\x55\x03\x02\x02\x02\x57\x5a\x03\x02\x02\
	\x02\x58\x56\x03\x02\x02\x02\x58\x59\x03\x02\x02\x02\x59\x11\x03\x02\x02\
	\x02\x5a\x58\x03\x02\x02\x02\x5b\x63\x0a\x02\x02\x02\x5c\x5e\x0a\x03\x02\
	\x02\x5d\x5c\x03\x02\x02\x02\x5e\x61\x03\x02\x02\x02\x5f\x5d\x03\x02\x02\
	\x02\x5f\x60\x03\x02\x02\x02\x60\x62\x03\x02\x02\x02\x61\x5f\x03\x02\x02\
	\x02\x62\x64\x0a\x02\x02\x02\x63\x5f\x03\x02\x02\x02\x63\x64\x03\x02\x02\
	\x02\x64\x13\x03\x02\x02\x02\x65\x67\x07\x0c\x02\x02\x66\x65\x03\x02\x02\
	\x02\x67\x6a\x03\x02\x02\x02\x68\x66\x03\x02\x02\x02\x68\x69\x03\x02\x02\
	\x02\x69\x6b\x03\x02\x02\x02\x6a\x68\x03\x02\x02\x02\x6b\x6f\x07\x0b\x02\
	\x02\x6c\x6e\x07\x0c\x02\x02\x6d\x6c\x03\x02\x02\x02\x6e\x71\x03\x02\x02\
	\x02\x6f\x6d\x03\x02\x02\x02\x6f\x70\x03\x02\x02\x02\x70\x15\x03\x02\x02\
	\x02\x71\x6f\x03\x02\x02\x02\x72\x73\x09\x04\x02\x02\x73\x17\x03\x02\x02\
	\x02\x74\x75\x09\x05\x02\x02\x75\x19\x03\x02\x02\x02\x10\x1b\x1f\x23\x2a\
	\x2d\x3f\x49\x4c\x51\x58\x5f\x63\x68\x6f";

