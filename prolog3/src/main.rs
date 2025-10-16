#![allow(nonstandard_style)]
use std::{collections::HashMap, fs, io::prelude::*};

#[derive(Debug)]
struct Token {
    text: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum LexerState {
    ExpectStartOfIdentifier,
    ExpectIdentifier,
    ExpectSymbol,
    Trash
}

struct Lexer {
    stateMap: HashMap<(LexerState, u8), LexerState>,
    state: LexerState,
}

#[derive(Debug)]
struct LexingError {
    
}

impl Lexer {
    fn new(symbolChars: &[u8]) -> Self {
        let mut map = HashMap::new();

        for c in (('a' as u8)..=('z' as u8)).chain(('A' as u8)..=('Z' as u8)) {
            map.insert((LexerState::ExpectStartOfIdentifier, c), LexerState::ExpectIdentifier);
            map.insert((LexerState::ExpectSymbol, c), LexerState::ExpectIdentifier);
        }

        for c in (('a' as u8)..=('z' as u8)).chain(('A' as u8)..=('Z' as u8)).chain(('0' as u8)..=('9' as u8)) {
            map.insert((LexerState::ExpectIdentifier, c), LexerState::ExpectIdentifier);
        }

        for c in symbolChars {
            map.insert((LexerState::ExpectIdentifier, *c), LexerState::ExpectSymbol);
            map.insert((LexerState::ExpectSymbol, *c), LexerState::ExpectSymbol);
        }

        Self {
            stateMap: map,
            state: LexerState::ExpectStartOfIdentifier
        }
    }

    fn lex(&mut self, text: &str) -> Result<Vec<Token>, LexingError> {
        let mut tokens = Vec::new();

        let mut nameBuf = Vec::new();
        for c in text.bytes() {
            let nextState = match self.stateMap.get(&(self.state, c)) {
                Some(x) => *x,
                None => LexerState::Trash,
            };

            if
                nextState != self.state
                && !(nextState == LexerState::ExpectIdentifier && self.state == LexerState::ExpectStartOfIdentifier)
            {
                if nextState == LexerState::ExpectSymbol {
                    //TODO: symbol matching
                } else {
                    tokens.push(Token{text: match String::from_utf8(nameBuf.clone()) {
                        Ok(x) => x,
                        Err(_) => return Err(LexingError{})
                    }});
                    nameBuf.clear();
                }
            }
            nameBuf.push(c);

            self.state = nextState;
        }

        if !nameBuf.is_empty() {
            tokens.push(Token{text: match String::from_utf8(nameBuf) {
                Ok(x) => x,
                Err(_) => return Err(LexingError{}),
            }});
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
    let mut lexer = Lexer::new(b",.;:()-");

    // let text = TranslationUnit::fromFile("test.p").unwrap();
    let text = TranslationUnit::fromLiteral("test(A):-haha");

    let tokens = lexer.lex(text.text()).unwrap();

    // println!("{tokens:?}");
    for t in tokens {
        println!("{t:?}");
    }

    ()
}
