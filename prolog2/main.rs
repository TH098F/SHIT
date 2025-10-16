#![allow(nonstandard_style)]

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Token {
    None,
    Identifier(String),
    RuleDelim,
    Dot,
    Comma,
    SemiColon,
    ParanOpen,
    ParanClose,
    Variable(String),
}

impl Token {
    fn fromChar(c: char) -> Self {
        match c {
            '.' => Self::Dot,
            ',' => Self::Comma,
            ';' => Self::SemiColon,
            '(' => Self::ParanOpen,
            ')' => Self::ParanClose,
            ':' => Self::RuleDelim,
            _ => Self::None
        }
    }
}

#[derive(Debug)]
enum LexingError {
    InvalidText,
    UnexpectedSymbol,
    InvalidSyntax,
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

        let mut nameBuf = String::new();
        for c in text.chars() {
            let nextState = self.stateMap.get(&(self.state, c)).unwrap();

            if *nextState == LexerState::Trash {
                return Err(LexingError::UnexpectedSymbol);
            }

            nameBuf.push(c);

            if *nextState != self.state {
                let token = match self.state {
                    LexerState::ExpectStartOfVarOrIdentifier => None,
                    LexerState::ExpectIdentifierDef => Some(Token::Identifier(nameBuf.to_owned())),
                    LexerState::ExpectIdentifierUsage => Some(Token::Identifier(nameBuf.to_owned())),
                    LexerState::ExpectVariable => Some(Token::Variable(nameBuf.to_owned())),
                    LexerState::ExpectOperator => Some(Token::fromChar(nameBuf.chars().nth(0).unwrap())),
                    LexerState::ExpectRuleDelim => None,
                    LexerState::Trash => return Err(LexingError::InvalidSyntax)
                };

                if let Some(tok) = token { tokens.push(tok); }
                nameBuf.clear();
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

    fn fromLiteral(text: &str) -> Self {
        Self { content: text.to_owned() }
    }
}

fn main() -> () {
    let lexer = LexLuthor::new();

    // let text = TranslationUnit::fromFile("test.p").unwrap();
    let text = TranslationUnit::fromLiteral("test(A):-haha");

    let tokens = lexer.lex(text.text()).unwrap();

    for t in tokens {
        println!("{t:?}");
    }

    ()
}