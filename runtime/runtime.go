package runtime

import (
	"github.com/ippan/clover/ast"
	"github.com/ippan/clover/token"
)

type Runtime struct {
}

func (r *Runtime) Eval(node ast.Node) Object {
	switch node := node.(type) {
	case *ast.Program:
		return r.evalProgram(node)
	case *ast.ExpressionStatement:
		return r.Eval(node.Expression)
	case *ast.IntegerLiteral:
		return &Integer{Value: node.Value}
	case *ast.FloatLiteral:
		return &Float{Value: node.Value}
	case *ast.BooleanLiteral:
		return getBooleanObject(node.Value)
	case *ast.StringLiteral:
		return &String{Value: node.Token.Literal}
	case *ast.PrefixExpression:
		return r.evalPrefixExpression(node)
	case *ast.InfixExpression:
		return r.evalInfixExpression(node)
	}

	return nil
}

func getBooleanObject(value bool) *Boolean {
	if value {
		return TRUE
	}
	return FALSE
}

func (r *Runtime) evalProgram(program *ast.Program) Object {
	var result Object

	for _, statement := range program.Statements {
		result = r.Eval(statement)
	}

	return result
}

func (r *Runtime) evalInfixExpression(ie *ast.InfixExpression) Object {

	left := r.Eval(ie.Left)

	switch ie.Token.Type {
	case token.PLUS:
		return left.Add(r.Eval(ie.Right))
	case token.MINUS:
		return left.Sub(r.Eval(ie.Right))
	case token.STAR:
		return left.Multiply(r.Eval(ie.Right))
	case token.SLASH:
		return left.Divide(r.Eval(ie.Right))
	case token.EQUAL:
		return left.Equal(r.Eval(ie.Right))
	case token.NOT_EQUAL:
		return left.Equal(r.Eval(ie.Right)).Not()
	}

	return nil
}

func (r *Runtime) evalPrefixExpression(pe *ast.PrefixExpression) Object {
	switch pe.Token.Type {
	case token.NOT:
		return r.Eval(pe.Right).Not()
	case token.MINUS:
		return r.Eval(pe.Right).Negative()
	}
	return nil
}
