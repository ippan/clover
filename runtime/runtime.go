package runtime

import (
	"github.com/ippan/clover/ast"
	"github.com/ippan/clover/token"
)

type Runtime struct {
	context *Environment
}

func New() *Runtime {
	return &Runtime{context: NewEnvironment()}
}

func (r *Runtime) Eval(node ast.Node) Object {
	return r.eval(node, r.context)
}

func (r *Runtime) eval(node ast.Node, context Context) Object {
	switch node := node.(type) {
	case *ast.Program:
		return r.evalProgram(node, context)
	case *ast.ExpressionStatement:
		return r.eval(node.Expression, context)
	case *ast.IntegerLiteral:
		return &Integer{Value: node.Value}
	case *ast.FloatLiteral:
		return &Float{Value: node.Value}
	case *ast.BooleanLiteral:
		return getBooleanObject(node.Value)
	case *ast.NullLiteral:
		return NULL
	case *ast.StringLiteral:
		return &String{Value: node.Token.Literal}
	case *ast.PrefixExpression:
		return r.evalPrefixExpression(node, context)
	case *ast.InfixExpression:
		return r.evalInfixExpression(node, context)
	case *ast.IfExpression:
		return r.evalIfExpression(node, context)
	case *ast.Identifier:
		return context.Get(node.Value)
	}

	return nil
}

func getBooleanObject(value bool) *Boolean {
	if value {
		return TRUE
	}
	return FALSE
}

func (r *Runtime) evalProgram(program *ast.Program, context Context) Object {
	var result Object

	for _, statement := range program.Statements {
		result = r.eval(statement, context)
	}

	return result
}

func (r *Runtime) evalInfixExpression(ie *ast.InfixExpression, context Context) Object {

	left := r.eval(ie.Left, context)

	switch ie.Token.Type {
	case token.PLUS:
		return left.Add(r.eval(ie.Right, context))
	case token.MINUS:
		return left.Sub(r.eval(ie.Right, context))
	case token.STAR:
		return left.Multiply(r.eval(ie.Right, context))
	case token.SLASH:
		return left.Divide(r.eval(ie.Right, context))
	case token.EQUAL:
		return left.Equal(r.eval(ie.Right, context))
	case token.NOT_EQUAL:
		return left.Equal(r.eval(ie.Right, context)).Not()
	case token.ASSIGN:
		return r.evalAssignExpression(left, r.eval(ie.Right, context))
	case token.PLUS_ASSIGN:
		return r.evalAssignExpression(left, left.Add(r.eval(ie.Right, context)))
	case token.MINUS_ASSIGN:
		return r.evalAssignExpression(left, left.Sub(r.eval(ie.Right, context)))
	case token.STAR_ASSIGN:
		return r.evalAssignExpression(left, left.Multiply(r.eval(ie.Right, context)))
	case token.SLASH_ASSIGN:
		return r.evalAssignExpression(left, left.Divide(r.eval(ie.Right, context)))
	}

	return nil
}

func (r *Runtime) evalAssignExpression(left Object, right Object) Object {

	if binding, ok := left.(*ObjectBinding); ok {
		binding.BindingContext.Set(binding.Name, right)
		return right
	}

	// TODO : raise error
	return nil
}

func (r *Runtime) evalPrefixExpression(pe *ast.PrefixExpression, context Context) Object {
	switch pe.Token.Type {
	case token.NOT:
		return r.eval(pe.Right, context).Not()
	case token.MINUS:
		return r.eval(pe.Right, context).Negative()
	}
	return nil
}

func (r *Runtime) evalIfExpression(ie *ast.IfExpression, context Context) Object {
	if r.eval(ie.Condition, context).ToBoolean().Value {
		return r.eval(ie.TruePart, context)
	} else if ie.FalsePart != nil {
		return r.eval(ie.FalsePart, context)
	}

	return NULL
}
