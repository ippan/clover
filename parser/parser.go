package parser

import (
	"github.com/ippan/clover/lexer"
	"github.com/ippan/clover/token"
)

type Parser struct {
	l            *lexer.Lexer
	currentToken token.Token
	peekToken    token.Token
}
