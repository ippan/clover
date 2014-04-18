apply = (Runtime)->

  class Runtime.Context
    constructor: ->
      @locals = {}

    try_get: (name)->
      if name of @locals
        @locals[name].bind(this, name)

    try_set: (name, value)->
      if name of @locals
        @locals[name] = value

    get: (name)->
      @try_get(name) || (new Runtime.Nil()).bind(this, name)

    set: (name, value)->
      @try_set(name, value) || @set_local(name, value)

    set_local: (name, value)->
      @locals[name] = value

  class Runtime.GlobalContext extends Runtime.Context
    constructor: ->
      super()
      # TODO : for test only, remove later
      @set 'print', new Runtime.PrintFunction()



  class Runtime.ClassContext extends Runtime.Context
    constructor: (@global_context)->
      super()

    try_get: (name)->
      super(name) || (@global_context? && @global_context.try_get(name))

    try_set: (name, value)->
      super(name, value) || (@global_context? && @global_context.try_set(name, value))

  class Runtime.FunctionContext extends Runtime.Context
    constructor: (@global_context, @scope_context)->
      super()

    try_get: (name)->
      super(name) || (@scope_context? && @scope_context.try_get(name)) || (@global_context? && @global_context.try_get(name))

    try_set: (name, value)->
      super(name, value) || (@class_context? && @class_context.try_set(name, value)) || (@global_context? && @global_context.try_set(name, value))










exports.apply = apply