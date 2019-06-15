using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Transactions;
using Clover.Ast;

namespace Clover
{
    
    public interface FileLoader
    {
        String LoadFile(String base_filename, String filename);
    }

    public class DefaultFileLoader : FileLoader
    {
        public string LoadFile(string base_filename, string filename)
        {
            if (String.IsNullOrEmpty(base_filename))
                return File.ReadAllText(filename);

            return string.Empty;
        }
    }

    public class Parser
    {
        private delegate Expression PrefixFunction();
        private delegate Expression InfixFunction(Expression expression);
        
        private enum SymbolPriority
        {
            Lowest = 0,
            Assign,
            Boolean,
            Equals,
            LessGreater,
            Sum,
            Product,
            Prefix,
            Call,
            InstanceGet
        }

        private static readonly Dictionary<Token, SymbolPriority> precedences = new Dictionary<Token, SymbolPriority>
        {
            { Token.Assign, SymbolPriority.Assign },
            { Token.PlusAssign, SymbolPriority.Assign },
            { Token.MinusAssign, SymbolPriority.Assign },
            { Token.StarAssign, SymbolPriority.Assign },
            { Token.SlashAssign, SymbolPriority.Assign },
            { Token.And, SymbolPriority.Boolean },
            { Token.Or, SymbolPriority.Boolean },
            { Token.Equal, SymbolPriority.Equals },
            { Token.NotEqual, SymbolPriority.Equals },
            { Token.Less, SymbolPriority.LessGreater },
            { Token.Greater, SymbolPriority.LessGreater },
            { Token.LessEqual, SymbolPriority.LessGreater },
            { Token.GreaterEqual, SymbolPriority.LessGreater },
            { Token.Plus, SymbolPriority.Sum },
            { Token.Minus, SymbolPriority.Sum },
            { Token.Star, SymbolPriority.Product },
            { Token.Slash, SymbolPriority.Product },
            { Token.BitAnd, SymbolPriority.Product },
            { Token.BitOr, SymbolPriority.Product },
            { Token.Dot, SymbolPriority.InstanceGet },
            { Token.LeftBracket, SymbolPriority.InstanceGet },
            { Token.LeftParentheses, SymbolPriority.Call }
        };
        
        private Lexer lexer;

        private readonly FileLoader file_loader;

        private readonly Dictionary<Token, PrefixFunction> prefix_functions = new Dictionary<Token, PrefixFunction>();
        private readonly Dictionary<Token, InfixFunction> infix_functions = new Dictionary<Token, InfixFunction>();

        private TokenData CurrentTokenData;
        private TokenData PeekTokenData;
        
        public Parser(FileLoader loader = null)
        {
            file_loader = loader ?? new DefaultFileLoader();

            prefix_functions[Token.Identifier] = ParseIdentifier;
            prefix_functions[Token.Base] = ParseKeywordLiteral;
            prefix_functions[Token.This] = ParseKeywordLiteral;
            prefix_functions[Token.Null] = ParseKeywordLiteral;
            prefix_functions[Token.String] = ParseStringLiteral;
            prefix_functions[Token.Integer] = ParseIntegerLiteral;
            prefix_functions[Token.Float] = ParseFloatLiteral;
            prefix_functions[Token.True] = ParseBooleanLiteral;
            prefix_functions[Token.False] = ParseBooleanLiteral;
            prefix_functions[Token.Minus] = ParsePrefixExpression;
            prefix_functions[Token.Not] = ParsePrefixExpression;
            prefix_functions[Token.LeftParentheses] = ParseGroupExpression;
            prefix_functions[Token.If] = ParseIfExpression;
            prefix_functions[Token.Local] = ParseLocalExpression;
            prefix_functions[Token.Function] = ParseFunctionExpression;
            prefix_functions[Token.Return] = ParseReturnExpression;
            prefix_functions[Token.LeftBracket] = ParseArrayExpression;

            infix_functions[Token.Assign] = ParseInfixExpression;
            infix_functions[Token.PlusAssign] = ParseInfixExpression;
            infix_functions[Token.MinusAssign] = ParseInfixExpression;
            infix_functions[Token.StarAssign] = ParseInfixExpression;
            infix_functions[Token.SlashAssign] = ParseInfixExpression;
            infix_functions[Token.And] = ParseInfixExpression;
            infix_functions[Token.Or] = ParseInfixExpression;
            infix_functions[Token.Equal] = ParseInfixExpression;
            infix_functions[Token.NotEqual] = ParseInfixExpression;
            infix_functions[Token.Less] = ParseInfixExpression;
            infix_functions[Token.Greater] = ParseInfixExpression;
            infix_functions[Token.LessEqual] = ParseInfixExpression;
            infix_functions[Token.GreaterEqual] = ParseInfixExpression;
            infix_functions[Token.Plus] = ParseInfixExpression;
            infix_functions[Token.Minus] = ParseInfixExpression;
            infix_functions[Token.Star] = ParseInfixExpression;
            infix_functions[Token.Slash] = ParseInfixExpression;
            infix_functions[Token.BitAnd] = ParseInfixExpression;
            infix_functions[Token.BitOr] = ParseInfixExpression;
            infix_functions[Token.Dot] = ParseInstanceGetExpression;
            infix_functions[Token.LeftBracket] = ParseInstanceGetExpression;
            infix_functions[Token.LeftParentheses] = ParseCallExpression;
        }

