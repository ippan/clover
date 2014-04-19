apply = (Node, Runtime)->

  Node.Program::execute = (context)->

    context = new Runtime.GlobalContext() unless context?

    for expression in @expressions
      expression.execute context


  Node.Number::execute = (context)->
    new Runtime.Number(@number)

  Node.String::execute = (context)->

    # TODO : translate string
    string = @translated && @string || @string.substring(1, @string.length - 1)

    new Runtime.String(string)

  
  Node.BinaryOperation::execute = (context)->
    first = @first.execute(context)
    second = @second.execute(context)

    if first.has_user_op(@op_method)
      op_function = first.get(@op_method)
      op_function.call([ second ])
    else
      first[@op_method](second)

  Node.Null::execute = (context)->
    new Runtime.Null() 

  Node.Boolean::execute = (context)->
    new Runtime.Boolean(@boolean)

  Node.Function::execute = (context)->

    parameters = []
    # TODO : add defulat parameter support
    for parameter in @parameters
      parameters.push [ parameter, new Runtime.Null() ]

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
    instance = (@instance? && @instance.execute(context)) || context
    instance.get @member.execute(context).runtime_value

  Node.BaseGetMember::execute = (context)->
    name = @member.execute(context).runtime_value
    context.scope_context.base.get(name).bind context.environment, name

  Node.IfElse::execute = (context)->
    # TODO : use block context
    if @condition.execute(context).to_bool().runtime_value
      for expression in @true_part
        expression.execute(context) 
    else if @false_part?
      for expression in @false_part
        expression.execute(context) 
        

exports.apply = apply
