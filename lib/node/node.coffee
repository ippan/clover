class Program
  constructor: (@nodes)->

  dump: ->
    for node in @nodes
      node.dump()

exports.Program = Program


class Assign
  constructor: (@first, @second)->

  dump: ->
    console.log "#{@first} = #{@second}"

exports.Assign = Assign


