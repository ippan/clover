parser = require('../lib/parser/parser').parser

describe 'Parser', ->

  it 'should ignore spaces', ->
    parser.parse('a=1').should.eql(parser.parse('a         =   1  '))

  it 'should ignore empty lines', ->
    parser.parse('a = 1').should.eql(parser.parse('\na = 1\n\n'))
