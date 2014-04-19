apply = (Runtime)->

  class Runtime.Object

    op_assign: (target)->
      if @identifier? and @environment?
        @environment.set(@identifier, target)

    bind: (@environment, @identifier)->
      this

    to_string: ->
      new Runtime.String('object')

    get: (name)->
      this[name]

    to_bool: ->
      new Runtime.Boolean(true)

  class Runtime.Null extends Runtime.Object
    to_bool: ->
      new Runtime.Boolean(false)

    to_string: ->
      new Runtime.String('null')

  class Runtime.Boolean extends Runtime.Object
    constructor: (@boolean = true)->

    to_string: ->
      new Runtime.String(@boolean.toString())

    to_bool: ->
      this

  class Runtime.Number extends Runtime.Object
    constructor: (@number = 0)->

    op_plus: (target)->
      new Runtime.Number(@number + target.number)

    op_minus: (target)->
      new Runtime.Number(@number - target.number)

    op_multiply: (target)->
      new Runtime.Number(@number * target.number)

    op_divide: (target)->
      new Runtime.Number(@number / target.number)  

    op_greater: (target)->      
      new Runtime.Boolean(@number > target.number)

    op_less: (target)->
      new Runtime.Boolean(@number < target.number)

    to_string: ->
      new Runtime.String(@number.toString())  


  class Runtime.String extends Runtime.Object
    constructor: (@string = '')->

    op_plus: (target)->
      new Runtime.String("#{ @string }#{ target.to_string().string }")

    to_string: ->
      this


  class Runtime.Callable extends Runtime.Object
    call: (parameters)->


  class Runtime.NativeFunction extends Runtime.Callable

    call: (parameters)->


  # TODO : for test only, remove later
  class Runtime.PrintFunction extends Runtime.NativeFunction
    call: (parameters)->
      console.log parameters[0].to_string().string
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

  class Runtime.Class extends Runtime.Object
    constructor: (context, @expressions, @extends)->
      parent = @extends && @extends.execute(context)
      parent_context = parent.context if parent

      @context = new Runtime.ClassContext(context.global_context || context, parent_context)
      @context.building = true
      expression.execute(@context) for expression in @expressions
      @context.building = false


  class Runtime.Instance extends Runtime.Object
    constructor: (@class, context)->
      @context = new Runtime.InstanceContext(@class.context, context.global_context || context)

    get: (name)->
      @context.get name


exports.apply = apply