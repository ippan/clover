using System;

namespace Clover
{
    public enum Token
    {
        Invalid,
        Eof,
        None,
        
        Identifier,
        String,
        Integer,
        Float,
        
        True,
        False,
        Null,
        
        Assign,
        Plus,
        Minus,
        Star,
        Slash,
        PlusAssign,
        MinusAssign,
        StarAssign,
        SlashAssign,
        
        BitAnd,
        BitOr,
        
        Not,
        And,
        Or,
        
        Equal,
        NotEqual,
        Less,
        Greater,
        LessEqual,
        GreaterEqual,
        
        LeftParentheses,
        RightParentheses,
        LeftBracket,
        RightBracket,
        LeftBrace,
        RightBrace,
        
        Comma,
        Colon,
        Dot,
        
        End,
        Local,
        Function,
        Return,
        Class,
        Extends,
        New,
        Base,
        This,
        Constructor,
        At,
        
        If,
        Else,
        While,
        
        Load
    }

    public struct TokenData
    {
        public Token Token;
        public String Value;
        public String Filename;
        public int Line;
    }

}