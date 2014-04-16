Runtime = {}

class Runtime.Object





class Runtime.Number extends Runtime.Object
  initialize: (@number = 0)->

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
  initialize: (@string = '')->


  op_plus: (target)->
    new Runtime.String("#{ @string }#{ target.to_string().string }")

  to_string: ->
    new Runtime.String(@string)


exports.Runtime = Runtime