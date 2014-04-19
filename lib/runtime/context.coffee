apply = (Runtime)->

  class Runtime.Context
    constructor: ->
      @locals = {}

    has_local: (name)->
      name of @locals

    try_get: (name)->

      if @has_local(name)
        @locals[name]

    try_set: (name, value)->
      if name of @locals
        @set_local name, value

    get: (name)->
      (@try_get(name) || new Runtime.Null()).bind(this, name)

    set: (name, value)->
      @try_set(name, value) || @set_local(name, value)

    set_local: (name, value)->
      @locals[name] = value

  class Runtime.GlobalContext extends Runtime.Context
    constructor: ->
      super()
      # TODO : for test only, remove later
      @set 'print', new Runtime.PrintFunction()
      @set 'dump', new Runtime.DumpFunction()


  class Runtime.ClassContext extends Runtime.Context
    constructor: (@global_context, @base)->
      super()
      @building = true

    has_local: (name)->
      super(name) || (@base? && @base.has_local(name))

    try_get: (name)->      
      super(name) || (@base? && @base.try_get(name)) || (@building && @global_context.try_get(name))

  class Runtime.InstanceContext extends Runtime.Context
    constructor: (@class_context, @global_context)->
      super()

    has_local: (name)->
      super(name) || @class_context.has_local(name)

    try_get: (name)->
      super(name) || @class_context.try_get(name)

    try_set: (name, value)->
      @set_local(name, value) if @has_local(name) || @class_context.has_local(name)

  class Runtime.HashContext extends Runtime.Context
    constructor: (@global_context)->
      super()

  class Runtime.FunctionContext extends Runtime.Context
    constructor: (@global_context, @scope_context, @environment)->
      super()

    try_get: (name)->
      super(name) || @environment.try_get(name) || @scope_context.try_get(name) || @global_context.try_get(name)

    try_set: (name, value)->
      super(name, value) || @environment.try_set(name, value) || (@scope_context? && @scope_context.try_set(name, value)) || (@global_context? && @global_context.try_set(name, value))

  class Runtime.BlockContext extends Runtime.Context
    constructor: (@scope_context)->
      try_get: (name)->
        super(name) || @scope_context.try_get(name)

      try_set: (name, value)->
        super(name, value) || @scope_context.try_set(name, value)









exports.apply = apply