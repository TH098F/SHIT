#![allow(nonstandard_style)]

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    None,
    Identifier,
    RuleDelim,
    Dot,
    Comma,
    SemiColon,
    ParanOpen,
    ParanClose,
    Variable,
}

enum LexingError {
    InvalidText,
    UnexpectedSymbol,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum LexerState {
    Trash,
    ExpectStartOfVarOrIdentifier,
    ExpectIdentifierDef,
    ExpectIdentifierUsage,
    ExpectVariable,
    ExpectOperator,
    ExpectRuleDelim,
}

impl LexerState {
    fn new() -> Self {
        LexerState::Trash
    }
}

struct LexLuthor {
    stateMap: HashMap<(LexerState, char), LexerState>,
    state: LexerState,
}

impl LexLuthor {
    fn new() -> Self {
        let mut map = HashMap::new();

        for c in 'a'..'z' {
            map.insert((LexerState::ExpectStartOfVarOrIdentifier, c), LexerState::ExpectIdentifierDef);
        }

        for c in 'A'..'Z' {
            map.insert((LexerState::ExpectStartOfVarOrIdentifier, c), LexerState::ExpectVariable);
        }

        for c in ('a'..'z').chain('A'..'Z').chain('0'..'9') {
            map.insert((LexerState::ExpectIdentifierDef, c), LexerState::ExpectIdentifierDef);
            map.insert((LexerState::ExpectVariable, c), LexerState::ExpectVariable);
        }

        map.insert((LexerState::ExpectIdentifierDef, '('), LexerState::ExpectStartOfVarOrIdentifier);
        map.insert((LexerState::ExpectIdentifierUsage, '('), LexerState::ExpectStartOfVarOrIdentifier);

        map.insert((LexerState::ExpectVariable, ','), LexerState::ExpectStartOfVarOrIdentifier);
        map.insert((LexerState::ExpectIdentifierUsage, ','), LexerState::ExpectStartOfVarOrIdentifier);

        map.insert((LexerState::ExpectIdentifierUsage, ')'), LexerState::ExpectOperator);
        map.insert((LexerState::ExpectVariable, ')'), LexerState::ExpectOperator);

        for op in [',', ';', '!', '.'] {
            map.insert((LexerState::ExpectOperator, op), LexerState::ExpectIdentifierUsage);
        }

        map.insert((LexerState::ExpectOperator, ':'), LexerState::ExpectRuleDelim);
        map.insert((LexerState::ExpectRuleDelim, '-'), LexerState::ExpectIdentifierUsage);

        Self {
            state: LexerState::new(),
            stateMap: map
        }
    }

    fn lex(&self, text: &str) -> Result<Vec<Token>, LexingError> {
        let mut tokens = Vec::new();

        if !text.is_ascii() { return Err(LexingError::InvalidText); }

        let token = Token::None;
        for c in text.chars() {
            let nextState = self.stateMap.get(&(self.state, c)).unwrap();

            if *nextState == LexerState::Trash {
                return Err(LexingError::UnexpectedSymbol);
            }

            if *nextState != self.state {
                tokens.push(token);
            }
        }

        return Ok(tokens);
    }
}

struct TranslationUnit {
    content: String,
}

impl TranslationUnit {
    fn fromFile<PathType>(path: PathType) -> Option<Self>
    where
    PathType: AsRef<std::path::Path>
    {
        let mut file = fs::File::open(path).ok()?;
        let mut content = String::new();

        file.read_to_string(&mut content).ok()?;

        Some(Self {
            content: content
        })
    }

    fn text(&self) -> &str {
        &self.content
    }
}

fn main() -> () {
    let lexer = LexLuthor::new();

    let text = TranslationUnit::fromFile("test.p").unwrap();

    let tokens = lexer.lex(text.text());

    ()
}