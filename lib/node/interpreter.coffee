apply = (Node, Context)->

  Node.Program::execute(context) = ->
    for expresstion in @expresstions
      expresstion.execute context











exports.apply = apply
