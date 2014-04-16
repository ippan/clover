Node = {}

class Node.Program
  constructor: (@expressions)->

class Node.BinaryOperation
  constructor: (@first, @second)->

  operator: ''

class Node.Assign extends Node.BinaryOperation
  op_method: 'op_assign'
  operator: '='

class Node.Plus extends Node.BinaryOperation
  op_method: 'op_plus'
  operator: '+'

class Node.Minus extends Node.BinaryOperation
  op_method: 'op_minus'
  operator: '-'

class Node.Multiply extends Node.BinaryOperation
  op_method: 'op_multiply'
  operator: '*'

class Node.Divide extends Node.BinaryOperation
  op_method: 'op_divide'
  operator: '/'

class Node.Identifier
  constructor: (@name)->

class Node.Number
  constructor: (@number)->

class Node.String
  constructor: (@string)->

class Node.Uminus
  constructor: (@node)->

class Node.Function
  constructor: (@expressions, @parameters)->

class Node.Class
  constructor: (@expressions, @extends)->

class Node.FunctionCall
  constructor: (@function, @parameters)->

class Node.NewClass
  constructor: (@class, @parameters)->

class Node.GetMember
  constructor: (@instance, @member)->

require('./debug').apply Node

exports.Node = Node
