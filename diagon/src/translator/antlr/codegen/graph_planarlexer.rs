// Generated from graph_planar.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{cast, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, EmptyContext, EmptyCustomRuleContext};
use antlr_rust::token::*;
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::{lazy_static, Tid, TidAble, TidExt};

use std::cell::RefCell;
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
pub const channelNames: [&'static str; 0 + 2] = ["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];

pub const modeNames: [&'static str; 1] = ["DEFAULT_MODE"];

pub const ruleNames: [&'static str; 11] = [
    "RIGHT_ARROW",
    "NONE_ARROW",
    "LEFT_RIGHT_ARROW",
    "LEFT_ARROW",
    "STRING",
    "ID",
    "LETTER",
    "EOL",
    "WS",
    "COMMENT",
    "LINE_COMMENT",
];

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

pub type LexerContext<'input> =
    BaseRuleContext<'input, EmptyCustomRuleContext<'input, LocalTokenFactory<'input>>>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

#[derive(Tid)]
pub struct graph_planarLexer<'input, Input: CharStream<From<'input>>> {
    base: BaseLexer<'input, graph_planarLexerActions, Input, LocalTokenFactory<'input>>,
}

impl<'input, Input: CharStream<From<'input>>> Deref for graph_planarLexer<'input, Input> {
    type Target = BaseLexer<'input, graph_planarLexerActions, Input, LocalTokenFactory<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> DerefMut for graph_planarLexer<'input, Input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> graph_planarLexer<'input, Input> {
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
        "graph_planarLexer.g4"
    }

    pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                graph_planarLexerActions {},
                tf,
            ),
        }
    }
}

impl<'input, Input: CharStream<From<'input>>> graph_planarLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(input: Input) -> Self {
        graph_planarLexer::new_with_token_factory(
            input,
            <&LocalTokenFactory<'input> as Default>::default(),
        )
    }
}

pub struct graph_planarLexerActions {}

impl graph_planarLexerActions {}

impl<'input, Input: CharStream<From<'input>>>
    Actions<'input, BaseLexer<'input, graph_planarLexerActions, Input, LocalTokenFactory<'input>>>
    for graph_planarLexerActions
{
}

impl<'input, Input: CharStream<From<'input>>> graph_planarLexer<'input, Input> {}

