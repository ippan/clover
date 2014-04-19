apply = (Node, Runtime)->

  Node.Program::execute = (context)->

    context = new Runtime.GlobalContext() unless context?

    for expression in @expressions
      expression.execute context


  Node.Number::execute = (context)->
    new Runtime.Number(@number)

  Node.String::execute = (context)->

    # TODO : translate string

    new Runtime.String(@string.substring(1, @string.length - 1))

  
  Node.BinaryOperation::execute = (context)->
    @first.execute(context)[@op_method](@second.execute(context))


  Node.Identifier::execute = (context)->
    context.get @name

  Node.Null::execute = (context)->
    new Runtime.Null() 

  Node.Boolean::execute = (context)->
    new Runtime.Boolean(@boolean)

  Node.Function::execute = (context)->

    parameters = []
    # TODO : add defulat parameter support
    for parameter in @parameters
      parameters.push [ parameter.name, new Runtime.Null() ]

    new Runtime.Function(context, @expressions, parameters)

  Node.FunctionCall::execute = (context)->
    parameters = []

    for parameter in @parameters
      parameters.push parameter.execute(context)

    @function.execute(context).call(parameters) || new Runtime.Null()


  Node.Class::execute = (context)->
    new Runtime.Class(context, @expressions, @extends)

  Node.NewClass::execute = (context)->
    class_object = @class.execute(context)
    instance = new Runtime.Instance(class_object, context)

    initialize = instance.context.try_get 'initialize'

    if initialize

      initialize.bind instance.context, 'initialize'

      parameters = []

      for parameter in @parameters
        parameters.push parameter.execute(context)

      initialize.call parameters

    instance
    

  Node.GetMember::execute = (context)->

    instance = @instance.execute(context)
    instance.get @member.name

  Node.BaseGetMember::execute = (context)->
    context.scope_context.base.get(@member.name).bind context.environment, @member.name

  Node.IfElse::execute = (context)->
    # TODO : use block context
    if @condition.execute(context).to_bool().boolean
      for expression in @true_part
        expression.execute(context) 
    else if @false_part?
      for expression in @false_part
        expression.execute(context) 
        

exports.apply = apply
