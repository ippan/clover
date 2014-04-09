Node = {}

class Node.Program
  constructor: (@expressions)->

class Node.BinaryOperation
  constructor: (@first, @second)->

  operator: ''

class Node.Assign extends Node.BinaryOperation
  operator: '='

class Node.Plus extends Node.BinaryOperation
  operator: '+'

class Node.Minus extends Node.BinaryOperation
  operator: '-'

class Node.Multiply extends Node.BinaryOperation
  operator: '*'

class Node.Divide extends Node.BinaryOperation
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

class Node.FunctionCall
  constructor: (@function, @parameters)->

class Node.GetMember
  constructor: (@instance, @member)->

require('./debug').apply Node

exports.Node = Node
