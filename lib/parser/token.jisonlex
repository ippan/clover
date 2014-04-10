DIGIT [0-9]

%%

// comment
(\#).* // skip

// space
[ \r\t]+ // skip

\n return 'NEW_LINE'

// keyword
'function'
|'end'
|'if'
|'and'
|'or'
|'true'
|'false'
|'null'
|'class'
|'extends'
|'enum'
|'new'
|'while' return yytext.toUpperCase()

// number
{DIGIT}+(\.{DIGIT}+)? return 'NUMBER'

// string
\"[^"]*\" return 'STRING'
\'[^']*\' return 'STRING'

// identifier
[a-zA-Z_][\w\_]* return 'IDENTIFIER'

// multi-character operator
//(\|\||&&|[<]=|[>]=|==|!=|\+=|-=|\*=|\/=) return yytext

(\|\||\&\&|[<]\=|[>]\=|\=\=|\!\=|\+\=|\-\=|\*\=|\/\=) return yytext

// operator
. return yytext

<<EOF>> return 'EOF'
