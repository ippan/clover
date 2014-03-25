DIGIT [0-9]

%%

// comment
(\#).* // skip

// space
[ \r\t]+ // skip

[\n] return 'NEW_LINE'

// number
{DIGIT}+(\.{DIGIT}+)? return 'NUMBER'

// string
\"[^"]*\" return 'STRING'
\'[^']*\' return 'STRING'

// identifier
[a-zA-Z_][\w\_]* return 'IDENTIFIER'

// multi-character operator
//(\|\||&&|[<]=|[>]=|==|!=|\+=|-=|\*=|\/=) return yytext

// operator
. return yytext

<<EOF>> return 'EOF'
