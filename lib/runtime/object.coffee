apply = (Runtime)->

  class Runtime.Object

    op_assign: (target)->
      if @identifier? and @context?
        @context.set(@identifier, target)

    bind: (@context, @identifier)->
      this

  class Runtime.Nil extends Runtime.Object



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

    to_string: ->
      new Runtime.String(@number.toString())


  class Runtime.String extends Runtime.Object
    constructor: (@string = '')->

    op_plus: (target)->
      new Runtime.String("#{ @string }#{ target.to_string().string }")

    to_string: ->
      new Runtime.String(@string)


  class Runtime.Callable extends Runtime.Object
    call: (parameters)->


  class Runtime.NativeFunction extends Runtime.Callable

    call: (parameters)->


  # TODO : for test only, remove later
  class Runtime.PrintFunction extends Runtime.NativeFunction
    call: (parameters)->
      console.log parameters[0].to_string().string
      null


  class Runtime.Function extends Runtime.Callable
    constructor: (@scrope_context, @expressions, @parameters)->
      @global_context = @scrope_context.global_context || @scrope_context

    call: (parameters)->
      function_context = new Runtime.FunctionContext(@global_context, @scrope_context)

      i = 0
      for parameter in @parameters
        function_context.set_local parameter[0], parameters[i] || parameter[1]
        i += 1

      result = null
      for expression in @expressions
        result = expression.execute function_context

      result


exports.apply = apply