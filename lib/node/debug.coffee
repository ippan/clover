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

    for expression in @expressions
        expression.dump()
        process.stdout.write "\n"
    process.stdout.write "end"


exports.apply = apply
