use std::collections::HashMap;
use std::io;
use std::string::String;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Tag {
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
    BoolOp,
    RelOp,
    Unary,
    NumOp,
    /* Identifiers, Types and Numbers. */
    Type,
    Ide,
    Num,
    //Real(String), /* Hash trait not implemented for float. */
}

#[derive(PartialEq)]
pub enum TokenInfo {
    NoInfo,

    Or,
    And,

    Ge, Gr,
    Leq, Les,
    Equ, Neq,

    Minus,
    Not,

    Add, Sub,
    Mul, Div,

    Int, Float,

    Num(u32),
    Ide(String),
}

#[derive(PartialEq)]
pub struct Token {
    pub tag : Tag,
    pub info : TokenInfo,
}

impl Token {
    pub fn new(tag : Tag, info : TokenInfo) -> Self {
        Token {
            tag : tag,
            info : info,
        }
    }
}

pub struct Scanner {
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
            buffer : s,
            read_from_stdin : true,
            lookahead : ' ',
        }
    }

    pub fn new_static(s : String) -> Scanner {
        println!("{}", s);
        Scanner {
            buffer : s,
            read_from_stdin : false,
            lookahead : ' ',
        }
    }

    /* Scan the input until it finds a token. */
    pub fn scan(&mut self) -> Token {
        match self.lookahead {
            '\0' => Token::new(Tag::Eof, TokenInfo::NoInfo),
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
            '+' => self.single_token(Token::new(Tag::NumOp, TokenInfo::Add)),
            '-' => self.single_token(Token::new(Tag::NumOp, TokenInfo::Sub)),
            '*' => self.single_token(Token::new(Tag::NumOp, TokenInfo::Mul)),
            '/' => self.single_token(Token::new(Tag::NumOp, TokenInfo::Div)),
            '(' => self.single_token(Token::new(Tag::LParen, TokenInfo::NoInfo)),
            ')' => self.single_token(Token::new(Tag::RParen, TokenInfo::NoInfo)),
            '[' => self.single_token(Token::new(Tag::LArrParen, TokenInfo::NoInfo)),
            ']' => self.single_token(Token::new(Tag::RArrParen, TokenInfo::NoInfo)),
            '{' => self.single_token(Token::new(Tag::OpenBlock, TokenInfo::NoInfo)),
            '}' => self.single_token(Token::new(Tag::CloseBlock, TokenInfo::NoInfo)),
            ';' => self.single_token(Token::new(Tag::SemiColon, TokenInfo::NoInfo)),
            _ => Token::new(Tag::Eof, TokenInfo::NoInfo), /* ERRORE!!! */
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
        Token::new(Tag::Num, TokenInfo::Num(v))
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
            Token::new(Tag::If, TokenInfo::NoInfo)
        } else if x == "else".as_bytes() {
            Token::new(Tag::Else, TokenInfo::NoInfo)
        } else if x == "while".as_bytes() {
            Token::new(Tag::While, TokenInfo::NoInfo)
        } else if x == "break".as_bytes() {
            Token::new(Tag::Break, TokenInfo::NoInfo)
        } else {
            Token::new(Tag::Ide, TokenInfo::Ide(s1))
        }
    }

    fn scan_relop(&mut self) -> Token {
        // '='|'!'|'<'|'>'
        let c = self.lookahead;
        self.lookahead = self.read_char();
        match (c, self.lookahead) {
            ('=', '=') => self.single_token(Token::new(Tag::RelOp, TokenInfo::Equ)),
            ('!', '=') => self.single_token(Token::new(Tag::RelOp, TokenInfo::Neq)),
            ('<', '=') => self.single_token(Token::new(Tag::RelOp, TokenInfo::Leq)),
            ('>', '=') => self.single_token(Token::new(Tag::RelOp, TokenInfo::Ge)),
            ('>', _)   => Token::new(Tag::RelOp, TokenInfo::Gr),
            ('<', _)   => Token::new(Tag::RelOp, TokenInfo::Les),
            ('!', _)   => Token::new(Tag::Unary, TokenInfo::Not),
            ('=', _)   => Token::new(Tag::Assign, TokenInfo::NoInfo),
            (_, _)     => panic!("scan_relop: this case should be impossible."),
        }
    }
    fn scan_boolop(&mut self) -> Token {
        let c = self.lookahead;
        self.lookahead = self.read_char();
        match (c, self.lookahead) {
            ('|', '|') => self.single_token(Token::new(Tag::BoolOp, TokenInfo::Or)),
            ('&', '&') => self.single_token(Token::new(Tag::BoolOp, TokenInfo::And)),
            _          => Token::new(Tag::Error, TokenInfo::NoInfo),
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
