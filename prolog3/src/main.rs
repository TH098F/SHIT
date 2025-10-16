#![allow(nonstandard_style)]

mod lexer;
use lexer::Lexer;

mod translationUnit;
use translationUnit::TranslationUnit;

fn main() -> () {
    let mut lexer = Lexer::new(b",.;:()-", &[b"(", b")", b":-", b",", b".", b";"]);

    // let text = TranslationUnit::fromFile("test.p").unwrap();
    let text = TranslationUnit::fromLiteral(b"test(A) :- haha(A, penis)");

    let tokens = lexer.lex(text.text()).unwrap();

    ()
}
