
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
| '(' expression ')' { $$ = $2 }
;

literal:
  NUMBER { $$ = new Node.Number(Number($1)) }
| STRING { $$ = new Node.String($1) }
| NULL { $$ = new Node.Null() }
| boolean
;

operator:
  expression '>' expression { $$ = new Node.Greater($1, $3) }
| expression '<' expression { $$ = new Node.Less($1, $3) }
| expression '+' expression { $$ = new Node.Plus($1, $3) }
| expression '-' expression { $$ = new Node.Minus($1, $3) }
| expression '*' expression { $$ = new Node.Multiply($1, $3) }
| expression '/' expression { $$ = new Node.Divide($1, $3) }
| '-' expression %prec UMINUS { $$ = new Node.Uminus($2) }
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
  identifier
| BASE '.' identifier { $$ = new Node.BaseGetMember($3) }
| factor '.' identifier { $$ = new Node.GetMember($1, $3) }
;

boolean:
  TRUE { $$ = new Node.Boolean(true) }
| FALSE { $$ = new Node.Boolean(false) }
;

identifier:
  IDENTIFIER { $$ = new Node.Identifier($1) }
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
| identifier { $$ = [ $1 ] }
| parameter_names ',' identifier { $$ = $1.concat($3) }
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
