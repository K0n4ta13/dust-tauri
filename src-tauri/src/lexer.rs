use logos::{Lexer, Logos, Skip};
use serde::Serialize;

#[derive(Logos, Debug, PartialEq, Serialize)]
#[logos(skip r"[ \t\f]+", )]
#[logos(extras = (isize, isize, isize))]
pub enum Token {
    // Keywords
    #[token("let", word_callback)]
    Let,
    #[token("if", word_callback)]
    If,
    #[token("else if", word_callback)]
    ElseIf,
    #[token("else", word_callback)]
    Else,

    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", word_callback)]
    Ident,

    // Primitives
    #[regex(r"\d+", |lex| word_callback(lex); lex.slice().parse::<i64>().unwrap())]
    Integer(i64),
    #[regex(r"\d+\.\d*", |lex| word_callback(lex); lex.slice().parse::<f64>().unwrap())]
    Float(f64),
    #[token("false", |lex| word_callback(lex); false,)]
    #[token("true", |lex| word_callback(lex); true)]
    Boolean(bool),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| word_callback(lex); lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),

    // Operators
    #[token("=", word_callback)]
    Assign,
    #[token("+", word_callback)]
    Plus,
    #[token("-", word_callback)]
    Minus,
    #[token("*", word_callback)]
    Multiply,
    #[token("/", word_callback)]
    Divide,
    #[token("==", word_callback)]
    Equals,

    // Delimiters
    #[token(";", word_callback)]
    SemiColon,
    #[token("(", word_callback)]
    LParen,
    #[token(")", word_callback)]
    RParen,
    #[token("{", word_callback)]
    LBrace,
    #[token("}", word_callback)]
    RBrace,

    // Functions
    #[token("print", word_callback)]
    Print,
    #[token("input", word_callback)]
    Input,

    // Extras
    #[regex(r"\n", newline_callback)]
    Newline,

    // Error
    Error
}

fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.2 = lex.span().start as isize;
    Skip
}

fn word_callback(lex: &mut Lexer<Token>) {
    lex.extras.1 = lex.span().start as isize - lex.extras.2;
}

#[tauri::command]
pub fn tokens(source: &str) -> Vec<(Token, &str)> {
    Token::lexer(source).spanned()
        .map(|(token, span)| (token.unwrap_or(Token::Error), &source[span]))
        .collect()
}
