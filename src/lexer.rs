use std::collections::HashMap;
use std::io;

#[derive(PartialEq, Eq, Hash)]
enum Boolop {
    Or,
    And
}

#[derive(PartialEq, Eq, Hash)]
enum Relop {
    Ge, Gr,
    Leq, les,
    Equ, Neq,
}

#[derive(PartialEq, Eq, Hash)]
enum Unary {
    Minus,
    Not,
}

#[derive(PartialEq, Eq, Hash)]
enum Numop {
    Add, Sub,
    Mul, Div,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Token {
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
    lookahead : char,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            createdTokens : HashMap::new(),
            lookahead : ' ',
        }
    }

    /* Scan the input until it finds a token. */
    pub fn scan(&self) -> Token {
        match self.lookahead {
            ' '|'\t'|'\n' => {
                /* Skip spaces. Read new character. */
                self.lookahead = read_char();
                self.scan()
            },
            '0' ... '9' => self.scan_number(),
            'a' ... 'z' |
            'A' ... 'Z' => self.scan_iden_keyword(),
            '='|'!'|'<'|'>' => self.scan_relop(),
            '|'|'&' => self.scan_boolop(),
            '+' => Token::NumOP(Numop::Add),
            '-' => Token::NumOP(Numop::Sub),
            '*' => Token::NumOP(Numop::Mul),
            '/' => Token::NumOP(Numop::Div),
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LArrParen,
            ']' => Token::RArrParen,
            '{' => Token::OpenBlock,
            '}' => Token::CloseBlock,
            ';' => Token::SemiColon,
            _ => Token::If, /* ERRORE!!! */
        }
    }

    fn gen_token(&self, tok : Token) -> &Token {
        self.createdTokens.get(&tok)
            .unwrap_or(&tok)
    }

    fn scan_number(&self) -> Token {
        let v = 0;
        while self.lookahead.is_digit(10) {
            v = v*10;
            v += self.lookahead.to_digit(10).unwrap();
            self.lookahead = read_char();
        }
        /* TODO: check for float. */
        Token::Num(v)
    }

    fn scan_iden_keyword(&self) -> Token {
        let keywords = {"if"; "else"; "while"; "break"};
        let s = String::new();
        while self.lookahead.is_alphanumeric() {
            s.push(self.lookahead);
            self.lookahead = read_char();
        }
        /* TODO: check for keywords. */
        Token::Ide(s)
    }

    fn scan_relop(&self) -> Token {
        // '='|'!'|'<'|'>'
        Token::LParen
    }
    fn scan_boolop(&self) -> Token {
        // '|'|'&'
        Token::LParen
    }
}

fn read_char() -> char {
    'a'
}
