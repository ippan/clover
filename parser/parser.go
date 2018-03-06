package parser

import (
	"fmt"
	"github.com/ippan/clover/ast"
	"github.com/ippan/clover/lexer"
	"github.com/ippan/clover/token"
	"strconv"
)

const (
	_ int = iota
	LOWEST
	ASSIGN
	BOOLEAN
	EQUALS
	LESSGREATER
	SUM
	PRODUCT
	PREFIX
	CALL
	DOT
)

var precedences = map[token.TokenType]int{
	token.ASSIGN:           ASSIGN,
	token.PLUS_ASSIGN:      ASSIGN,
	token.MINUS_ASSIGN:     ASSIGN,
	token.STAR_ASSIGN:      ASSIGN,
	token.SLASH_ASSIGN:     ASSIGN,
	token.AND:              BOOLEAN,
	token.OR:               BOOLEAN,
	token.EQUAL:            EQUALS,
	token.NOT_EQUAL:        EQUALS,
	token.LESS:             LESSGREATER,
	token.GREATER:          LESSGREATER,
	token.LESS_EQUAL:       LESSGREATER,
	token.GREATER_EQUAL:    LESSGREATER,
	token.PLUS:             SUM,
	token.MINUS:            SUM,
	token.STAR:             PRODUCT,
	token.SLASH:            PRODUCT,
	token.BIT_AND:          PRODUCT,
	token.BIT_OR:           PRODUCT,
	token.DOT:              DOT,
	token.LEFT_PARENTHESES: CALL,
}

type prefixFunction func() ast.Expression
type infixFunction func(expression ast.Expression) ast.Expression

type Parser struct {
	l      *lexer.Lexer
	errors []string

	currentToken token.Token
	peekToken    token.Token

	currentTokenLine int
	peekTokenLine    int

	prefixFunctions map[token.TokenType]prefixFunction
	infixFunctions  map[token.TokenType]infixFunction
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l}

	p.prefixFunctions = make(map[token.TokenType]prefixFunction)

	p.registerPrefixFunction(token.IDENTIFIER, p.parseIdentifier)
	p.registerPrefixFunction(token.BASE, p.parseBaseLiteral)
	p.registerPrefixFunction(token.THIS, p.parseThisLiteral)
	p.registerPrefixFunction(token.INTEGER, p.parseIntegerLiteral)
	p.registerPrefixFunction(token.FLOAT, p.parseFloatLiteral)
	p.registerPrefixFunction(token.TRUE, p.parseBooleanLiteral)
	p.registerPrefixFunction(token.FALSE, p.parseBooleanLiteral)
	p.registerPrefixFunction(token.STRING, p.parseStringLiteral)
	p.registerPrefixFunction(token.NULL, p.parseNullLiteral)
	p.registerPrefixFunction(token.MINUS, p.parsePrefixExpression)
	p.registerPrefixFunction(token.NOT, p.parsePrefixExpression)
	p.registerPrefixFunction(token.LEFT_PARENTHESES, p.parseGroupExpression)
	p.registerPrefixFunction(token.IF, p.parseIfExpression)
	p.registerPrefixFunction(token.WHILE, p.parseWhileExpression)
	p.registerPrefixFunction(token.FUNCTION, p.parseFunctionExpression)
	p.registerPrefixFunction(token.CLASS, p.parseClassExpression)

	p.infixFunctions = make(map[token.TokenType]infixFunction)

	p.registerInfixFunction(token.ASSIGN, p.parseInfixExpression)
	p.registerInfixFunction(token.PLUS_ASSIGN, p.parseInfixExpression)
	p.registerInfixFunction(token.MINUS_ASSIGN, p.parseInfixExpression)
	p.registerInfixFunction(token.STAR_ASSIGN, p.parseInfixExpression)
	p.registerInfixFunction(token.SLASH_ASSIGN, p.parseInfixExpression)
	p.registerInfixFunction(token.AND, p.parseInfixExpression)
	p.registerInfixFunction(token.OR, p.parseInfixExpression)
	p.registerInfixFunction(token.EQUAL, p.parseInfixExpression)
	p.registerInfixFunction(token.NOT_EQUAL, p.parseInfixExpression)
	p.registerInfixFunction(token.LESS, p.parseInfixExpression)
	p.registerInfixFunction(token.GREATER, p.parseInfixExpression)
	p.registerInfixFunction(token.LESS_EQUAL, p.parseInfixExpression)
	p.registerInfixFunction(token.GREATER_EQUAL, p.parseInfixExpression)
	p.registerInfixFunction(token.PLUS, p.parseInfixExpression)
	p.registerInfixFunction(token.MINUS, p.parseInfixExpression)
	p.registerInfixFunction(token.STAR, p.parseInfixExpression)
	p.registerInfixFunction(token.SLASH, p.parseInfixExpression)
	p.registerInfixFunction(token.BIT_AND, p.parseInfixExpression)
	p.registerInfixFunction(token.BIT_OR, p.parseInfixExpression)
	p.registerInfixFunction(token.DOT, p.parseInfixExpression)
	p.registerInfixFunction(token.LEFT_PARENTHESES, p.parseCallExpression)

	// run twice to set current and peek token
	p.nextToken()
	p.nextToken()

	return p
}

