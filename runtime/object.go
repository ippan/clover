package runtime

import (
	"bytes"
	"fmt"
	"github.com/ippan/clover/ast"
	"strconv"
)

type ObjectType string

const (
	TYPE_NULL   = "NULL"
	TYPE_ERROR  = "ERROR"
	TYPE_RETURN = "RETURN"

	TYPE_INTEGER = "INTEGER"
	TYPE_FLOAT   = "FLOAT"
	TYPE_BOOLEAN = "BOOLEAN"
	TYPE_STRING  = "STRING"

	TYPE_CLASS    = "CLASS"
	TYPE_INSTANCE = "INSTANCE"

	TYPE_FUNCTION = "FUNCTION"
)

var (
	NULL         = &Null{}
	TRUE         = &Boolean{Value: true}
	FALSE        = &Boolean{Value: false}
	EMPTY_STRING = &String{Value: ""}
	ONE          = &Integer{Value: 1}
	MINUS_ONE    = &Integer{Value: -1}
	ZERO         = &Integer{Value: 0}
)

type Object interface {
	Type() ObjectType
	Inspect() string

	// operators
	// +
	Add(other Object) Object
	// -
	Sub(other Object) Object
	// *
	Multiply(other Object) Object
	// /
	Divide(other Object) Object

	Equal(other Object) *Boolean
	Compare(other Object) *Integer

	ToInteger() *Integer
	ToFloat() *Float
	ToString() *String
	ToBoolean() *Boolean

	Not() *Boolean
	Negative() Object
}

type BaseObject struct {
}

func (bo *BaseObject) Add(other Object) Object {
	// TODO : raise error
	return nil
}
func (bo *BaseObject) Sub(other Object) Object {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) Multiply(other Object) Object {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) Divide(other Object) Object {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) ToInteger() *Integer {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) ToFloat() *Float {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) ToString() *String {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) Equal(other Object) *Boolean { return FALSE }
func (bo *BaseObject) Compare(other Object) *Integer {
	// TODO : raise error
	return nil
}

func (bo *BaseObject) ToBoolean() *Boolean { return TRUE }
func (bo *BaseObject) Not() *Boolean       { return bo.ToBoolean().Not() }
func (bo *BaseObject) Negative() Object {
	// TODO : raise error
	return nil
}

type Integer struct {
	BaseObject
	Value int64
}

func (i *Integer) Type() ObjectType { return TYPE_INTEGER }
func (i *Integer) Inspect() string  { return fmt.Sprintf("%d", i.Value) }

func (i *Integer) Add(other Object) Object {
	if other.Type() == TYPE_FLOAT {
		return i.ToFloat().Add(other)
	}

	if other.Type() == TYPE_INTEGER {
		return &Integer{Value: i.Value + other.ToInteger().Value}
	}

	// TODO : raise error
	return nil
}
func (i *Integer) Sub(other Object) Object {
	if other.Type() == TYPE_FLOAT {
		return i.ToFloat().Sub(other)
	}

	if other.Type() == TYPE_INTEGER {
		return &Integer{Value: i.Value - other.ToInteger().Value}
	}

	// TODO : raise error
	return nil
}
func (i *Integer) Multiply(other Object) Object {
	if other.Type() == TYPE_FLOAT {
		return i.ToFloat().Multiply(other)
	}

	if other.Type() == TYPE_INTEGER {
		return &Integer{Value: i.Value * other.ToInteger().Value}
	}

	// TODO : raise error
	return nil
}
func (i *Integer) Divide(other Object) Object {
	if other.Type() == TYPE_FLOAT {
		return i.ToFloat().Divide(other)
	}

	if other.Type() == TYPE_INTEGER {
		return &Integer{Value: i.Value / other.ToInteger().Value}
	}

	// TODO : raise error
	return nil
}

func (i *Integer) Equal(other Object) *Boolean {
	if other.Type() == TYPE_INTEGER {
		if i.Value == other.ToInteger().Value {
			return TRUE
		}
	}
	return FALSE
}

func (i *Integer) Compare(other Object) *Integer {
	if other.Type() == TYPE_FLOAT {
		return i.ToFloat().Compare(other)
	}

	if other.Type() != TYPE_INTEGER {
		// TODO : raise error
		return nil
	}

	value := i.Value - other.ToInteger().Value

	if value > 0 {
		return ONE
	} else if value < 0 {
		return MINUS_ONE
	} else {
		return ZERO
	}
}

func (i *Integer) Not() *Boolean { return i.ToBoolean().Not() }

func (i *Integer) Negative() Object {
	return &Integer{Value: -i.Value}
}

func (i *Integer) ToInteger() *Integer { return i }
func (i *Integer) ToFloat() *Float     { return &Float{Value: float64(i.Value)} }
func (i *Integer) ToString() *String   { return &String{Value: fmt.Sprintf("%d", i.Value)} }
func (i *Integer) ToBoolean() *Boolean {
	if i.Value == 0 {
		return FALSE
	}
	return TRUE
}

type Float struct {
	BaseObject
	Value float64
}

func (f *Float) Type() ObjectType             { return TYPE_FLOAT }
func (f *Float) Inspect() string              { return fmt.Sprintf("%f", f.Value) }
func (f *Float) Add(other Object) Object      { return &Float{Value: f.Value + other.ToFloat().Value} }
func (f *Float) Sub(other Object) Object      { return &Float{Value: f.Value - other.ToFloat().Value} }
func (f *Float) Multiply(other Object) Object { return &Float{Value: f.Value * other.ToFloat().Value} }
func (f *Float) Divide(other Object) Object   { return &Float{Value: f.Value / other.ToFloat().Value} }

