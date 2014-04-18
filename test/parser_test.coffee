clover = require('../lib/parser/parser')
parser = clover.parser
Node = clover.Node
Runtime = clover.Runtime

describe 'Parser', ->

  it 'should ignore spaces', ->
    parser.parse('a=1').should.eql(parser.parse('a         =   1  '))

  it 'should ignore empty lines', ->
    parser.parse('a = 1').should.eql(parser.parse('\na = 1\n\n'))

  it 'multiply > plus', ->
    parser.parse('3 + 2 * 1').should.eql(new Node.Program([new Node.Plus(new Node.Number('3'), new Node.Multiply(new Node.Number('2'), new Node.Number('1')))]))
    parser.parse('3 * 2 + 1').should.eql(new Node.Program([new Node.Plus(new Node.Multiply(new Node.Number('3'), new Node.Number('2')), new Node.Number('1'))]))



describe 'Interpreter', ->