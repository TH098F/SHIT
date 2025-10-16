#![allow(nonstandard_style)]

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Token {
    None,
    Identifier(String),
    RuleDelim,
    Dot,
    Comma,
    SemiColon,
    ParenOpen,
    ParenClose,
    Variable(String),
}

impl Token {
    fn fromChar(c: char) -> Self {
        match c {
            '.' => Self::Dot,
            ',' => Self::Comma,
            ';' => Self::SemiColon,
            '(' => Self::ParenOpen,
            ')' => Self::ParenClose,
            ':' => Self::RuleDelim,
            _ => panic!("PENIS")
        }
    }
}

#[derive(Debug)]
enum LexingError {
    InvalidText,
    UnexpectedSymbol,
    InvalidSyntax,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter, Debug)]
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

        for c in 'a'..='z' {
            map.insert((LexerState::ExpectStartOfVarOrIdentifier, c), LexerState::ExpectIdentifierDef);
        }

        for c in 'A'..='Z' {
            map.insert((LexerState::ExpectStartOfVarOrIdentifier, c), LexerState::ExpectVariable);
        }

        for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
            map.insert((LexerState::ExpectIdentifierDef, c), LexerState::ExpectIdentifierDef);
            map.insert((LexerState::ExpectVariable, c), LexerState::ExpectVariable);
            map.insert((LexerState::ExpectIdentifierUsage, c), LexerState::ExpectIdentifierUsage);
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

        for state in LexerState::iter() {
            if state == LexerState::Trash { continue; };
            for b in 0u8..=255 {
                let c = b as char;
                if !map.contains_key(&(state, c)) {
                    map.insert((state, c), LexerState::Trash);
                }
            }
        }

        Self {
            state: LexerState::ExpectIdentifierDef,
            stateMap: map
        }
    }

    fn lex(&mut self, text: &str) -> Result<Vec<Token>, LexingError> {
        let mut tokens = Vec::new();

        if !text.is_ascii() { return Err(LexingError::InvalidText); }

        let mut nameBuf = String::new();
        for c in text.chars() {
            let nextState = self.stateMap.get(&(self.state, c)).unwrap();

            println!("{:?} --{c}-> {nextState:?}", self.state);

            if *nextState == LexerState::Trash {
                return Err(LexingError::UnexpectedSymbol);
            }

            nameBuf.push(c);

            if *nextState != self.state {
                match match self.state {
                    LexerState::ExpectIdentifierDef => Some(Token::Identifier(nameBuf.clone())),
                    LexerState::ExpectIdentifierUsage => Some(Token::Identifier(nameBuf.clone())),
                    LexerState::ExpectOperator => Some(Token::fromChar(c)),
                    LexerState::ExpectRuleDelim => None,
                    LexerState::ExpectStartOfVarOrIdentifier => Some(Token::fromChar(c)),
                    LexerState::ExpectVariable => Some(Token::Variable(nameBuf.clone())),
                    _ => None
                } {
                    Some(token) => tokens.push(token),
                    None => ()
                }
            }

            self.state = *nextState;
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

    fn fromLiteral(text: &str) -> Self {
        Self { content: text.to_owned() }
    }
}

fn main() -> () {
    let mut lexer = LexLuthor::new();

    // let text = TranslationUnit::fromFile("test.p").unwrap();
    let text = TranslationUnit::fromLiteral("test(A):-haha");

    let tokens = lexer.lex(text.text()).unwrap();

    // println!("{tokens:?}");
    for t in tokens {
        println!("{t:?}");
    }

    ()
}