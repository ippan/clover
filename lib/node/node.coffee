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

class Node.Mod extends Node.BinaryOperation
  op_method: 'op_mod'
  operator: '%'

class Node.Greater extends Node.BinaryOperation
  op_method: 'op_greater'
  operator: '>'

class Node.Less extends Node.BinaryOperation
  op_method: 'op_less'
  operator: '<'

class Node.Equal extends Node.BinaryOperation
  op_method: 'op_equal'
  operator: '=='

class Node.Hash
  constructor: (@key_values)->

class Node.KeyValue
  constructor: (@key, @value)->

class Node.And
  constructor: (@left, @right)->

class Node.Or
  constructor: (@left, @right)->

class Node.Null

class Node.Boolean
  constructor: (@boolean)->

class Node.Number
  constructor: (@number)->

class Node.String
  constructor: (@string, @translated = false)->

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

class Node.BaseGetMember
  constructor: (@member)->    

class Node.While
  constructor: (@condition, @expressions)->

class Node.IfElse
  constructor: (@condition, @true_part, @false_part)->

require('./debug').apply Node

exports.Node = Node
