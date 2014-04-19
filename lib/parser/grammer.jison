
%{
  var Node = require('../node/node').Node
  exports.Node = Node

  var Runtime = require('../runtime/runtime').Runtime
  require('../node/interpreter').apply(Node, Runtime)

  exports.Runtime = Runtime
%}

%left ','
%right '='
%right '+=' '-='
%right '*=' '/='
%left OR
%left AND
%left '>' '<' 
%left '+' '-'
%left '*' '/'
%left '.'
%nonassoc UMINUS

%%

program:
  expressions EOF { return new Node.Program($1) }
;

expressions:
{ $$ = [] }
| expression { $$ = [ $1 ] }
| expressions NEW_LINE expression { $$ = $1.concat($3) }
| expressions NEW_LINE { $$ = $1 }
;

expression:
  literal
| new_class
| function_call
| operator
| factor
| assign_statment
| if_statment
| function
| class
| hash
| '(' expression ')' { $$ = $2 }
;

literal:
  NUMBER { $$ = new Node.Number(Number($1)) }
| STRING { $$ = new Node.String($1) }
| NULL { $$ = new Node.Null() }
| boolean
;

operator:
  expression AND expression { $$ = new Node.And($1, $3) }
| expression OR expression { $$ = new Node.Or($1, $3) }
| expression '>' expression { $$ = new Node.Greater($1, $3) }
| expression '<' expression { $$ = new Node.Less($1, $3) }
| expression '+' expression { $$ = new Node.Plus($1, $3) }
| expression '-' expression { $$ = new Node.Minus($1, $3) }
| expression '*' expression { $$ = new Node.Multiply($1, $3) }
| expression '/' expression { $$ = new Node.Divide($1, $3) }
| '-' expression %prec UMINUS { $$ = new Node.Uminus($2) }
;

hash:
  '{' key_values '}' { $$ = new Node.Hash($2) }
;

key_values:
{ $$ = [] }
| key_value { $$ = [ $1 ] }
| key_values ',' key_value { $$ = $1.concat($3) }
| key_values ',' { $$ = $1 }
| key_values NEW_LINE key_value { $$ = $1.concat($3) }
| key_values NEW_LINE { $$ = $1 }
;

key_value:
  IDENTIFIER ':' expression { $$ = new Node.KeyValue($1, $3) }
;

if_statment:
  IF expression NEW_LINE expressions else_part END { $$ = new Node.IfElse($2, $4, $5) }
;

else_part:
{ $$ = null }
| ELSE expressions { $$ = $2 }
;

assign_statment:
  expression '=' expression { $$ = new Node.Assign($1, $3) }
| expression '+=' expression { $$ = new Node.Assign($1, new Node.Plus($1, $3)) }
| expression '-=' expression { $$ = new Node.Assign($1, new Node.Minus($1, $3)) }
| expression '*=' expression { $$ = new Node.Assign($1, new Node.Multiply($1, $3)) }
| expression '/=' expression { $$ = new Node.Assign($1, new Node.Divide($1, $3)) }
;

factor:
  IDENTIFIER { $$ = new Node.GetMember(null, new Node.String($1, true)) }
| BASE '.' IDENTIFIER { $$ = new Node.BaseGetMember(new Node.String($3, true)) }
| factor '.' IDENTIFIER { $$ = new Node.GetMember($1, new Node.String($3, true)) }
| factor '[' expression ']' { $$ = new Node.GetMember($1, $3) }
;

boolean:
  TRUE { $$ = new Node.Boolean(true) }
| FALSE { $$ = new Node.Boolean(false) }
;

function:
  FUNCTION '(' parameter_names ')' expressions END { $$ = new Node.Function($5, $3) }
;

class:
  CLASS expressions END { $$ = new Node.Class($2) }
| CLASS EXTENDS factor expressions END { $$ = new Node.Class($4, $3) }
;

parameter_names:
{ $$ = [] }
| IDENTIFIER { $$ = [ $1 ] }
| parameter_names ',' IDENTIFIER { $$ = $1.concat($3) }
;

function_call:
  factor '(' parameters ')' { $$ = new Node.FunctionCall($1, $3) }
;

new_class:
  factor '.' NEW '(' parameters ')' { $$ = new Node.NewClass($1, $5) }
;

parameters:
{ $$ = [] }
| expression { $$ = [ $1 ] }
| parameters ',' expression { $$ = $1.concat($3) }
;
