using System;
using System.Collections.Generic;
using System.IO;
using System.Net.Mime;

namespace Clover
{
    public class Lexer
    {
        private String filename;
        private String source;
        private int line;
        private int current_position = 0;
        private int token_line = 0;
        
        private static readonly Dictionary<string, Token> Keywords = new Dictionary<string, Token>()
        {
            { "function", Token.Function },
            { "local", Token.Local },
            { "New", Token.New },
            { "end", Token.End },
            { "if", Token.If },
            { "else", Token.Else },
            { "and", Token.And },
            { "or", Token.Or },
            { "not", Token.Not },
            { "true", Token.True },
            { "false", Token.False },
            { "null", Token.Null },
            { "class", Token.Class },
            { "extends", Token.Extends },
            { "return", Token.Return },
            { "while", Token.While },
            { "base", Token.Base },
            { "this", Token.This },
            { "constructor", Token.Constructor },
            { "load", Token.Load }
        };

        private static readonly Dictionary<string, Token> Symbols = new Dictionary<string, Token>()
        {
            { "=", Token.Assign },
            { "+", Token.Plus },
            { "-", Token.Minus },
            { "*", Token.Star },
            { "/", Token.Slash },
            { "!", Token.Not },
            { "(", Token.LeftParentheses },
            { ")", Token.RightParentheses },
            { "[", Token.LeftBracket },
            { "]", Token.RightBracket },
            { "{", Token.LeftBrace },
            { "}", Token.RightBrace },
            { ",", Token.Comma },
            { ":", Token.Colon },
            { "&", Token.BitAnd },
            { "|", Token.BitOr },
            { ".", Token.Dot },
            { "@", Token.At },
            { "==", Token.Equal },
            { "!=", Token.NotEqual },
            { "&&", Token.And },
            { "||", Token.Or },
            { ">",  Token.Greater },
            { "<",  Token.Less },
            { ">=", Token.GreaterEqual },
            { "<=", Token.LessEqual },
            { "+=", Token.PlusAssign },
            { "-=", Token.MinusAssign },
            { "*=", Token.StarAssign },
            { "/=", Token.SlashAssign },
        };
        
        public Lexer(String filename, String source)
        {
            this.filename = filename;
            this.source = source;
            line = 0;
            current_position = 0;
        }

        public TokenData Lex()
        {
            while (IsSpace(CurrentCharacter))
                NextCharacter();

            token_line = line;
            
            if (Eof)
                return MakeTokenData(Token.Eof, "end of file");

            if (CurrentCharacter == '#')
            {
                SkipComment();
                return Lex();
            }

            if (IsString(CurrentCharacter))
                return LexString();

            if (IsIdentifier(CurrentCharacter))
                return LexIdentifier();
            
            if (IsNumber(CurrentCharacter))
                return LexNumber();

            if (IsSymbol(CurrentCharacter))
                return LexSymbol();
            
            return MakeTokenData(Token.Invalid, $"Unkown character '{CurrentCharacter}'");
        }

        private TokenData MakeTokenData(Token token, String value)
        {
            return new TokenData()
            {
                Token = token,
                Value = value,
                Filename = Filename,
                Line = TokenLine
            };
        }

        private TokenData LexIdentifier()
        {
            int position = current_position;

            do
            {
                NextCharacter();
            } while (IsIdentifier(CurrentCharacter) || IsNumber(CurrentCharacter));

            string identifier = source.Substring(position, current_position - position);

            if (Keywords.ContainsKey(identifier))
                return MakeTokenData(Keywords[identifier], identifier);

            return MakeTokenData(Token.Identifier, identifier);
        }

        private TokenData LexString()
        {
            // TODO : parse string with special character like \n if it is a "" string"

            Char boundary = CurrentCharacter;
            int position = current_position + 1;

            do
            {
                NextCharacter();
            } while (CurrentCharacter != '\0' && CurrentCharacter != boundary);

            int end_position = current_position;

            NextCharacter();

            return MakeTokenData(Token.String, source.Substring(position, end_position - position));
        }

        private TokenData LexNumber()
        {
            Token token = Token.Integer;
            int position = current_position;

            while (IsNumber(CurrentCharacter) || CurrentCharacter == '.')
            {
                if (CurrentCharacter == '.')
                {
                    if (token == Token.Float)
                        break;

                    if (!IsNumber(PeekCharacter))
                        break;
                    
                    token = Token.Float;
                }
                
                NextCharacter();
            }

            return MakeTokenData(token, source.Substring(position, current_position - position));
        }

        private TokenData LexSymbol()
        {
            string symbol = CurrentCharacter.ToString();

            NextCharacter();

            if (IsSymbol(CurrentCharacter))
            {
                string multi_characters_symbol = symbol + CurrentCharacter;

                if (Symbols.ContainsKey(multi_characters_symbol))
                {
                    NextCharacter();
                    return MakeTokenData(Symbols[multi_characters_symbol], multi_characters_symbol);
                }
            }

            if (Symbols.ContainsKey(symbol))
                return MakeTokenData(Symbols[symbol], symbol);


            return MakeTokenData(Token.Invalid, $"Unknown symbol '{symbol}'");
        }

        private void SkipComment()
        {
            while (CurrentCharacter != '\0' && CurrentCharacter != '\n')
            {
                NextCharacter();
            }

            NextCharacter();
        }

        public int TokenLine
        {
            get { return token_line + 1; }
        }
        public String Filename
        {
            get { return filename; }
        }

        private void NextCharacter()
        {
            if (CurrentCharacter == '\n')
            {
                line += 1;
            }

            current_position += 1;
        }

        private bool IsSpace(Char character)
        {
            return character == ' ' || character == '\t' || character == '\r' || character == '\v' || character == '\f' || character == '\n';
        }

        private bool IsSymbol(Char character)
        {
            return Symbols.ContainsKey(character.ToString());
        }

        private bool IsString(Char character)
        {
            return character == '\'' || character == '"';
        }

        private bool IsAlpha(Char character)
        {
            return (character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z');
        }

        private bool IsIdentifier(Char character)
        {
            return character == '_' || IsAlpha(character);
        }

        private bool IsNumber(Char character)
        {
            return character >= '0' && character <= '9';
        }

        private Char CurrentCharacter
        {
            get { return Eof ? '\0' : source[current_position]; }
        }

        private Char PeekCharacter
        {
            get { return current_position + 1 >= source.Length ? '\0' : source[current_position + 1]; }
        }

        private bool Eof
        {
            get { return current_position >= source.Length; }
        }
    }
}