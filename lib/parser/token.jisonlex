DIGIT [0-9]
KEYWORD 'function'|'end'|'if'|'else'|'elseif'|'and'|'or'|'true'|'false'|'null'|'class'|'extends'|'new'|'base'|'while'

%%

// comment
(\#).* // skip

// space
[ \r\t]+ // skip

\n return 'NEW_LINE'

{KEYWORD}+[\w\_]+ return 'IDENTIFIER'

// keyword
{KEYWORD} return yytext.toUpperCase()

// number
{DIGIT}+(\.{DIGIT}+)? return 'NUMBER'

// string
\"[^"]*\" return 'STRING'
\'[^']*\' return 'STRING'

// identifier
[a-zA-Z\_][\w\_]* return 'IDENTIFIER'

'||' return 'OR'
'&&' return 'AND'

// multi-character operator
([<]\=|[>]\=|\=\=|\!\=|\+\=|\-\=|\*\=|\/\=) return yytext

// operator
. return yytext

<<EOF>> return 'EOF'
