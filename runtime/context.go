package runtime

type Context interface {
	Exists(key string) bool
	Get(key string) Object
	Set(key string, value Object)
}

type Environment struct {
	slots map[string]Object
}

func NewEnvironment() *Environment {
	return &Environment{slots: make(map[string]Object)}
}

func (e *Environment) Exists(key string) bool {
	_, ok := e.slots[key]
	return ok
}

func (e *Environment) Get(key string) Object {
	if value, ok := e.slots[key]; ok {
		return &ObjectBinding{Name: key, Value: value, BindingContext: e}
	}

	return &ObjectBinding{Name: key, Value: NULL, BindingContext: e}
}

func (e *Environment) Set(key string, value Object) {
	if binding, ok := value.(*ObjectBinding); ok {
		e.slots[key] = binding.UnWarp()
	} else {
		e.slots[key] = value
	}
}

type FunctionEnvironment struct {
	slots  map[string]Object
	parent Context
}

func NewFunctionEnvironment(parent Context) *FunctionEnvironment {
	return &FunctionEnvironment{slots: make(map[string]Object), parent: parent}
}

func (fe *FunctionEnvironment) Exists(key string) bool {
	_, ok := fe.slots[key]
	return ok || fe.parent.Exists(key)
}

func (fe *FunctionEnvironment) Get(key string) Object {
	if value, ok := fe.slots[key]; ok {
		return &ObjectBinding{Name: key, Value: value, BindingContext: fe}
	}

	if fe.parent.Exists(key) {
		return fe.parent.Get(key)
	}

	return &ObjectBinding{Name: key, Value: NULL, BindingContext: fe}
}

func (fe *FunctionEnvironment) Set(key string, value Object) {
	if fe.parent.Exists(key) {
		fe.parent.Set(key, value)
		return
	}

	if binding, ok := value.(*ObjectBinding); ok {
		fe.slots[key] = binding.UnWarp()
	} else {
		fe.slots[key] = value
	}
}
