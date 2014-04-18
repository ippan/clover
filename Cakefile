{ spawn, exec } = require('child_process')

task "test", "test", ->
  child_process = spawn 'mocha', [], { stdio: 'inherit' }

task "build", "", ->
  child_process = spawn 'jison', ['./lib/parser/grammer.jison', './lib/parser/token.jisonlex', '-o', './lib/parser/parser.js'], { stdio: 'inherit' }
