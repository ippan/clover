package ast

import (
	"bytes"
	"github.com/ippan/clover/token"
)

type Node interface {
	String() string
}

type Statement interface {
	Node
}

type Expression interface {
	Node
}

type Program struct {
	Statements []Statement
}

func (p *Program) String() string {
	var out bytes.Buffer

	for _, s := range p.Statements {
		out.WriteString(s.String())
		out.WriteString("\n")
	}

	return out.String()
}

// statements

type ReturnStatement struct {
	Token       token.Token
	ReturnValue Expression
}

func (rs *ReturnStatement) String() string {
	var out bytes.Buffer

	out.WriteString(rs.Token.Literal + " ")

	if rs.ReturnValue != nil {
		out.WriteString(rs.ReturnValue.String())
	}

	return out.String()
}

type ExpressionStatement struct {
	Token      token.Token
	Expression Expression
}

func (es *ExpressionStatement) String() string {
	if es.Expression != nil {
		return es.Expression.String()
	}
	return ""
}

// expressions

type Identifier struct {
	Token token.Token
	Value string
}

func (i *Identifier) String() string { return i.Token.Literal }

type IntegerLiteral struct {
	Token token.Token
	Value int64
}

func (il *IntegerLiteral) String() string { return il.Token.Literal }

type FloatLiteral struct {
	Token token.Token
	Value float64
}

func (fl *FloatLiteral) String() string { return fl.Token.Literal }

type BooleanLiteral struct {
	Token token.Token
	Value bool
}

func (bl *BooleanLiteral) String() string { return bl.Token.Literal }

type StringLiteral struct {
	Token token.Token
}

func (sl *StringLiteral) String() string { return `"` + sl.Token.Literal + `"` }

type NullLiteral struct {
	Token token.Token
}

func (nl *NullLiteral) String() string { return nl.Token.Literal }

type PrefixExpression struct {
	Token token.Token
	Right Expression
}

func (pe *PrefixExpression) String() string {
	var out bytes.Buffer

	out.WriteString("(")
	out.WriteString(pe.Token.Literal)
	out.WriteString(pe.Right.String())
	out.WriteString(")")

	return out.String()
}

type InfixExpression struct {
	Left  Expression
	Token token.Token
	Right Expression
}

func (ie *InfixExpression) String() string {
	var out bytes.Buffer

	out.WriteString("(")
	out.WriteString(ie.Left.String())
	out.WriteString(ie.Token.Literal)
	out.WriteString(ie.Right.String())
	out.WriteString(")")

	return out.String()
}

type IfExpression struct {
	Token     token.Token
	Condition Expression
	TruePart  *Program
	FalsePart *Program
}

func (ie *IfExpression) String() string {
	var out bytes.Buffer

	out.WriteString(ie.Token.Literal)
	out.WriteString("(")
	out.WriteString(ie.Condition.String())
	out.WriteString(")\n")
	out.WriteString(ie.TruePart.String())

	if ie.FalsePart != nil {
		out.WriteString("else\n")
		out.WriteString(ie.FalsePart.String())
	}

	out.WriteString("end")

	return out.String()
}

type WhileExpression struct {
	Token     token.Token
	Condition Expression
	Body      *Program
}

func (we *WhileExpression) String() string {
	var out bytes.Buffer

	out.WriteString(we.Token.Literal)
	out.WriteString("(")
	out.WriteString(we.Condition.String())
	out.WriteString(")\n")
	out.WriteString(we.Body.String())
	out.WriteString("end")

	return out.String()
}

type Parameter struct {
	Name  *Identifier
	Value Expression
}

type FunctionExpression struct {
	Token      token.Token
	Parameters []*Parameter
	Body       *Program
}

func (fe *FunctionExpression) String() string {
	var out bytes.Buffer

	out.WriteString(fe.Token.Literal)
	out.WriteString("(")
	for i, parameter := range fe.Parameters {
		out.WriteString(parameter.Name.String())

		if parameter.Value != nil {
			out.WriteString(" = " + parameter.Value.String())
		}

		if i != len(fe.Parameters)-1 {
			out.WriteString(", ")
		}

	}

	out.WriteString(")\n")
	out.WriteString(fe.Body.String())
	out.WriteString("end")

	return out.String()
}

type CallExpression struct {
	Function  Expression
	Arguments []Expression
}

func (ce *CallExpression) String() string {
	var out bytes.Buffer

	out.WriteString(ce.Function.String())
	out.WriteString("(")
	for i, arg := range ce.Arguments {
		out.WriteString(arg.String())

		if i != len(ce.Arguments)-1 {
			out.WriteString(", ")
		}

	}

	out.WriteString(")")

	return out.String()
}

type ClassExpression struct {
	Token  token.Token
	Parent Expression
	Body   []*Parameter
}

func (ce *ClassExpression) String() string {
	var out bytes.Buffer

	out.WriteString(ce.Token.Literal)

	if ce.Parent != nil {
		out.WriteString(" extends ")
		out.WriteString(ce.Parent.String())
	}
	out.WriteString("\n")

	for _, parameter := range ce.Body {
		out.WriteString(parameter.Name.String())
		if parameter.Value != nil {
			out.WriteString(" = ")
			out.WriteString(parameter.Value.String())
		}

		out.WriteString("\n")
	}

	out.WriteString("end")

	return out.String()
}
