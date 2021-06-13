use crate::intermediate::{ Token, TokenValue, Position, TokenList, CompileErrorList };
use std::iter::Peekable;
use std::str::Chars;

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

struct LexState<'a> {
    source: Peekable<Chars<'a>>,
    position: Position,
    current: char
}

impl<'a> LexState<'a> {
    fn skip_spaces_and_comments(&mut self) {
        while is_space(self.current) || is_comment_prefix(self.current) {
            while is_space(self.current) {
                self.next_character();
            };

            if is_comment_prefix(self.current) {
                self.skip_comment();
            };
        };
    }

    fn skip_comment(&mut self) {
        while self.next_character().is_some() && self.current != '\n' {};
    }

    fn next_character(&mut self) -> Option<char> {
        if let Some(character) = self.source.next() {

            self.current = character;
            self.position.column += 1;

            if character == '\n' {
                self.position.line += 1;
                self.position.column = 0;
            };

            return Some(character);
        };

        self.current = '\0';
        None
    }

    fn lex_string(&mut self) -> Token {
        let position = self.position;
        let mut value = String::new();
        let mut escaping = false;

        while let Some(character) = self.next_character() {
            if escaping {
                value.push(match character {
                    '\\' | '\"' => character,
                    't' => '\t',
                    'n' => '\n',
                    'r' => '\r',
                    _ => character
                });

                escaping = false;

                continue;
            };

            if character == '\"' {
                break;
            };

            if character == '\\' {
                escaping = true;
                continue;
            };

            value.push(character);
        }

        if self.current != '\"' {
            return Token::new(TokenValue::Invalid("end of file while parsing string".to_string()), position);
        };

        // we stop at " character, so move to next
        self.next_character();

        Token::new(TokenValue::String(value), position)
    }

    fn lex_number(&mut self) -> Token {
        let position = self.position;

        let mut number_string = String::new();
        let mut is_float = false;

        let mut character = self.current;

        while is_number(character) || character == '.' {
            if character == '.' {
                if is_float {
                    break;
                }

                if !is_number(self.peek()) {
                    break;
                }

                is_float = true;
            };

            number_string.push(character);

            self.next_character();
            character = self.current;
        };

        // TODO : add error handling
        let value = if is_float {
            TokenValue::Float(number_string.parse().unwrap())
        } else {
            TokenValue::Integer(number_string.parse().unwrap())
        };

        Token::new(value, position)
    }

    fn lex_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let position = self.position;

        loop {
            identifier.push(self.current);

            if self.next_character().is_none() {
                break;
            };

            if !is_identifier(self.current) && !is_number(self.current) {
                break;
            };
        };


        if let Some(keyword) = get_keyword(identifier.as_str()) {
            Token::new(keyword, position)
        } else {
            Token::new(TokenValue::Identifier(identifier), position)
        }
    }

    fn lex_symbol(&mut self) -> Token {
        let position = self.position;

        let symbol_string = String::from(self.current);

        self.next_character();

        if is_symbol(self.current) {
            let mut multi_character_symbol_string = symbol_string.clone();
            multi_character_symbol_string.push(self.current);

            if let Some(symbol) = get_symbol(multi_character_symbol_string.as_str()) {
                self.next_character();
                return Token::new(symbol, position);
            };
        };

        Token::new(get_symbol(symbol_string.as_str()).unwrap(), position)
    }

    fn peek(&mut self) -> char {
        if let Some(&character) = self.source.peek() {
            character
        } else {
            '\0'
        }
    }
}

impl<'a> Iterator for LexState<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_spaces_and_comments();

        let character = self.current;

        if character == '\0' {
            return None;
        };

        if is_identifier(character) {
            return Some(self.lex_identifier());
        };

        if is_string(character) {
            return Some(self.lex_string());
        };

        if is_number(character) {
            return Some(self.lex_number());
        };

        if is_symbol(character) {
            return Some(self.lex_symbol());
        };

        self.next_character();
        Some(Token::new(TokenValue::Invalid(format!("unknown character [{}]", character)), self.position))
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
    character == '\"'
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

fn is_comment_prefix(character: char) -> bool {
    character == '#'
}

fn is_symbol(character: char) -> bool {
    let string = String::from(character);

    get_symbol(string.as_str()).is_some()
}

// token helpers

fn get_keyword(keyword: &str) -> Option<TokenValue> {
    match_token! {
        keyword,
        "true"          => TokenValue::True,
        "false"         => TokenValue::False,
        "null"          => TokenValue::Null,

        "and"           => TokenValue::And,
        "or"            => TokenValue::Or,
        "not"           => TokenValue::Not,

        "include"       => TokenValue::Include,
        "from"          => TokenValue::From,
        "model"         => TokenValue::Model,
        "function"      => TokenValue::Function,
        "end"           => TokenValue::End,
        "implement"     => TokenValue::Implement,
        "local"         => TokenValue::Local,
        "apply"         => TokenValue::Apply,
        "to"            => TokenValue::To,
        "return"        => TokenValue::Return,
        "public"        => TokenValue::Public,
        "as"            => TokenValue::As,
        "this"          => TokenValue::This,
        "if"            => TokenValue::If,
        "else"          => TokenValue::Else,
        "elseif"        => TokenValue::ElseIf,
        "while"         => TokenValue::While,
        "for"           => TokenValue::For,
        "in"            => TokenValue::In,
        "break"         => TokenValue::Break,

        "rescue"        => TokenValue::Rescue
    }
}

fn get_symbol(symbol: &str) -> Option<TokenValue> {
    match_token! {
        symbol,
        "="     =>  TokenValue::Assign,
        "+"     =>  TokenValue::Plus,
        "-"     =>  TokenValue::Minus,
        "*"     =>  TokenValue::Star,
        "/"     =>  TokenValue::Slash,
        "%"     =>  TokenValue::Percent,
        "!"     =>  TokenValue::Not,
        "("     =>  TokenValue::LeftParentheses,
        ")"     =>  TokenValue::RightParentheses,
        "["     =>  TokenValue::LeftBracket,
        "]"     =>  TokenValue::RightBracket,
        ","     =>  TokenValue::Comma,
        ":"     =>  TokenValue::Colon,
        "&"     =>  TokenValue::BitAnd,
        "|"     =>  TokenValue::BitOr,
        "."     =>  TokenValue::Dot,
        ">"     =>  TokenValue::Greater,
        "<"     =>  TokenValue::Less,

        "=="    =>  TokenValue::Equal,
        "!="    =>  TokenValue::NotEqual,
        "&&"    =>  TokenValue::And,
        "||"    =>  TokenValue::Or,
        ">="    =>  TokenValue::GreaterEqual,
        "<="    =>  TokenValue::LessEqual,
        "+="    =>  TokenValue::PlusAssign,
        "-="    =>  TokenValue::MinusAssign,
        "*="    =>  TokenValue::StarAssign,
        "/="    =>  TokenValue::SlashAssign,
        "%="    =>  TokenValue::PercentAssign
    }
}

// the main lex function
pub fn lex(source: &str) -> Result<TokenList, CompileErrorList> {
    let mut state = LexState {
        source: source.chars().peekable(),
        position: Position::new(1, 0),
        current: '\0'
    };

    state.next_character();

    let mut tokens = TokenList::new();

    while let Some(token) = state.next() {
        tokens.push(token);
    };

    tokens.push(Token::new(TokenValue::Eof, state.position));

    Ok(tokens)
}
