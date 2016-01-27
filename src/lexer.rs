use std::collections::HashMap;
use std::io;
use std::io::{Read, Stdin};
use std::string::String;
use std::string;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Boolop {
    Or,
    And
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Relop {
    Ge, Gr,
    Leq, les,
    Equ, Neq,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Unary {
    Minus,
    Not,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Numop {
    Add, Sub,
    Mul, Div,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Eof, // Finished.
    /* Reserved words. */
    If,
    Else,
    While,
    Break,
    /* Separators. */
    CloseBlock,
    OpenBlock,
    SemiColon,
    LArrParen, RArrParen, LParen, RParen,
    Assign,
    /* Operators. */
    BoolOP(Boolop),
    RelOP(Relop),
    Unary(Unary),
    NumOP(Numop),
    /* Identifiers ad Numbers. */
    Ide(String),
    Num(i32),
    Real(String), /* Hash trait not implemented for float. */
}

pub struct Scanner {
    /* This HashMap should save some space.
        If we scan a token already seen, we don't allocate new memory.
        This way we don't end up we thousands of If and While tokens.
    */
    createdTokens : HashMap<Token, Token>,
    buffer : String,
    lookahead : char,
}

impl Scanner {
    pub fn new() -> Scanner {
        let mut s = String::new();
        io::stdin().read_line(&mut s);
        println!("{}", s);
        Scanner {
            createdTokens : HashMap::new(),
            buffer : s,
            lookahead : ' ',
        }
    }

    /* Scan the input until it finds a token. */
    pub fn scan(&mut self) -> Token {
        match self.lookahead {
            '\0' => Token::Eof,
            ' '|'\t'|'\n' => {
                /* Skip spaces. Read new character. */
                self.lookahead = self.read_char();
                self.scan()
            },
            '0' ... '9' => self.scan_number(),
            'a' ... 'z' |
            'A' ... 'Z' => self.scan_iden_keyword(),
            '='|'!'|'<'|'>' => self.scan_relop(),
            '|'|'&' => self.scan_boolop(),
            '+' => self.single_token(Token::NumOP(Numop::Add)),
            '-' => self.single_token(Token::NumOP(Numop::Sub)),
            '*' => self.single_token(Token::NumOP(Numop::Mul)),
            '/' => self.single_token(Token::NumOP(Numop::Div)),
            '(' => self.single_token(Token::LParen),
            ')' => self.single_token(Token::RParen),
            '[' => self.single_token(Token::LArrParen),
            ']' => self.single_token(Token::RArrParen),
            '{' => self.single_token(Token::OpenBlock),
            '}' => self.single_token(Token::CloseBlock),
            ';' => self.single_token(Token::SemiColon),
            _ => Token::Eof, /* ERRORE!!! */
        }
    }

    fn single_token(&mut self, tok : Token) -> Token {
        self.lookahead = ' ';
        tok
    }

    fn scan_number(&mut self) -> Token {
        let mut v = 0;
        while self.lookahead.is_digit(10) {
            v = v*10;
            v += self.lookahead.to_digit(10).unwrap();
            self.lookahead = self.read_char();
        }
        /* TODO: check for float. */
        Token::Num(0)
    }

    fn scan_iden_keyword(&mut self) -> Token {
        //let keywords = {"if"; "else"; "while"; "break"};
        let mut s = String::new();
        while self.lookahead.is_alphanumeric() {
            s.push(self.lookahead);
            self.lookahead = self.read_char();
        }
        /* TODO: check for keywords. */
        Token::Ide(s)
    }

    fn scan_relop(&mut self) -> Token {
        // '='|'!'|'<'|'>'
        self.lookahead = ' '; // match token.
        Token::LParen
    }
    fn scan_boolop(&mut self) -> Token {
        // '|'|'&'
        self.lookahead = ' '; // match token.
        Token::LParen
    }

    fn read_char(&mut self) -> char {
        if self.buffer.len() == 0 {
            '\0'
        } else {
            self.buffer.remove(0)
        }
    }
}
