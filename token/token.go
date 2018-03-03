package token

type TokenType int

const (
	_ = iota
	INVALID
	EOF

	IDENTIFIER
	STRING
	INTEGER
	FLOAT

	TRUE
	FALSE
	NULL

	ASSIGN
	PLUS
	MINUS
	STAR
	SLASH
	PLUS_ASSIGN
	MINUS_ASSIGN
	STAR_ASSIGN
	SLASH_ASSIGN

	BIT_AND
	BIT_OR

	NOT
	AND
	OR

	EQUAL
	NOT_EQUAL
	LESS
	GREATER
	GREATER_EQUAL
	LESS_EQUAL

	LEFT_PARENTHESES
	RIGHT_PARENTHESES

	COMMA

	END
	FUNCTION
	CLASS
	EXTENDS

	IF
	ELSE
	WHILE
)

type Token struct {
	Type    TokenType
	Literal string
}

func New(token_type TokenType, literal string) Token {
	return Token{Type: token_type, Literal: literal}
}
