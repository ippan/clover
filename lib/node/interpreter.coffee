apply = (Node, Runtime)->

  Node.Program::execute(context) = ->
    for expresstion in @expresstions
      expresstion.execute context











exports.apply = apply
