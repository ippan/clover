runtime = {}

merge = (source, target)->
  for key, value of source
    target[key] = value

require('./context').apply runtime
require('./object').apply runtime

exports.Runtime = runtime