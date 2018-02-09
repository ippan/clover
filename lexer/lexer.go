package lexer

import (
	"github.com/ippan/clover/token"
)

type Lexer struct {
	input string
	position int
	line int
	current_character byte
}

var keyword_tokens = map[string]token.TokenType {
	"function": token.FUNCTION,
	"end": token.END,
	"if": token.IF,
	"else": token.ELSE,
	"and": token.AND,
	"or": token.OR,
	"not": token.NOT,
	"true": token.TRUE,
	"false": token.FALSE,
	"null": token.NULL,
	"class": token.CLASS,
	"extends": token.EXTENDS,
	"while": token.WHILE,
}

var symbol_tokens = map[string]token.TokenType {
	"=": token.EQUAL,
	"+": token.PLUS,
	"-": token.MINUS,
	"*": token.STAR,
	"/": token.SLASH,
	"!": token.NOT,
	"(": token.LEFT_PARENTHESES,
	")": token.RIGHT_PARENTHESES,
	",": token.COMMA,
	"&": token.BIT_AND,
	"|": token.BIT_OR,

	"==": token.EQUAL,
	"!=": token.NOT_EQUAL,
	"&&": token.AND,
	"||": token.OR,
	">": token.GREATER,
	"<": token.LESS,
	">=": token.GREATER_EQUAL,
	"<=": token.LESS_EQUAL,
	"+=": token.PLUS_ASSIGN,
	"-=": token.MINUS_ASSIGN,
	"*=": token.STAR_ASSIGN,
	"/=": token.SLASH_ASSIGN,
}

func New(input string) *Lexer {
	lexer := &Lexer{ input: input, line: 1 }
	lexer.next_character()
	return lexer
}

func (lexer *Lexer) next_character() {
	if lexer.position >= len(lexer.input) {
		lexer.current_character = 0
	} else {
		lexer.current_character = lexer.input[lexer.position]

		if lexer.current_character == '\n' {
			lexer.line += 1
		}
	}
	lexer.position += 1
}

func is_space(character byte) bool {
	return character == ' ' || character == '\t' || character == '\r' || character == '\v' || character == '\f' || character == '\n';
}

func is_string(character byte) bool {
	return character == '"' || character == '\''
}

func is_alpha(character byte) bool {
	return (character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z')
}

func is_identifier(character byte) bool {
	return character == '_' || is_alpha(character)
}

func is_number(character byte) bool {
	return character >= '0' && character <= '9'
}

func (lexer *Lexer) skip_comment() {
	for !(lexer.current_character == 0 || lexer.current_character == '\n') {
		lexer.next_character()
	}
	lexer.next_character()
}

func (lexer *Lexer) lex_string() token.Token {

	// TODO : parase string with special character like \n if it is a "" string"

	boundary_character := lexer.current_character
	position := lexer.position
	for {
		lexer.next_character()
		if lexer.current_character == boundary_character || lexer.current_character == 0 {
			break
		}
	}
	end_position := lexer.position - 1
	lexer.next_character()

	return token.New(token.STRING, lexer.input[position:end_position])
}

func (lexer *Lexer) lex_identifier() token.Token {
	position := lexer.position - 1
	for {
		lexer.next_character()
		if !(is_identifier(lexer.current_character) || is_number(lexer.current_character)) {
			identifier := lexer.input[position:lexer.position - 1]

			if keyword_type, ok := keyword_tokens[identifier]; ok {
				return token.New(keyword_type, identifier)
			}

			return token.New(token.IDENTIFIER, identifier)
		}
	}
}

func (lexer *Lexer) lex_number() token.Token {
	var token_type token.TokenType = token.INTEGER
	position := lexer.position - 1

	for {
		lexer.next_character()
		if is_number(lexer.current_character) || lexer.current_character == '.' {
			if lexer.current_character == '.' {
				if token_type == token.FLOAT {
					token_type = token.INVALID
				} else if token_type == token.INTEGER {
					token_type = token.FLOAT
				}
			}
		} else {
			return token.New(token_type, lexer.input[position:lexer.position - 1])
		}
	}
}

func (lexer *Lexer) lex_symbol() token.Token {

	symbol := string(lexer.current_character)

	lexer.next_character()

	if is_symbol(lexer.current_character) {
		multi_characters_symbol := lexer.input[lexer.position - 2:lexer.position]

		if token_type, ok := symbol_tokens[multi_characters_symbol]; ok {
			lexer.next_character()
			return token.New(token_type, multi_characters_symbol)
		}
	}

	if token_type, ok := symbol_tokens[symbol]; ok {
		return token.New(token_type, symbol)
	}

	return token.New(token.INVALID, symbol)
}

func is_symbol(character byte) bool {
	_, ok := symbol_tokens[string(character)]
	return ok
}

func (lexer *Lexer) Lex() token.Token {
	if lexer.current_character == 0 {
		return token.New(token.EOF, "")
	}

	if is_space(lexer.current_character) {
		lexer.next_character()
		return lexer.Lex()
	}

	if lexer.current_character == '#' {
		lexer.skip_comment()
		return lexer.Lex()
	}

	if is_string(lexer.current_character) {
		return lexer.lex_string()
	}

	if is_identifier(lexer.current_character) {
		return lexer.lex_identifier()
	}

	if is_number(lexer.current_character) {
		return lexer.lex_number()
	}

	if is_symbol(lexer.current_character) {
		return lexer.lex_symbol()
	}

	return token.New(token.INVALID, string(lexer.current_character))
}