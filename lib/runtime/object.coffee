apply = (Runtime)->

  class Runtime.Object

    # TODO : maybe change later, will not store @identifier
    op_assign: (target)->
      if @identifier? and @environment?
        @environment.set(@identifier, target)
      else
        throw 'can not assign'

    op_equal: (target)->
      new Runtime.Boolean(this == target)

    bind: (@environment, @identifier)->
      this

    to_string: ->
      new Runtime.String('object')

    get: (name)->
      value = this[name]
      if typeof(value) == 'function'
        value = value.bind(this)()
      ((value instanceof Runtime.Object) and value or new Runtime.Null()).bind(this, name)

    to_bool: ->
      new Runtime.Boolean(true)

    has_user_op: (name)->
      false

    string: ->
      new Runtime.NativeFunction (parameters)=>
        @to_string()

    bool: ->
      new Runtime.NativeFunction (parameters)=>
        @to_bool()

  class Runtime.Null extends Runtime.Object
    to_bool: ->
      new Runtime.Boolean(false)

    to_string: ->
      new Runtime.String('null')

    op_equal: (target)->
      new Runtime.Boolean(target instanceof Runtime.Null)

  class Runtime.Boolean extends Runtime.Object
    constructor: (@runtime_value = true)->

    to_string: ->
      new Runtime.String(@runtime_value.toString())

    to_bool: ->
      this

    op_equal: (target)->
      new Runtime.Boolean(this.runtime_value == target.runtime_value)

  class Runtime.Number extends Runtime.Object
    constructor: (@runtime_value = 0)->

    op_plus: (target)->
      new Runtime.Number(@runtime_value + target.runtime_value)

    op_minus: (target)->
      new Runtime.Number(@runtime_value - target.runtime_value)

    op_multiply: (target)->
      new Runtime.Number(@runtime_value * target.runtime_value)

    op_divide: (target)->
      new Runtime.Number(@runtime_value / target.runtime_value) 

    op_mod: (target)->
      new Runtime.Number(@runtime_value % target.runtime_value) 

    op_greater: (target)->      
      new Runtime.Boolean(@runtime_value > target.runtime_value)

    op_less: (target)->
      new Runtime.Boolean(@runtime_value < target.runtime_value)

    op_equal: (target)->
      new Runtime.Boolean(this.runtime_value == target.runtime_value)

    op_uminus: ->
      new Runtime.Number(-@runtime_value)

    to_string: ->
      new Runtime.String(@runtime_value.toString())  


  class Runtime.String extends Runtime.Object
    constructor: (@runtime_value = '')->

    op_plus: (target)->
      new Runtime.String("#{ @runtime_value }#{ target.to_string().runtime_value }")

    to_string: ->
      this


  class Runtime.Callable extends Runtime.Object
    call: (parameters)->


  class Runtime.NativeFunction extends Runtime.Callable
    constructor: (@runtime_value)->

    call: (parameters)->
      @runtime_value parameters if @runtime_value

  # TODO : for test only, remove later
  class Runtime.PrintFunction extends Runtime.NativeFunction
    call: (parameters)->
      console.log parameters[0].to_string().runtime_value
      null

  class Runtime.DumpFunction extends Runtime.NativeFunction
    call: (parameters)->
      console.log parameters[0]
      null

  class Runtime.Function extends Runtime.Callable
    constructor: (@scope_context, @expressions, @parameters)->
      @global_context = @scope_context.global_context || @scope_context

    call: (parameters)->
      function_context = new Runtime.FunctionContext(@global_context, @scope_context, @environment)

      i = 0
      for parameter in @parameters
        function_context.set_local parameter[0], parameters[i] || parameter[1]
        i += 1

      result = null
      for expression in @expressions
        result = expression.execute function_context

      result

  # TODO : create a native object binding object
  class Runtime.NativeObject extends Runtime.Object
    constructor: (@runtime_value)->

    get: (name)->
      value = @runtime_value[name]


  class Runtime.Hash extends Runtime.Object
    constructor: (context, key_values)->
      @context = new Runtime.HashContext(context.global_context || context)
      for key_value in key_values
        @context.set_local key_value.key, key_value.value.execute(context)

    get: (name)->
      @context.get name


  class Runtime.Class extends Runtime.Object
    constructor: (context, @expressions, @extends)->
      parent = @extends && @extends.execute(context)
      parent_context = parent.context if parent

      @context = new Runtime.ClassContext(context.global_context || context, parent_context)
      @context.building = true
      expression.execute(@context) for expression in @expressions
      @context.building = false

    has_user_op: (name)->
      @context.has_local name


  class Runtime.Instance extends Runtime.Object
    constructor: (@class, context)->
      @context = new Runtime.InstanceContext(@class.context, context.global_context || context)

    get: (name)->
      (@context.try_get(name) || super(name)).bind(@context, name)

    has_user_op: (name)->
      @context.has_local(name)



exports.apply = apply