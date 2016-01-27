use std::collections::HashMap;
use std::io;
use std::string::String;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Boolop {
    Or,
    And
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Relop {
    Ge, Gr,
    Leq, Les,
    Equ, Neq,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Unary {
    Minus,
    Not,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Numop {
    Add, Sub,
    Mul, Div,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Eof, // Finished.
    Error, // Parsing error.
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
    Num(u32),
    //Real(String), /* Hash trait not implemented for float. */
}

pub struct Scanner {
    /* This HashMap should save some space.
        If we scan a token already seen, we don't allocate new memory.
        This way we don't end up we thousands of If and While tokens.
    */
    createdTokens : HashMap<Token, Token>,
    buffer : String,
    read_from_stdin : bool,
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
            read_from_stdin : true,
            lookahead : ' ',
        }
    }

    pub fn new_static(s : String) -> Scanner {
        println!("{}", s);
        Scanner {
            createdTokens : HashMap::new(),
            buffer : s,
            read_from_stdin : true,
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
        Token::Num(v)
    }

    //TODO: really broken too much copies, inefficient.
    fn scan_iden_keyword(&mut self) -> Token {
        //let keywords = {"if"; "else"; "while"; "break"};
        let mut s = String::new();
        let mut s1 = String::new();
        while self.lookahead.is_alphanumeric() {
            s.push(self.lookahead);
            s1.push(self.lookahead);
            self.lookahead = self.read_char();
        }
        /* TODO: check for keywords. */
        let x = s.as_bytes();
        if x == "If".as_bytes() {
            Token::If
        } else if x == "else".as_bytes() {
            Token::Else
        } else if x == "while".as_bytes() {
            Token::While
        } else if x == "break".as_bytes() {
            Token::Break
        } else {
            Token::Ide(s1)
        }
    }

    fn scan_relop(&mut self) -> Token {
        // '='|'!'|'<'|'>'
        let c = self.lookahead;
        self.lookahead = self.read_char();
        match (c, self.lookahead) {
            ('=', '=') => self.single_token(Token::RelOP(Relop::Equ)),
            ('!', '=') => self.single_token(Token::RelOP(Relop::Neq)),
            ('<', '=') => self.single_token(Token::RelOP(Relop::Leq)),
            ('>', '=') => self.single_token(Token::RelOP(Relop::Ge)),
            ('>', _)   => Token::RelOP(Relop::Gr),
            ('<', _)   => Token::RelOP(Relop::Les),
            ('!', _)   => Token::Unary(Unary::Not),
            ('=', _)   => Token::Assign,
            (_, _)     => panic!("scan_relop: this case should be impossible."),
        }
    }
    fn scan_boolop(&mut self) -> Token {
        let c = self.lookahead;
        self.lookahead = self.read_char();
        match (c, self.lookahead) {
            ('|', '|') => self.single_token(Token::BoolOP(Boolop::Or)),
            ('&', '&') => self.single_token(Token::BoolOP(Boolop::And)),
            _          => Token::Error,
        }
    }

    fn read_char(&mut self) -> char {
        if self.buffer.len() == 0 {
            '\0'
        } else {
            self.buffer.remove(0)
        }
    }
}
