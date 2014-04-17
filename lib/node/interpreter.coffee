apply = (Node, Runtime)->

  Node.Program::execute = (context)->

    context = new Runtime.GlobalContext() unless context?

    for expression in @expressions
      expression.execute context


  Node.Number::execute = (context)->
    new Runtime.Number(@number)

  
  Node.BinaryOperation::execute = (context)->
    @first.execute(context)[@op_method](@second.execute(context))


  Node.Identifier::execute = (context)->
    context.get @name


  Node.FunctionCall::execute = (context)->
    @function.call context, @parameters

exports.apply = apply
