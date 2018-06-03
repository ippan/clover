package lexer

import (
	"github.com/ippan/clover/token"
	"testing"
)

func assertTokens(name string, tokens []token.Token, l *Lexer, t *testing.T) {
	for i, tokenExpected := range tokens {
		tok := l.Lex()

		if tok.Type != tokenExpected.Type {
			t.Fatalf("tests[%q] - token[%d] type not match. expected: %q, got: %q", name, i, tokenExpected.Type, tok.Type)
		}

		if tok.Literal != tokenExpected.Literal {
			t.Fatalf("tests[%q] - token[%d] literal not match. expected: %q, got: %q", name, i, tokenExpected.Literal, tok.Literal)
		}

	}
}

func TestLexClass(t *testing.T) {
	input := "test = class Test extends TestParent end"
	tokenExpected := []token.Token{
		{token.IDENTIFIER, "test"},
		{token.ASSIGN, "="},
		{token.CLASS, "class"},
		{token.IDENTIFIER, "Test"},
		{token.EXTENDS, "extends"},
		{token.IDENTIFIER, "TestParent"},
		{token.END, "end"},
	}

	l := New(input)

	assertTokens("class", tokenExpected, l, t)
}

func TestLexString(t *testing.T) {
	input := `test = "this is string."`
	tokenExpected := []token.Token{
		{token.IDENTIFIER, "test"},
		{token.ASSIGN, "="},
		{token.STRING, "this is string."},
	}

	l := New(input)

	assertTokens("string", tokenExpected, l, t)
}

func TestLexFunctions(t *testing.T) {
	input := "test = function Test() end"
	tokenExpected := []token.Token{
		{token.IDENTIFIER, "test"},
		{token.ASSIGN, "="},
		{token.FUNCTION, "function"},
		{token.IDENTIFIER, "Test"},
		{token.LEFT_PARENTHESES, "("},
		{token.RIGHT_PARENTHESES, ")"},
		{token.END, "end"},
	}

	l := New(input)

	assertTokens("function", tokenExpected, l, t)
}
