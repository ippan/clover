use crate::ast::token::TokenData;
use crate::ast::token::Token;

macro_rules! match_token {
    ($token: ident, $($key: expr => $value: expr), *) => {
        match $token {
        $(
            $key => Some($value),
        )*
            _ => None
        }
    }
}


pub struct Lexer {
    source: Vec<char>,
    line: u16,
    position: usize
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            line: 1,
            position: 0
        }
    }

    pub fn lex(&mut self) -> TokenData {
        while is_space(self.get_current_character()) {
            self.next_character();
        };

        if self.get_current_character() == '#' {
            self.skip_comment();
        }

        let character = self.get_current_character();

        if character == '\0' {
            return TokenData::new(Token::Eof, self.line);
        }

        if is_identifier(character) {
            return self.lex_identifier();
        }

        if is_string(character) {
            return self.lex_string();
        }

        if is_number(character) {
            return self.lex_number();
        }

        if is_symbol(character) {
            return self.lex_symbol();
        }

        TokenData::new(Token::Invalid(format!("unknown character [{}]", character)), self.line)
    }

    fn next_character(&mut self) -> char {
        let current_character = self.get_current_character();

        if current_character == '\0' {
            return current_character;
        }

        if current_character == '\n' {
            self.line += 1;
        }

        self.position += 1;

        self.get_current_character()
    }

    fn skip_comment(&mut self) {

        while self.get_current_character() != '\0' && self.get_current_character() != '\n' {
            self.next_character();
        }

        self.next_character();
    }

    fn lex_string(&mut self) -> TokenData {
        // TODO : parse string with special character

        let boundary = self.get_current_character();

        let mut value = String::new();

        self.next_character();

        while self.get_current_character() != '\0' && self.get_current_character() != boundary {
            value.push(self.get_current_character());
            self.next_character();
        };

        if self.get_current_character() == '\0' {
            return TokenData::new(Token::Invalid("end of file when parsing string".to_string()), self.line);
        };

        self.next_character();

        TokenData::new(Token::String(value), self.line)
    }

    fn lex_number(&mut self) -> TokenData {
        let mut number_string = String::new();
        let mut is_float = false;

        let mut character = self.get_current_character();

        while is_number(character) || character == '.' {

            if character == '.' {
                if is_float {
                    break;
                }

                if !is_number(self.peek_character()) {
                    break;
                }

                is_float = true
            }

            number_string.push(character);

            character = self.next_character();
        };

        if is_float {
            TokenData::new(Token::Float(number_string.parse().unwrap()), self.line)
        } else {
            TokenData::new(Token::Integer(number_string.parse().unwrap()), self.line)
        }

    }

    fn lex_identifier(&mut self) -> TokenData {
        let mut identifier = String::new();

        let mut character = self.get_current_character();

        loop {
            identifier.push(character);
            self.next_character();

            character = self.get_current_character();

            if !(is_identifier(character) || is_number(character)) {
                break;
            }
        };

        if let Some(keyword) = get_keyword(identifier.as_str()) {
            TokenData::new(keyword, self.line)
        } else {
            TokenData::new(Token::Identifier(identifier), self.line)
        }
    }

    fn lex_symbol(&mut self) -> TokenData {
        let symbol_string = String::from(self.get_current_character());

        self.next_character();

        if is_symbol(self.get_current_character()) {
            let mut multi_character_symbol_string = symbol_string.clone();
            multi_character_symbol_string.push(self.get_current_character());

            if let Some(symbol) = get_symbol(multi_character_symbol_string.as_str()) {
                self.next_character();
                return TokenData::new(symbol, self.line);
            };
        };

        TokenData::new(get_symbol(symbol_string.as_str()).unwrap(), self.line)
    }

    fn get_current_character(&self) -> char {
        match self.source.get(self.position) {
            Some(&character) => character,
            None => '\0'
        }
    }

    fn peek_character(&self) -> char {
        match self.source.get(self.position + 1) {
            Some(&character) => character,
            None => '\0'
        }
    }
}

// character helpers

fn is_space(character: char) -> bool {
    match character {
        ' ' | '\t' | '\r' | '\n' => true,
        _ => false
    }
}

fn is_string(character: char) -> bool {
    match character {
        '\'' | '"' => true,
        _ => false
    }
}

fn is_number(character: char) -> bool {
    match character {
        '0'..='9' => true,
        _ => false
    }
}

fn is_alpha(character: char) -> bool {
    match character {
        'a'..='z' | 'A'..='Z' => true,
        _ => false
    }
}

fn is_identifier(character: char) -> bool {
    is_alpha(character) || character == '_'
}

fn is_symbol(character: char) -> bool {
    let string = String::from(character);

    get_symbol(string.as_str()).is_some()
}

// token helpers

fn get_keyword(keyword: &str) -> Option<Token> {
    match_token! {
        keyword,
        "function" => Token::Function,
        "local"    => Token::Local,
        "New"      => Token::New,
        "end"      => Token::End,
        "if"       => Token::If,
        "else"     => Token::Else,
        "and"      => Token::And,
        "or"       => Token::Or,
        "not"      => Token::Not,
        "true"     => Token::True,
        "false"    => Token::False,
        "null"     => Token::Null,
        "class"    => Token::Class,
        "extends"  => Token::Extends,
        "return"   => Token::Return,
        "while"    => Token::While,
        "base"     => Token::Base,
        "this"     => Token::This,
        "load"     => Token::Load
    }
}

fn get_symbol(symbol: &str) -> Option<Token> {
    match_token! {
        symbol,
        "=" => Token::Assign,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Star,
        "/" => Token::Slash,
        "!" => Token::Not,
        "(" => Token::LeftParentheses,
        ")" => Token::RightParentheses,
        "[" => Token::LeftBracket,
        "]" => Token::RightBracket,
        "," => Token::Comma,
        ":" => Token::Colon,
        "&" => Token::BitAnd,
        "|" => Token::BitOr,
        "." => Token::Dot,
        "@" => Token::At,
        ">" => Token::Greater,
        "<" => Token::Less,

        "==" => Token::Equal,
        "!=" => Token::NotEqual,
        "&&" => Token::And,
        "||" => Token::Or,
        ">=" => Token::GreaterEqual,
        "<=" => Token::LessEqual,
        "+=" => Token::PlusAssign,
        "-=" => Token::MinusAssign,
        "*=" => Token::StarAssign,
        "/=" => Token::SlashAssign
    }
}