package lexer

import (
	"github.com/ippan/clover/token"
)

type Lexer struct {
	input            string
	position         int
	line             int
	currentCharacter byte
}

var keywordTokens = map[string]token.TokenType{
	"function": token.FUNCTION,
	"end":      token.END,
	"if":       token.IF,
	"else":     token.ELSE,
	"and":      token.AND,
	"or":       token.OR,
	"not":      token.NOT,
	"true":     token.TRUE,
	"false":    token.FALSE,
	"null":     token.NULL,
	"class":    token.CLASS,
	"extends":  token.EXTENDS,
	"while":    token.WHILE,
}

var symbolTokens = map[string]token.TokenType{
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
	">":  token.GREATER,
	"<":  token.LESS,
	">=": token.GREATER_EQUAL,
	"<=": token.LESS_EQUAL,
	"+=": token.PLUS_ASSIGN,
	"-=": token.MINUS_ASSIGN,
	"*=": token.STAR_ASSIGN,
	"/=": token.SLASH_ASSIGN,
}

func New(input string) *Lexer {
	lexer := &Lexer{input: input, line: 1}
	lexer.nextCharacter()
	return lexer
}

func (lexer *Lexer) nextCharacter() {
	if lexer.position >= len(lexer.input) {
		lexer.currentCharacter = 0
	} else {
		lexer.currentCharacter = lexer.input[lexer.position]

		if lexer.currentCharacter == '\n' {
			lexer.line += 1
		}
	}
	lexer.position += 1
}

func isSpace(character byte) bool {
	return character == ' ' || character == '\t' || character == '\r' || character == '\v' || character == '\f' || character == '\n'
}

func isString(character byte) bool {
	return character == '"' || character == '\''
}

func isAlpha(character byte) bool {
	return (character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z')
}

func isIdentifier(character byte) bool {
	return character == '_' || isAlpha(character)
}

func isNumber(character byte) bool {
	return character >= '0' && character <= '9'
}

func (lexer *Lexer) skipComment() {
	for !(lexer.currentCharacter == 0 || lexer.currentCharacter == '\n') {
		lexer.nextCharacter()
	}
	lexer.nextCharacter()
}

func (lexer *Lexer) lexString() token.Token {

	// TODO : parse string with special character like \n if it is a "" string"

	boundaryCharacter := lexer.currentCharacter
	position := lexer.position
	for {
		lexer.nextCharacter()
		if lexer.currentCharacter == boundaryCharacter || lexer.currentCharacter == 0 {
			break
		}
	}
	endPosition := lexer.position - 1
	lexer.nextCharacter()

	return token.New(token.STRING, lexer.input[position:endPosition])
}

func (lexer *Lexer) lexIdentifier() token.Token {
	position := lexer.position - 1
	for {
		lexer.nextCharacter()
		if !(isIdentifier(lexer.currentCharacter) || isNumber(lexer.currentCharacter)) {
			identifier := lexer.input[position : lexer.position-1]

			if keywordType, ok := keywordTokens[identifier]; ok {
				return token.New(keywordType, identifier)
			}

			return token.New(token.IDENTIFIER, identifier)
		}
	}
}

func (lexer *Lexer) lexNumber() token.Token {
	var tokenType token.TokenType = token.INTEGER
	position := lexer.position - 1

	for {
		lexer.nextCharacter()
		if isNumber(lexer.currentCharacter) || lexer.currentCharacter == '.' {
			if lexer.currentCharacter == '.' {
				if tokenType == token.FLOAT {
					tokenType = token.INVALID
				} else if tokenType == token.INTEGER {
					tokenType = token.FLOAT
				}
			}
		} else {
			return token.New(tokenType, lexer.input[position:lexer.position-1])
		}
	}
}

func (lexer *Lexer) lexSymbol() token.Token {

	symbol := string(lexer.currentCharacter)

	lexer.nextCharacter()

	if isSymbol(lexer.currentCharacter) {
		multiCharactersSymbol := lexer.input[lexer.position-2 : lexer.position]

		if tokenType, ok := symbolTokens[multiCharactersSymbol]; ok {
			lexer.nextCharacter()
			return token.New(tokenType, multiCharactersSymbol)
		}
	}

	if tokenType, ok := symbolTokens[symbol]; ok {
		return token.New(tokenType, symbol)
	}

	return token.New(token.INVALID, symbol)
}

func isSymbol(character byte) bool {
	_, ok := symbolTokens[string(character)]
	return ok
}

func (lexer *Lexer) Lex() token.Token {
	if lexer.currentCharacter == 0 {
		return token.New(token.EOF, "")
	}

	if isSpace(lexer.currentCharacter) {
		lexer.nextCharacter()
		return lexer.Lex()
	}

	if lexer.currentCharacter == '#' {
		lexer.skipComment()
		return lexer.Lex()
	}

	if isString(lexer.currentCharacter) {
		return lexer.lexString()
	}

	if isIdentifier(lexer.currentCharacter) {
		return lexer.lexIdentifier()
	}

	if isNumber(lexer.currentCharacter) {
		return lexer.lexNumber()
	}

	if isSymbol(lexer.currentCharacter) {
		return lexer.lexSymbol()
	}

	return token.New(token.INVALID, string(lexer.currentCharacter))
}
