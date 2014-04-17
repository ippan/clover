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


  class Runtime.Function extends Runtime.Object


  class Runtime.NativeFunction extends Runtime.Function

    call: (parameters)->


  # TODO : for test only, remove later
  class Runtime.PrintFunction extends Runtime.NativeFunction
    call: (context, parameters)->
      console.log parameters[0].to_string().string


exports.apply = apply