// Generated from math.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const STRING:isize=3; 
	pub const LPAREN:isize=4; 
	pub const RPAREN:isize=5; 
	pub const LBRACE:isize=6; 
	pub const RBRACE:isize=7; 
	pub const LBRACKET:isize=8; 
	pub const RBRACKET:isize=9; 
	pub const PLUS:isize=10; 
	pub const MINUS:isize=11; 
	pub const TIMES:isize=12; 
	pub const BANG:isize=13; 
	pub const DIV:isize=14; 
	pub const GT:isize=15; 
	pub const LT:isize=16; 
	pub const GE:isize=17; 
	pub const LE:isize=18; 
	pub const EQ:isize=19; 
	pub const POW:isize=20; 
	pub const SUBSCRIPT:isize=21; 
	pub const EOL:isize=22; 
	pub const WS:isize=23; 
	pub const VARIABLE:isize=24;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;25] = [
		"T__0", "T__1", "STRING", "LPAREN", "RPAREN", "LBRACE", "RBRACE", "LBRACKET", 
		"RBRACKET", "PLUS", "MINUS", "TIMES", "BANG", "DIV", "GT", "LT", "GE", 
		"LE", "EQ", "POW", "SUBSCRIPT", "EOL", "WS", "CHAR", "VARIABLE"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;22] = [
		None, Some("','"), Some("';'"), None, Some("'('"), Some("')'"), Some("'{'"), 
		Some("'}'"), Some("'['"), Some("']'"), Some("'+'"), Some("'-'"), Some("'*'"), 
		Some("'!'"), Some("'/'"), Some("'>'"), Some("'<'"), Some("'>='"), Some("'<='"), 
		Some("'='"), Some("'^'"), Some("'_'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;25]  = [
		None, None, None, Some("STRING"), Some("LPAREN"), Some("RPAREN"), Some("LBRACE"), 
		Some("RBRACE"), Some("LBRACKET"), Some("RBRACKET"), Some("PLUS"), Some("MINUS"), 
		Some("TIMES"), Some("BANG"), Some("DIV"), Some("GT"), Some("LT"), Some("GE"), 
		Some("LE"), Some("EQ"), Some("POW"), Some("SUBSCRIPT"), Some("EOL"), Some("WS"), 
		Some("VARIABLE")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

#[derive(Tid)]
pub struct mathLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>,
}

impl<'input, Input:CharStream<From<'input> >> Deref for mathLexer<'input,Input>{
	type Target = BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for mathLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> mathLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "mathLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","2");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				mathLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> mathLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		mathLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct mathLexerActions {
}

