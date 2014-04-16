runtime = {}

merge = (source, target)->
  for key, value of source
    target[key] = value

merge require('./context').Runtime, runtime
merge require('./object').Runtime, runtime

exports.Runtime = runtime