func (f *Float) Equal(other Object) *Boolean {
	if other.Type() == TYPE_FLOAT {
		if f.Value == other.ToFloat().Value {
			return TRUE
		}
	}
	return FALSE
}

func (f *Float) Compare(other Object) *Integer {

	if other.Type() != TYPE_FLOAT && other.Type() != TYPE_INTEGER {
		// TODO : raise error
		return nil
	}

	value := f.Value - other.ToFloat().Value

	if value > 0.0 {
		return ONE
	} else if value < 0.0 {
		return MINUS_ONE
	} else {
		return ZERO
	}
}

func (f *Float) Not() *Boolean { return f.ToBoolean().Not() }

func (f *Float) Negative() Object {
	return &Float{Value: -f.Value}
}

func (f *Float) ToInteger() *Integer { return &Integer{Value: int64(f.Value)} }
func (f *Float) ToFloat() *Float     { return f }
func (f *Float) ToString() *String   { return &String{Value: fmt.Sprintf("%f", f.Value)} }
func (f *Float) ToBoolean() *Boolean {
	if f.Value == 0.0 {
		return FALSE
	}
	return TRUE
}

type String struct {
	BaseObject
	Value string
}

func (s *String) Type() ObjectType { return TYPE_STRING }
func (s *String) Inspect() string  { return fmt.Sprintf(`"%s"`, s.Value) }

func (s *String) Add(other Object) Object { return &String{Value: s.Value + other.ToString().Value} }

func (s *String) Equal(other Object) *Boolean {
	if otherString, ok := other.(*String); ok {
		if s.Value == otherString.Value {
			return TRUE
		}
	}
	return FALSE
}

func (s *String) ToInteger() *Integer {
	i, err := strconv.ParseInt(s.Value, 0, 64)

	if err != nil {
		// TODO : raise error
		return nil
	}

	return &Integer{Value: i}
}

func (s *String) ToFloat() *Float {
	f, err := strconv.ParseFloat(s.Value, 64)

	if err != nil {
		// TODO : raise error
		return nil
	}

	return &Float{Value: f}
}

func (s *String) ToString() *String { return s }

type Boolean struct {
	BaseObject
	Value bool
}

func (b *Boolean) Type() ObjectType { return TYPE_BOOLEAN }
func (b *Boolean) Inspect() string  { return fmt.Sprintf("%t", b.Value) }

func (b *Boolean) Equal(other Object) *Boolean {
	if other.Type() == TYPE_BOOLEAN {
		if b.Value == other.ToBoolean().Value {
			return TRUE
		}
	}
	return FALSE
}

func (b *Boolean) ToString() *String   { return &String{Value: fmt.Sprintf("%t", b.Value)} }
func (b *Boolean) ToBoolean() *Boolean { return b }
func (b *Boolean) Not() *Boolean {
	if b.Value == true {
		return FALSE
	}

	return TRUE
}

type Null struct {
	BaseObject
}

func (n *Null) Type() ObjectType { return TYPE_NULL }
func (n *Null) Inspect() string  { return "null" }

func (n *Null) ToString() *String   { return EMPTY_STRING }
func (n *Null) ToBoolean() *Boolean { return FALSE }
func (n *Null) Not() *Boolean       { return TRUE }

type ObjectBinding struct {
	Name           string
	Value          Object
	BindingContext Context
}

func (ob *ObjectBinding) Type() ObjectType              { return ob.Value.Type() }
func (ob *ObjectBinding) Inspect() string               { return ob.Value.Inspect() }
func (ob *ObjectBinding) Add(other Object) Object       { return ob.Value.Add(other) }
func (ob *ObjectBinding) Sub(other Object) Object       { return ob.Value.Sub(other) }
func (ob *ObjectBinding) Multiply(other Object) Object  { return ob.Value.Multiply(other) }
func (ob *ObjectBinding) Divide(other Object) Object    { return ob.Value.Divide(other) }
func (ob *ObjectBinding) Equal(other Object) *Boolean   { return ob.Value.Equal(other) }
func (ob *ObjectBinding) Compare(other Object) *Integer { return ob.Value.Compare(other) }
func (ob *ObjectBinding) ToInteger() *Integer           { return ob.Value.ToInteger() }
func (ob *ObjectBinding) ToFloat() *Float               { return ob.Value.ToFloat() }
func (ob *ObjectBinding) ToString() *String             { return ob.Value.ToString() }
func (ob *ObjectBinding) ToBoolean() *Boolean           { return ob.Value.ToBoolean() }
func (ob *ObjectBinding) Not() *Boolean                 { return ob.Value.Not() }
func (ob *ObjectBinding) Negative() Object              { return ob.Value.Negative() }
func (ob *ObjectBinding) UnWarp() Object {
	if binding, ok := ob.Value.(*ObjectBinding); ok {
		return binding.UnWarp()
	}
	return ob.Value
}

type Function struct {
	BaseObject
	BindingContext Context
	Parameters     []*ast.Parameter
	Body           *ast.Program
}

func (f *Function) Type() ObjectType { return TYPE_FUNCTION }
func (f *Function) Inspect() string {
	var out bytes.Buffer

	out.WriteString("function(")
	for i, parameter := range f.Parameters {
		out.WriteString(parameter.Name.String())

		if parameter.Value != nil {
			out.WriteString(" = " + parameter.Value.String())
		}

		if i != len(f.Parameters)-1 {
			out.WriteString(", ")
		}
	}

	out.WriteString(")\n")
	out.WriteString(f.Body.String())
	out.WriteString("end")

	return out.String()
}

type Return struct {
	BaseObject
	Value Object
}

func (r *Return) Type() ObjectType { return TYPE_RETURN }
func (r *Return) Inspect() string {
	return fmt.Sprintf("return %s", r.Value.Inspect())
}
