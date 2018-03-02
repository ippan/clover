package parser

import (
	"github.com/ippan/clover/lexer"
	"github.com/ippan/clover/token"
)

type Parser struct {
	l *lexer.Lexer
	current_token token.Token
	peek_token token.Token
}