impl mathLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>> for mathLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> mathLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>> for mathLexerActions{
}
impl<'input> TokenAware<'input> for mathLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for mathLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x1a\x7d\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\
		\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\
		\x04\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\
		\x09\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x03\x02\x03\x02\
		\x03\x03\x03\x03\x03\x04\x03\x04\x07\x04\x3c\x0a\x04\x0c\x04\x0e\x04\x3f\
		\x0b\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\
		\x03\x08\x03\x08\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\
		\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\x03\x10\
		\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\x03\x14\
		\x03\x14\x03\x15\x03\x15\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x05\x17\
		\x6c\x0a\x17\x03\x18\x06\x18\x6f\x0a\x18\x0d\x18\x0e\x18\x70\x03\x18\x03\
		\x18\x03\x19\x03\x19\x05\x19\x77\x0a\x19\x03\x1a\x06\x1a\x7a\x0a\x1a\x0d\
		\x1a\x0e\x1a\x7b\x02\x02\x1b\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\
		\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\x1d\x10\x1f\
		\x11\x21\x12\x23\x13\x25\x14\x27\x15\x29\x16\x2b\x17\x2d\x18\x2f\x19\x31\
		\x02\x33\x1a\x03\x02\x06\x03\x02\x24\x24\x04\x02\x0b\x0b\x22\x22\x0a\x02\
		\x0b\x0c\x0f\x0f\x22\x24\x2a\x40\x5d\x5d\x5f\x61\x7d\x7d\x7f\x7f\x04\x02\
		\x30\x30\x32\x3b\x02\u{80}\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\
		\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\
		\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\
		\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\
		\x02\x02\x19\x03\x02\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\x03\x02\x02\
		\x02\x02\x1f\x03\x02\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\x03\x02\x02\
		\x02\x02\x25\x03\x02\x02\x02\x02\x27\x03\x02\x02\x02\x02\x29\x03\x02\x02\
		\x02\x02\x2b\x03\x02\x02\x02\x02\x2d\x03\x02\x02\x02\x02\x2f\x03\x02\x02\
		\x02\x02\x33\x03\x02\x02\x02\x03\x35\x03\x02\x02\x02\x05\x37\x03\x02\x02\
		\x02\x07\x39\x03\x02\x02\x02\x09\x42\x03\x02\x02\x02\x0b\x44\x03\x02\x02\
		\x02\x0d\x46\x03\x02\x02\x02\x0f\x48\x03\x02\x02\x02\x11\x4a\x03\x02\x02\
		\x02\x13\x4c\x03\x02\x02\x02\x15\x4e\x03\x02\x02\x02\x17\x50\x03\x02\x02\
		\x02\x19\x52\x03\x02\x02\x02\x1b\x54\x03\x02\x02\x02\x1d\x56\x03\x02\x02\
		\x02\x1f\x58\x03\x02\x02\x02\x21\x5a\x03\x02\x02\x02\x23\x5c\x03\x02\x02\
		\x02\x25\x5f\x03\x02\x02\x02\x27\x62\x03\x02\x02\x02\x29\x64\x03\x02\x02\
		\x02\x2b\x66\x03\x02\x02\x02\x2d\x6b\x03\x02\x02\x02\x2f\x6e\x03\x02\x02\
		\x02\x31\x76\x03\x02\x02\x02\x33\x79\x03\x02\x02\x02\x35\x36\x07\x2e\x02\
		\x02\x36\x04\x03\x02\x02\x02\x37\x38\x07\x3d\x02\x02\x38\x06\x03\x02\x02\
		\x02\x39\x3d\x07\x24\x02\x02\x3a\x3c\x0a\x02\x02\x02\x3b\x3a\x03\x02\x02\
		\x02\x3c\x3f\x03\x02\x02\x02\x3d\x3b\x03\x02\x02\x02\x3d\x3e\x03\x02\x02\
		\x02\x3e\x40\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x40\x41\x07\x24\x02\
		\x02\x41\x08\x03\x02\x02\x02\x42\x43\x07\x2a\x02\x02\x43\x0a\x03\x02\x02\
		\x02\x44\x45\x07\x2b\x02\x02\x45\x0c\x03\x02\x02\x02\x46\x47\x07\x7d\x02\
		\x02\x47\x0e\x03\x02\x02\x02\x48\x49\x07\x7f\x02\x02\x49\x10\x03\x02\x02\
		\x02\x4a\x4b\x07\x5d\x02\x02\x4b\x12\x03\x02\x02\x02\x4c\x4d\x07\x5f\x02\
		\x02\x4d\x14\x03\x02\x02\x02\x4e\x4f\x07\x2d\x02\x02\x4f\x16\x03\x02\x02\
		\x02\x50\x51\x07\x2f\x02\x02\x51\x18\x03\x02\x02\x02\x52\x53\x07\x2c\x02\
		\x02\x53\x1a\x03\x02\x02\x02\x54\x55\x07\x23\x02\x02\x55\x1c\x03\x02\x02\
		\x02\x56\x57\x07\x31\x02\x02\x57\x1e\x03\x02\x02\x02\x58\x59\x07\x40\x02\
		\x02\x59\x20\x03\x02\x02\x02\x5a\x5b\x07\x3e\x02\x02\x5b\x22\x03\x02\x02\
		\x02\x5c\x5d\x07\x40\x02\x02\x5d\x5e\x07\x3f\x02\x02\x5e\x24\x03\x02\x02\
		\x02\x5f\x60\x07\x3e\x02\x02\x60\x61\x07\x3f\x02\x02\x61\x26\x03\x02\x02\
		\x02\x62\x63\x07\x3f\x02\x02\x63\x28\x03\x02\x02\x02\x64\x65\x07\x60\x02\
		\x02\x65\x2a\x03\x02\x02\x02\x66\x67\x07\x61\x02\x02\x67\x2c\x03\x02\x02\
		\x02\x68\x69\x07\x0f\x02\x02\x69\x6c\x07\x0c\x02\x02\x6a\x6c\x07\x0c\x02\
		\x02\x6b\x68\x03\x02\x02\x02\x6b\x6a\x03\x02\x02\x02\x6c\x2e\x03\x02\x02\
		\x02\x6d\x6f\x09\x03\x02\x02\x6e\x6d\x03\x02\x02\x02\x6f\x70\x03\x02\x02\
		\x02\x70\x6e\x03\x02\x02\x02\x70\x71\x03\x02\x02\x02\x71\x72\x03\x02\x02\
		\x02\x72\x73\x08\x18\x02\x02\x73\x30\x03\x02\x02\x02\x74\x77\x0a\x04\x02\
		\x02\x75\x77\x09\x05\x02\x02\x76\x74\x03\x02\x02\x02\x76\x75\x03\x02\x02\
		\x02\x77\x32\x03\x02\x02\x02\x78\x7a\x05\x31\x19\x02\x79\x78\x03\x02\x02\
		\x02\x7a\x7b\x03\x02\x02\x02\x7b\x79\x03\x02\x02\x02\x7b\x7c\x03\x02\x02\
		\x02\x7c\x34\x03\x02\x02\x02\x08\x02\x3d\x6b\x70\x76\x7b\x03\x08\x02\x02";