impl<'input, Input: CharStream<From<'input>>>
    LexerRecog<
        'input,
        BaseLexer<'input, graph_planarLexerActions, Input, LocalTokenFactory<'input>>,
    > for graph_planarLexerActions
{
}
impl<'input> TokenAware<'input> for graph_planarLexerActions {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input>
    for graph_planarLexer<'input, Input>
{
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
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x0c\x5f\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x03\x02\x03\x02\x03\x02\
		\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\
		\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x07\x06\x2b\x0a\x06\x0c\x06\x0e\
		\x06\x2e\x0b\x06\x03\x06\x03\x06\x03\x07\x06\x07\x33\x0a\x07\x0d\x07\x0e\
		\x07\x34\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x05\x09\x3c\x0a\x09\x03\
		\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x07\x0b\x46\
		\x0a\x0b\x0c\x0b\x0e\x0b\x49\x0b\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\
		\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\x54\x0a\x0c\x0c\x0c\x0e\x0c\
		\x57\x0b\x0c\x03\x0c\x05\x0c\x5a\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
		\x05\x2c\x47\x55\x02\x0d\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\
		\x0f\x02\x11\x09\x13\x0a\x15\x0b\x17\x0c\x03\x02\x04\x09\x02\x0b\x0c\x0f\
		\x0f\x22\x22\x24\x24\x2f\x2f\x3e\x3e\x40\x40\x04\x02\x0b\x0b\x22\x22\x02\
		\x64\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\
		\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\x02\
		\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\
		\x02\x02\x17\x03\x02\x02\x02\x03\x19\x03\x02\x02\x02\x05\x1c\x03\x02\x02\
		\x02\x07\x1f\x03\x02\x02\x02\x09\x23\x03\x02\x02\x02\x0b\x26\x03\x02\x02\
		\x02\x0d\x32\x03\x02\x02\x02\x0f\x36\x03\x02\x02\x02\x11\x3b\x03\x02\x02\
		\x02\x13\x3d\x03\x02\x02\x02\x15\x41\x03\x02\x02\x02\x17\x4f\x03\x02\x02\
		\x02\x19\x1a\x07\x2f\x02\x02\x1a\x1b\x07\x40\x02\x02\x1b\x04\x03\x02\x02\
		\x02\x1c\x1d\x07\x2f\x02\x02\x1d\x1e\x07\x2f\x02\x02\x1e\x06\x03\x02\x02\
		\x02\x1f\x20\x07\x3e\x02\x02\x20\x21\x07\x2f\x02\x02\x21\x22\x07\x40\x02\
		\x02\x22\x08\x03\x02\x02\x02\x23\x24\x07\x3e\x02\x02\x24\x25\x07\x2f\x02\
		\x02\x25\x0a\x03\x02\x02\x02\x26\x2c\x07\x24\x02\x02\x27\x28\x07\x5e\x02\
		\x02\x28\x2b\x07\x24\x02\x02\x29\x2b\x0b\x02\x02\x02\x2a\x27\x03\x02\x02\
		\x02\x2a\x29\x03\x02\x02\x02\x2b\x2e\x03\x02\x02\x02\x2c\x2d\x03\x02\x02\
		\x02\x2c\x2a\x03\x02\x02\x02\x2d\x2f\x03\x02\x02\x02\x2e\x2c\x03\x02\x02\
		\x02\x2f\x30\x07\x24\x02\x02\x30\x0c\x03\x02\x02\x02\x31\x33\x05\x0f\x08\
		\x02\x32\x31\x03\x02\x02\x02\x33\x34\x03\x02\x02\x02\x34\x32\x03\x02\x02\
		\x02\x34\x35\x03\x02\x02\x02\x35\x0e\x03\x02\x02\x02\x36\x37\x0a\x02\x02\
		\x02\x37\x10\x03\x02\x02\x02\x38\x39\x07\x0f\x02\x02\x39\x3c\x07\x0c\x02\
		\x02\x3a\x3c\x07\x0c\x02\x02\x3b\x38\x03\x02\x02\x02\x3b\x3a\x03\x02\x02\
		\x02\x3c\x12\x03\x02\x02\x02\x3d\x3e\x09\x03\x02\x02\x3e\x3f\x03\x02\x02\
		\x02\x3f\x40\x08\x0a\x02\x02\x40\x14\x03\x02\x02\x02\x41\x42\x07\x31\x02\
		\x02\x42\x43\x07\x2c\x02\x02\x43\x47\x03\x02\x02\x02\x44\x46\x0b\x02\x02\
		\x02\x45\x44\x03\x02\x02\x02\x46\x49\x03\x02\x02\x02\x47\x48\x03\x02\x02\
		\x02\x47\x45\x03\x02\x02\x02\x48\x4a\x03\x02\x02\x02\x49\x47\x03\x02\x02\
		\x02\x4a\x4b\x07\x2c\x02\x02\x4b\x4c\x07\x31\x02\x02\x4c\x4d\x03\x02\x02\
		\x02\x4d\x4e\x08\x0b\x02\x02\x4e\x16\x03\x02\x02\x02\x4f\x50\x07\x31\x02\
		\x02\x50\x51\x07\x31\x02\x02\x51\x55\x03\x02\x02\x02\x52\x54\x0b\x02\x02\
		\x02\x53\x52\x03\x02\x02\x02\x54\x57\x03\x02\x02\x02\x55\x56\x03\x02\x02\
		\x02\x55\x53\x03\x02\x02\x02\x56\x59\x03\x02\x02\x02\x57\x55\x03\x02\x02\
		\x02\x58\x5a\x07\x0f\x02\x02\x59\x58\x03\x02\x02\x02\x59\x5a\x03\x02\x02\
		\x02\x5a\x5b\x03\x02\x02\x02\x5b\x5c\x07\x0c\x02\x02\x5c\x5d\x03\x02\x02\
		\x02\x5d\x5e\x08\x0c\x02\x02\x5e\x18\x03\x02\x02\x02\x0a\x02\x2a\x2c\x34\
		\x3b\x47\x55\x59\x03\x08\x02\x02";
