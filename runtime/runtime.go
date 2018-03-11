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
	case *ast.ReturnStatement:
		return r.evalReturnStatement(node, context)
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
	case *ast.WhileExpression:
		return r.evalWhileExpression(node, context)
	case *ast.Identifier:
		return context.Get(node.Value)
	case *ast.FunctionExpression:
		return r.evalFunctionExpression(node, context)
	case *ast.CallExpression:
		return r.evalCallExpression(node, context)
	case *ast.ClassExpression:
		return r.evalClassExpression(node, context)
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
		if _, ok := result.(*Return); ok {
			return result
		}
	}

	return result
}

func (r *Runtime) evalReturnStatement(rs *ast.ReturnStatement, context Context) Object {
	var result Object = NULL

	if rs.ReturnValue != nil {
		result = r.eval(rs.ReturnValue, context)
	}

	return &Return{Value: result}
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
	case token.GREATER:
		return getBooleanObject(left.Compare(r.eval(ie.Right, context)).Value > 0)
	case token.LESS:
		return getBooleanObject(left.Compare(r.eval(ie.Right, context)).Value < 0)
	case token.GREATER_EQUAL:
		return getBooleanObject(left.Compare(r.eval(ie.Right, context)).Value >= 0)
	case token.LESS_EQUAL:
		return getBooleanObject(left.Compare(r.eval(ie.Right, context)).Value <= 0)
	case token.AND:
		if left.ToBoolean().Value == false {
			return left
		}
		return r.eval(ie.Right, context)
	case token.OR:
		if left.ToBoolean().Value {
			return left
		}
		return r.eval(ie.Right, context)
	case token.DOT:
		return r.evalGetMemberExpression(left, ie.Right)
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
	fe := NewFunctionEnvironment(context)
	if r.eval(ie.Condition, fe).ToBoolean().Value {
		return r.eval(ie.TruePart, NewFunctionEnvironment(fe))
	} else if ie.FalsePart != nil {
		return r.eval(ie.FalsePart, NewFunctionEnvironment(fe))
	}

	return NULL
}

func (r *Runtime) evalWhileExpression(ie *ast.WhileExpression, context Context) Object {
	fe := NewFunctionEnvironment(context)
	var result Object = NULL

	for r.eval(ie.Condition, fe).ToBoolean().Value {
		result = r.eval(ie.Body, fe)
		if _, ok := result.(*Return); ok {
			return result
		}
		fe = NewFunctionEnvironment(context)
	}

	return result
}

func (r *Runtime) evalFunctionExpression(fe *ast.FunctionExpression, context Context) Object {
	return &Function{BindingContext: context, Parameters: fe.Parameters, Body: fe.Body}
}

func unwrap(wrapped Object) Object {
	if binding, ok := wrapped.(*ObjectBinding); ok {
		return binding.UnWarp()
	}
	return wrapped
}

func (r *Runtime) prepareParameters(context Context, bindingContext Context, parameterContext Context, parameters []*ast.Parameter, arguments []ast.Expression) {
	for i, parameter := range parameters {
		if i < len(arguments) {
			// argument - use caller context
			parameterContext.InstanceSet(parameter.Name.Value, r.eval(arguments[i], context))
		} else if parameter.Value != nil {
			// optional parameter - use function context
			parameterContext.InstanceSet(parameter.Name.Value, r.eval(parameter.Value, bindingContext))
		} else {
			parameterContext.InstanceSet(parameter.Name.Value, NULL)
		}
	}
}

func (r *Runtime) evalCallExpression(ce *ast.CallExpression, context Context) Object {

	var result Object = NULL

	switch function := unwrap(r.eval(ce.Function, context)).(type) {
	case *Function:
		fe := NewFunctionEnvironment(function.BindingContext)
		r.prepareParameters(context, function.BindingContext, fe, function.Parameters, ce.Arguments)
		result = r.eval(function.Body, fe)
	case *Constructor:
		return r.evalCallConstructorExpression(function, ce.Arguments, context)
	default:
		// TODO : raise error
		return nil
	}

	if returnObject, ok := result.(*Return); ok {
		return returnObject.Value
	}

	return result
}

func (r *Runtime) evalClassExpression(ce *ast.ClassExpression, context Context) Object {

	c := &Class{BindingContext: context, Body: ce.Body}

	if ce.Parent != nil {
		c.Parent = unwrap(r.eval(ce.Parent, context))

		if c.Parent.Type() != TYPE_CLASS {
			// TODO : raise error
			return nil
		}

	}

	return c
}

func (r *Runtime) evalGetMemberExpression(receiver Object, member ast.Expression) Object {

	if identifier, ok := member.(*ast.Identifier); ok {
		if receiver.Type() == TYPE_CLASS && identifier.Value == "new" {
			if classObject, ok := unwrap(receiver).(*Class); ok {
				return &Constructor{Receiver: classObject}
			}
		}

		return receiver.GetMember(identifier.Value)
	}

	// TODO : raise error
	return nil
}

func (r *Runtime) evalCallConstructorExpression(constructor *Constructor, arguments []ast.Expression, context Context) Object {

	return nil
}