        private Expression ParseArrayExpression()
        {
            ArrayExpression array_expression = new ArrayExpression();
            
            NextToken();

            array_expression.Values = ParseCommaExpressions(Token.RightBracket);
            
            return array_expression;
        }

        private List<Expression> ParseCommaExpressions(Token end_token)
        {
            List<Expression> expressions = new List<Expression>();
            
            bool last_is_comma = false;
            
            while (!(CurrentTokenIs(end_token) || CurrentTokenIs(Token.Eof)))
            {
                expressions.Add(ParseExpression());
                
                NextToken();
                
                if (CurrentTokenIs(Token.Comma))
                {
                    last_is_comma = true;
                    NextToken();
                }
                else
                {
                    last_is_comma = false;
                }
            }

            if (last_is_comma)
            {
                // TODO : raise error
            }

            if (!CurrentTokenIs(end_token))
            {
                // TODO : raise error
            }
            
            return expressions;
        }

        private Expression ParseInstanceGetExpression(Expression instnace)
        {
            InstanceGetExpression instance_get_expression = new InstanceGetExpression { Instance = instnace };

            if (CurrentTokenIs(Token.Dot))
            {
                if (!ExpectPeek(Token.Identifier))
                {
                    // TODO : raise error
                    return null;
                }

                instance_get_expression.Index = new StringLiteral { Data = CurrentTokenData };
                
                return instance_get_expression;
            }

            if (CurrentTokenIs(Token.LeftBracket))
            {
                NextToken();
                instance_get_expression.Index = ParseExpression();
                
                if (!ExpectPeek(Token.RightBracket))
                {
                    // TODO : raise error
                    return null;
                }

                return instance_get_expression;
            }

            // TODO : raise error
            return null;
        }

        private Expression ParseReturnExpression()
        {
            ReturnExpression return_expression = new ReturnExpression { Data = CurrentTokenData };

            if (PeekTokenData.Line == CurrentTokenData.Line)
            {
                NextToken();
                return_expression.Value = ParseExpression();
            }

            return return_expression;
        }

        private Expression ParseCallExpression(Expression function)
        {
            CallExpression call_expression = new CallExpression { Function = function };

            NextToken();

            call_expression.Parameters = ParseCommaExpressions(Token.RightParentheses);
            
            return call_expression;
        }

        private Expression ParseLocalExpression()
        {
            if (!ExpectPeek(Token.Identifier))
            {
                // TODO : raise error
            }
            
            LocalExpression local_expression = new LocalExpression { Identifier = (Identifier)ParseIdentifier() };

            if (PeekTokenIs(Token.Assign))
            {
                NextToken();
                NextToken();
                local_expression.Value = ParseExpression();
            }

            return local_expression;
        }

        private bool ExpectToken(Token token_to_check, Token token_expected)
        {
            if (token_to_check == token_expected)
                return true;
            
            // TODO : raise error
            return false;
        }

        private bool ExpectPeek(Token token_expected)
        {
            if (ExpectToken(PeekTokenData.Token, token_expected))
            {
                NextToken();
                return true;
            }

            return false;
        }

        private void NextToken()
        {
            CurrentTokenData = PeekTokenData;

            PeekTokenData = lexer.Lex();
        }

        private bool CurrentTokenIs(Token token)
        {
            return CurrentTokenData.Token == token;
        }

        private bool PeekTokenIs(Token token)
        {
            return PeekTokenData.Token == token;
        }

        private SymbolPriority CurrentPrecedence
        {
            get
            {
                if (precedences.ContainsKey(CurrentTokenData.Token))
                    return precedences[CurrentTokenData.Token];
                return SymbolPriority.Lowest;
            }
        }
        
        private SymbolPriority PeekPrecedence
        {
            get
            {
                if (precedences.ContainsKey(PeekTokenData.Token))
                    return precedences[PeekTokenData.Token];
                return SymbolPriority.Lowest;
            }
        }
        
        public Node Parse(string filename, string source = "")
        {
            if (String.IsNullOrEmpty(source))
                source = file_loader.LoadFile("", filename);
            
            lexer = new Lexer(filename, source);
            
            NextToken();
            NextToken();
            
            return ParseProgram(new[]{ Token.Eof });
        }

        private Program ParseProgram(IEnumerable<Token> terminators)
        {
            Program program = new Program();

            while (true)
            {
                if (terminators.Any(CurrentTokenIs))
                {
                    return program;
                }

                Expression expression = ParseExpression();
                
                if (expression != null)
                    program.Expressions.Add(expression);
                
                NextToken();
            }
        }

