apply = (Node, Runtime)->

  Node.Program::execute = (context)->

    context = new Runtime.GlobalContext() unless context?

    for expression in @expressions
      expression.execute context


  Node.Number::execute = (context)->
    new Runtime.Number(@number)

  Node.String::execute = (context)->



    new Runtime.String(@string.substring(1, @string.length - 1))

  
  Node.BinaryOperation::execute = (context)->
    @first.execute(context)[@op_method](@second.execute(context))


  Node.Identifier::execute = (context)->
    context.get @name

  Node.Function::execute = (context)->

    parameters = []
    # TODO : add defulat parameter support
    for parameter in @parameters
      parameters.push [ parameter.name, new Runtime.Nil() ]

    new Runtime.Function(context, @expressions, parameters)

  Node.FunctionCall::execute = (context)->
    parameters = []

    for parameter in @parameters
      parameters.push parameter.execute(context)

    @function.execute(context).call(parameters) || new Runtime.Nil()

exports.apply = apply
