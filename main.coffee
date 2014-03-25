parser = require('./lib/parser/parser').parser

code = "a = 2\nb = 3"

console.log code

node = parser.parse(code)
node.dump()