        private Expression ParseExpression(SymbolPriority precedence = SymbolPriority.Lowest)
        {
            if (!prefix_functions.ContainsKey(CurrentTokenData.Token))
            {
                // TODO : raise error, prefix function not exists
                throw new Exception("prefix function not exists");
            }
            
            PrefixFunction prefix_function = prefix_functions[CurrentTokenData.Token];

            Expression left_expression = prefix_function();

            while (!PeekTokenIs(Token.Eof) && precedence < PeekPrecedence)
            {
                if (!infix_functions.ContainsKey(PeekTokenData.Token))
                    return left_expression;
                
                NextToken();

                left_expression = infix_functions[CurrentTokenData.Token](left_expression);
            }

            return left_expression;
        }

        private Expression ParseIdentifier()
        {
            return new Identifier { Data = CurrentTokenData };
        }

        private Literal ParseKeywordLiteral()
        {
            if (CurrentTokenIs(Token.Base))
                return new BaseLiteral { Data = CurrentTokenData };

            if (CurrentTokenIs(Token.This))
                return new ThisLiteral { Data = CurrentTokenData };
            
            return new NullLiteral { Data = CurrentTokenData };
        }

        private Literal ParseStringLiteral()
        {
            return new StringLiteral { Data = CurrentTokenData };
        }

        
        private IntegerLiteral ParseIntegerLiteral()
        {
            IntegerLiteral integer_literal = new IntegerLiteral { Data = CurrentTokenData };
            
            if (!Int64.TryParse(integer_literal.Data.Value, out integer_literal.Value))
            {
                // TODO : raise parse error
            }

            return integer_literal;
        }

        private FloatLiteral ParseFloatLiteral()
        {
            FloatLiteral float_literal = new FloatLiteral { Data = CurrentTokenData };

            if (!double.TryParse(float_literal.Data.Value, out float_literal.Value))
            {
                // TODO : raise parse error
            }

            return float_literal;
        }

        private BooleanLiteral ParseBooleanLiteral()
        {
            BooleanLiteral boolean_literal = new BooleanLiteral { Data = CurrentTokenData };

            boolean_literal.Value = CurrentTokenIs(Token.True);

            return boolean_literal;
        }

        private PrefixExpression ParsePrefixExpression()
        {
            PrefixExpression prefix_expression = new PrefixExpression { Data = CurrentTokenData };
            NextToken();
            prefix_expression.Right = ParseExpression(SymbolPriority.Prefix);
            return prefix_expression;
        }

        private Expression ParseGroupExpression()
        {
            NextToken();

            Expression expression = ParseExpression();

            ExpectPeek(Token.RightParentheses);

            return expression;
        }

        private Expression ParseInfixExpression(Expression left)
        {
            InfixExpression infix_expression = new InfixExpression { Left = left, Data = CurrentTokenData };
            SymbolPriority current_precedence = CurrentPrecedence;
            NextToken();
            infix_expression.Right = ParseExpression(current_precedence);
            return infix_expression;
        }

        private Expression ParseIfExpression()
        {
            if (!ExpectPeek(Token.LeftParentheses))
            {
                // TODO : raise error
            }
            
            NextToken();
            
            IfExpression if_expression = new IfExpression { Condition = ParseExpression()};
            
            if (!ExpectPeek(Token.RightParentheses))
            {
                // TODO : raise error
            }
            
            NextToken();

            if_expression.TruePart = ParseProgram(new[]{ Token.Else, Token.End });

            if (CurrentTokenIs(Token.Else))
            {
                NextToken();
                if_expression.FalsePart = ParseProgram(new[]{ Token.End });
            }

            if (!CurrentTokenIs(Token.End))
            {
                // TODO : raise error
            }

            return if_expression;
        }

        private List<LocalExpression> ParseParameters(Token terminator)
        {
            List<LocalExpression> parameters = new List<LocalExpression>();

            bool last_is_comma = false;
            
            while (!(CurrentTokenIs(terminator) || CurrentTokenIs(Token.Eof)))
            {
                if (!CurrentTokenIs(Token.Identifier))
                {
                    // TODO : raise error
                }

                LocalExpression local_expression = new LocalExpression { Identifier = (Identifier)ParseIdentifier() };

                NextToken();

                if (CurrentTokenIs(Token.Assign))
                {
                    NextToken();
                    local_expression.Value = ParseExpression();
                    NextToken();
                }

                parameters.Add(local_expression);
                
                if (CurrentTokenIs(Token.Comma))
                {
                    NextToken();
                    last_is_comma = true;
                }
                else
                {
                    last_is_comma = false;
                }
            }

            if (last_is_comma)
            {
                // TODO : raise error
            }

            return parameters;
        }

        private Expression ParseFunctionExpression()
        {
            if (!ExpectPeek(Token.LeftParentheses))
            {
                // TODO : raise error
            }
            
            NextToken();
            
            FunctionExpression function_expression = new FunctionExpression();

            function_expression.Parameters = ParseParameters(Token.RightParentheses);
            
            if (!CurrentTokenIs(Token.RightParentheses))
            {
                // TODO : raise error
            }
            
            NextToken();

            function_expression.Body = ParseProgram(new[] { Token.End });

            if (!CurrentTokenIs(Token.End))
            {
                // TODO : raise error
            }

            return function_expression;
        }

    }
}