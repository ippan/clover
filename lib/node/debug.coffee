dump_expressions = (expressions)->
  for expression in expressions
    expression.dump()
    process.stdout.write "\n"

apply = (Node)->
  Node.Program::dump = ->
    for expression in @expressions
      expression.dump()
      process.stdout.write "\n"

  Node.BinaryOperation::dump = ->
    process.stdout.write '('
    @first.dump()
    process.stdout.write ") #{@operator} ("
    @second.dump()
    process.stdout.write ')'

  Node.Identifier::dump = ->
    process.stdout.write @name

  Node.Null::dump = ->
    process.stdout.write 'null'

  Node.Boolean::dump = ->
    process.stdout.write @boolean.toString()

  Node.Number::dump = ->
    process.stdout.write @number

  Node.String::dump = ->
    process.stdout.write @string

  Node.Uminus::dump = ->
    process.stdout.write '-'
    @node.dump()


  Node.Function::dump = ->
    process.stdout.write "function ("

    for parameter, i in @parameters
      parameter.dump()
      process.stdout.write(', ') if i < @parameters.length - 1

    process.stdout.write ")\n"
    dump_expressions @expressions        
    process.stdout.write "end"

  Node.Class::dump = ->
    process.stdout.write 'class'

    if @extends
      process.stdout.write ' extends '
      @extends.dump()

    process.stdout.write "\n"
    dump_expressions @expressions
    process.stdout.write "end"

  Node.GetMember::dump = ->
    @instance.dump()
    process.stdout.write '.'
    @member.dump()

  Node.BaseGetMember::dump = ->
    process.stdout.write 'base.'
    @member.dump()

  Node.FunctionCall::dump = ->
    @function.dump()
    process.stdout.write '('

    for parameter, i in @parameters
      parameter.dump()
      process.stdout.write(', ') if i < @parameters.length - 1

    process.stdout.write ')'

  Node.IfElse::dump = ->
    process.stdout.write 'if '
    @condition.dump()
    process.stdout.write "\n"
    dump_expressions @true_part
    process.stdout.write "else\n"
    dump_expressions @false_part

  Node.NewClass::dump = ->
    @class.dump()
    process.stdout.write '.new('
    for parameter, i in @parameters
      parameter.dump()
      process.stdout.write(', ') if i < @parameters.length - 1
    process.stdout.write ')'
exports.apply = apply
