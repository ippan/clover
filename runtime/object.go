package runtime

import (
	"fmt"
	"strconv"
)

type ObjectType string

const (
	TYPE_NULL  = "NULL"
	TYPE_ERROR = "ERROR"

	TYPE_INTEGER = "INTEGER"
	TYPE_FLOAT   = "FLOAT"
	TYPE_BOOLEAN = "BOOLEAN"
	TYPE_STRING  = "STRING"

	TYPE_CLASS    = "CLASS"
	TYPE_INSTANCE = "INSTANCE"

	TYPE_FUNCTION = "FUNCTION"
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
func (bo *BaseObject) ToBoolean() *Boolean         { return TRUE }
func (bo *BaseObject) Not() *Boolean               { return bo.ToBoolean().Not() }
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
	if _, ok := other.(*Float); ok {
		return i.ToFloat().Add(other)
	}

	return &Integer{Value: i.Value + other.ToInteger().Value}
}
func (i *Integer) Sub(other Object) Object {
	if _, ok := other.(*Float); ok {
		return i.ToFloat().Sub(other)
	}

	return &Integer{Value: i.Value - other.ToInteger().Value}
}
func (i *Integer) Multiply(other Object) Object {
	if _, ok := other.(*Float); ok {
		return i.ToFloat().Multiply(other)
	}

	return &Integer{Value: i.Value * other.ToInteger().Value}
}
func (i *Integer) Divide(other Object) Object {
	if _, ok := other.(*Float); ok {
		return i.ToFloat().Divide(other)
	}

	return &Integer{Value: i.Value / other.ToInteger().Value}
}

func (i *Integer) Equal(other Object) *Boolean {
	if otherInteger, ok := other.(*Integer); ok {
		if i.Value == otherInteger.Value {
			return TRUE
		}
	}
	return FALSE
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
	if otherFloat, ok := other.(*Float); ok {
		if f.Value == otherFloat.Value {
			return TRUE
		}
	}
	return FALSE
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
	if otherBoolean, ok := other.(*Boolean); ok {
		if b.Value == otherBoolean.Value {
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

var (
	TRUE  = &Boolean{Value: true}
	FALSE = &Boolean{Value: false}
)
