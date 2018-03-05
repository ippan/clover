package runtime

type Context interface {
	Exists(key string) bool
	InstanceExists(key string) bool
	Get(key string) Object
	InstanceGet(key string) Object
	Set(key string, value Object)
	InstanceSet(key string, value Object)
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

func (e *Environment) InstanceExists(key string) bool { return e.Exists(key) }

func (e *Environment) Get(key string) Object {
	if value, ok := e.slots[key]; ok {
		return &ObjectBinding{Name: key, Value: value, BindingContext: e}
	}

	return &ObjectBinding{Name: key, Value: NULL, BindingContext: e}
}

func (e *Environment) InstanceGet(key string) Object { return e.Get(key) }

func (e *Environment) Set(key string, value Object) {
	if binding, ok := value.(*ObjectBinding); ok {
		e.slots[key] = binding.UnWarp()
	} else {
		e.slots[key] = value
	}
}

func (e *Environment) InstanceSet(key string, value Object) { e.Set(key, value) }

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

func (fe *FunctionEnvironment) InstanceExists(key string) bool { return fe.Exists(key) }

func (fe *FunctionEnvironment) Get(key string) Object {
	if value, ok := fe.slots[key]; ok {
		return &ObjectBinding{Name: key, Value: value, BindingContext: fe}
	}

	if fe.parent.Exists(key) {
		return fe.parent.Get(key)
	}

	return &ObjectBinding{Name: key, Value: NULL, BindingContext: fe}
}

func (fe *FunctionEnvironment) InstanceGet(key string) Object {
	if value, ok := fe.slots[key]; ok {
		return &ObjectBinding{Name: key, Value: value, BindingContext: fe}
	}
	return &ObjectBinding{Name: key, Value: NULL, BindingContext: fe}
}

func (fe *FunctionEnvironment) Set(key string, value Object) {
	if fe.parent.Exists(key) {
		fe.parent.Set(key, value)
		return
	}

	fe.InstanceSet(key, value)
}

func (fe *FunctionEnvironment) InstanceSet(key string, value Object) {
	if binding, ok := value.(*ObjectBinding); ok {
		fe.slots[key] = binding.UnWarp()
	} else {
		fe.slots[key] = value
	}
}