func (p *Parser) Errors() []string {
	return p.errors
}

func (p *Parser) pushError(error string) {
	p.errors = append(p.errors, fmt.Sprintf("[line %d] %s", p.currentTokenLine, error))
}

func (p *Parser) nextToken() {
	p.currentToken = p.peekToken
	p.currentTokenLine = p.peekTokenLine

	p.peekToken = p.l.Lex()
	p.peekTokenLine = p.l.TokenLine()
}

func (p *Parser) currentTokenIs(t token.TokenType) bool {
	return p.currentToken.Type == t
}

func (p *Parser) peekTokenIs(t token.TokenType) bool {
	return p.peekToken.Type == t
}

func (p *Parser) expectToken(t token.Token, tt token.TokenType) bool {
	if t.Type == tt {
		return true
	}
	p.pushError(fmt.Sprintf("expected token type is %s, got %s", tt, t.Type))
	return false
}

func (p *Parser) expectPeek(t token.TokenType) bool {
	if p.expectToken(p.peekToken, t) {
		p.nextToken()
		return true
	}

	return false
}

func (p *Parser) currentPrecedence() int {
	if precedence, ok := precedences[p.currentToken.Type]; ok {
		return precedence
	}

	return LOWEST
}

func (p *Parser) peekPrecedence() int {
	if precedence, ok := precedences[p.peekToken.Type]; ok {
		return precedence
	}

	return LOWEST
}

func (p *Parser) registerPrefixFunction(tt token.TokenType, function prefixFunction) {
	p.prefixFunctions[tt] = function
}

func (p *Parser) registerInfixFunction(tt token.TokenType, function infixFunction) {
	p.infixFunctions[tt] = function
}

func (p *Parser) parseProgram(terminators []token.TokenType) *ast.Program {
	program := &ast.Program{Statements: []ast.Statement{}}

	for {
		for _, terminator := range terminators {
			if p.currentTokenIs(terminator) {
				return program
			}
		}

		statement := p.parseStatement()
		if statement != nil {
			program.Statements = append(program.Statements, statement)
		}
		p.nextToken()
	}
}

func (p *Parser) Parse() *ast.Program {
	return p.parseProgram([]token.TokenType{token.EOF})
}

func (p *Parser) parseExpressionStatement() *ast.ExpressionStatement {
	statement := &ast.ExpressionStatement{Token: p.currentToken}

	statement.Expression = p.parseExpression(LOWEST)

	return statement
}

func (p *Parser) parseReturnStatement() *ast.ReturnStatement {
	statement := &ast.ReturnStatement{Token: p.currentToken}

	// have return value?
	if (p.currentTokenLine == p.peekTokenLine) && !p.peekTokenIs(token.EOF) {
		p.nextToken()
		statement.ReturnValue = p.parseExpression(LOWEST)
	}

	return statement
}

func (p *Parser) parseStatement() ast.Statement {
	switch p.currentToken.Type {
	case token.RETURN:
		return p.parseReturnStatement()
	}

	return p.parseExpressionStatement()
}

func (p *Parser) parseExpression(precedence int) ast.Expression {
	prefix := p.prefixFunctions[p.currentToken.Type]

	if prefix == nil {
		p.pushError(fmt.Sprintf("prefix function for %s does not exists", p.currentToken.Literal))
		return nil
	}

	leftExpression := prefix()

	for !p.peekTokenIs(token.EOF) && precedence < p.peekPrecedence() {
		infix, ok := p.infixFunctions[p.peekToken.Type]
		if !ok {
			return leftExpression
		}

		p.nextToken()

		leftExpression = infix(leftExpression)
	}

	return leftExpression
}

func (p *Parser) parseIdentifier() ast.Expression {
	return &ast.Identifier{Token: p.currentToken, Value: p.currentToken.Literal}
}

func (p *Parser) parseBaseLiteral() ast.Expression {
	return &ast.BaseLiteral{Token: p.currentToken}
}

func (p *Parser) parseThisLiteral() ast.Expression {
	return &ast.ThisLiteral{Token: p.currentToken}
}

func (p *Parser) parseIntegerLiteral() ast.Expression {
	it := &ast.IntegerLiteral{Token: p.currentToken}

	value, error := strconv.ParseInt(p.currentToken.Literal, 0, 64)

	if error != nil {
		p.pushError(fmt.Sprintf("%q is not an integer", p.currentToken.Literal))
	}

	it.Value = value

	return it
}

func (p *Parser) parseFloatLiteral() ast.Expression {
	ft := &ast.FloatLiteral{Token: p.currentToken}

	value, error := strconv.ParseFloat(p.currentToken.Literal, 64)

	if error != nil {
		p.pushError(fmt.Sprintf("%q is not a float", p.currentToken.Literal))
	}

	ft.Value = value

	return ft
}

