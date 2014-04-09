
%{
  var Node = require('../node/node').Node
%}

%left ','
%right '='
%right '+=' '-='
%right '*=' '/='
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
  operator
| assign_statment
| function
;

operator:
  expression '+' expression { $$ = new Node.Plus($1, $3) }
| expression '-' expression { $$ = new Node.Minus($1, $3) }
| expression '*' expression { $$ = new Node.Multiply($1, $3) }
| expression '/' expression { $$ = new Node.Divide($1, $3) }
| '-' expression %prec UMINUS { $$ = new Node.Uminus($2) }
| factor
;

assign_statment:
  expression '=' expression { $$ = new Node.Assign($1, $3) }
| expression '+=' expression { $$ = new Node.Assign($1, new Node.Plus($1, $3)) }
| expression '-=' expression { $$ = new Node.Assign($1, new Node.Minus($1, $3)) }
| expression '*=' expression { $$ = new Node.Assign($1, new Node.Multiply($1, $3)) }
| expression '/=' expression { $$ = new Node.Assign($1, new Node.Divide($1, $3)) }
;

factor:
  NUMBER { $$ = new Node.Number($1) }
| STRING { $$ = new Node.String($1) }
| identifier
| factor '.' identifier { $$ = new Node.GetMember($1, $3) }
| function_call
;

identifier:
  IDENTIFIER { $$ = new Node.Identifier($1) }
;

function:
  FUNCTION identifier '(' parameter_names ')' expressions END { $$ = new Node.Assign($2, new Node.Function($6, $4)) }
;

parameter_names:
{ $$ = [] }
| identifier { $$ = [ $1 ] }
| parameter_names ',' identifier { $$ = $1.concat($3) }
;

function_call:
  factor '(' parameters ')' { $$ = new Node.FunctionCall($1, $3) }
;

parameters:
{ $$ = [] }
| factor { $$ = [ $1 ] }
| parameters ',' factor { $$ = $1.concat($3) }
;
