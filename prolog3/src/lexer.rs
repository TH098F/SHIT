#![allow(nonstandard_style)]
use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
    text: Vec<u8>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum LexerState {
    ExpectStartOfIdentifier,
    ExpectIdentifier,
    ExpectSymbol,
    Trash
}

pub struct Lexer {
    stateMap: HashMap<(LexerState, u8), LexerState>,
    state: LexerState,
    symbols: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct LexingError {
    
}

impl Lexer {
    pub fn new(symbolChars: &[u8], symbols: &[&[u8]]) -> Self {
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

        let mut _symbols = Vec::new();
        for sym in symbols {
            _symbols.push(sym.to_vec());
        }

        Self {
            stateMap: map,
            state: LexerState::ExpectStartOfIdentifier,
            symbols: _symbols
        }
    }

    pub fn lex(&mut self, text: &[u8]) -> Result<Vec<Token>, LexingError> {
        let mut tokens = Vec::new();

        let mut nameBuf: Vec<u8> = Vec::new();
        for c in text {
            if (*c as char).is_whitespace() {
                continue;
            }

            let nextState = match self.stateMap.get(&(self.state, *c)) {
                Some(x) => *x,
                None => LexerState::Trash,
            };

            if
                nextState != self.state
                && !(nextState == LexerState::ExpectIdentifier && self.state == LexerState::ExpectStartOfIdentifier)
            {
                if self.state == LexerState::ExpectSymbol {
                    // split symbol token
                    let mut i = 0;
                    while i < nameBuf.len() {
                        for sym in &self.symbols {
                            if i + sym.len() > nameBuf.len() { continue; }
                            if &nameBuf[i..(i + sym.len())] == &sym[..] {
                                tokens.push(Token{text: sym.clone()});
                                i += sym.len();
                                break;
                            }
                        }
                    }
                    nameBuf.clear();
                } else {
                    tokens.push(Token{text: nameBuf.clone()});
                    nameBuf.clear();
                }
            }

            nameBuf.push(*c);
            self.state = nextState;
        }

        if !nameBuf.is_empty() {
            tokens.push(Token{text: nameBuf});
        }

        return Ok(tokens);
    }
}