
%{
  var node = require('../node/node')
%}

%%

program:
  expressions EOF { return new node.Program($1) }
;

expressions:
{ $$ = [] }
| expression { $$ = [ $1 ] }
| expressions NEW_LINE expression { $$ = $1.concat($3) }
| expressions NEW_LINE { $$ = $1 }
;

expression:
  assign_statment
| factor
;

assign_statment:
  IDENTIFIER '=' factor { $$ = new node.Assign($1, $3) }
;

factor:
  NUMBER { $$ = $1 }
| STRING { $$ = $1 }
;