func (p *Parser) parseBooleanLiteral() ast.Expression {
	return &ast.BooleanLiteral{Token: p.currentToken, Value: p.currentTokenIs(token.TRUE)}
}

func (p *Parser) parseStringLiteral() ast.Expression {
	return &ast.StringLiteral{Token: p.currentToken}
}

func (p *Parser) parseNullLiteral() ast.Expression {
	return &ast.NullLiteral{Token: p.currentToken}
}

func (p *Parser) parsePrefixExpression() ast.Expression {
	pe := &ast.PrefixExpression{Token: p.currentToken}

	p.nextToken()

	pe.Right = p.parseExpression(PREFIX)

	return pe
}

func (p *Parser) parseInfixExpression(left ast.Expression) ast.Expression {
	ie := &ast.InfixExpression{Left: left, Token: p.currentToken}

	precedence := p.currentPrecedence()

	p.nextToken()

	ie.Right = p.parseExpression(precedence)

	return ie
}

func (p *Parser) parseGroupExpression() ast.Expression {
	p.nextToken()

	expression := p.parseExpression(LOWEST)

	if !p.expectPeek(token.RIGHT_PARENTHESES) {
		return nil
	}

	return expression
}

func (p *Parser) parseIfExpression() ast.Expression {
	ie := &ast.IfExpression{Token: p.currentToken}

	if !p.expectPeek(token.LEFT_PARENTHESES) {
		return nil
	}

	p.nextToken()
	ie.Condition = p.parseExpression(LOWEST)

	if !p.expectPeek(token.RIGHT_PARENTHESES) {
		return nil
	}

	p.nextToken()

	ie.TruePart = p.parseProgram([]token.TokenType{token.ELSE, token.END})

	if p.currentTokenIs(token.ELSE) {
		p.nextToken()
		ie.FalsePart = p.parseProgram([]token.TokenType{token.END})
	}

	if !p.expectToken(p.currentToken, token.END) {
		return nil
	}

	return ie
}

func (p *Parser) parseWhileExpression() ast.Expression {
	we := &ast.WhileExpression{Token: p.currentToken}

	if !p.expectPeek(token.LEFT_PARENTHESES) {
		return nil
	}

	p.nextToken()
	we.Condition = p.parseExpression(LOWEST)

	if !p.expectPeek(token.RIGHT_PARENTHESES) {
		return nil
	}

	p.nextToken()

	we.Body = p.parseProgram([]token.TokenType{token.END})

	if !p.expectToken(p.currentToken, token.END) {
		return nil
	}

	return we
}

func (p *Parser) parseParameters(separator token.TokenType, assignTokenType token.TokenType, terminator token.TokenType) []*ast.Parameter {
	var parameters []*ast.Parameter

	for !p.currentTokenIs(terminator) {

		if !p.expectToken(p.currentToken, token.IDENTIFIER) {
			return nil
		}

		parameter := &ast.Parameter{Name: p.parseIdentifier().(*ast.Identifier)}

		if p.peekTokenIs(assignTokenType) {
			p.nextToken()
			p.nextToken()
			parameter.Value = p.parseExpression(LOWEST)
		}

		parameters = append(parameters, parameter)

		if separator != token.EMPTY && !p.peekTokenIs(terminator) {
			if !p.expectPeek(separator) {
				return nil
			}
		}

		p.nextToken()
	}

	return parameters
}

func (p *Parser) parseFunctionExpression() ast.Expression {
	fe := &ast.FunctionExpression{Token: p.currentToken}

	if !p.expectPeek(token.LEFT_PARENTHESES) {
		return nil
	}

	p.nextToken()

	fe.Parameters = p.parseParameters(token.COMMA, token.ASSIGN, token.RIGHT_PARENTHESES)

	p.nextToken()

	fe.Body = p.parseProgram([]token.TokenType{token.END})

	if !p.expectToken(p.currentToken, token.END) {
		return nil
	}

	return fe
}

func (p *Parser) parseCallExpression(function ast.Expression) ast.Expression {
	ce := &ast.CallExpression{Function: function}

	if !p.expectToken(p.currentToken, token.LEFT_PARENTHESES) {
		return nil
	}

	p.nextToken()

	var arguments []ast.Expression

	for !p.currentTokenIs(token.RIGHT_PARENTHESES) {
		expression := p.parseExpression(LOWEST)
		arguments = append(arguments, expression)

		if !p.peekTokenIs(token.RIGHT_PARENTHESES) {
			if !p.expectPeek(token.COMMA) {
				return nil
			}
		}

		p.nextToken()
	}

	ce.Arguments = arguments

	return ce
}

func (p *Parser) parseClassExpression() ast.Expression {
	ce := &ast.ClassExpression{Token: p.currentToken}

	if p.peekTokenIs(token.EXTENDS) {
		p.nextToken()
		p.nextToken()
		ce.Parent = p.parseExpression(LOWEST)
	}

	p.nextToken()

	ce.Body = p.parseParameters(token.EMPTY, token.ASSIGN, token.END)

	if !p.expectToken(p.currentToken, token.END) {
		return nil
	}

	return ce
}
