using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Boolean = Clover.Runtime.Boolean;

namespace Clover.Ast
{
    public class Node
    {
        public override string ToString()
        {
            return string.Empty;
        }

        public virtual string Dump()
        {
            return $"{GetType().Name}[{ToString()}]";
        }
    }

    public class Expression : Node
    {
    }

    public class Identifier : Expression
    {
        public TokenData Data;

        public override string ToString()
        {
            return Data.Value;
        }
    }

    public class Literal : Expression
    {
        public TokenData Data;

        public override string ToString()
        {
            return Data.Value;
        }
    }

    public class BaseLiteral : Literal
    {
    }

    public class ThisLiteral : Literal
    {
    }

    public class NullLiteral : Literal
    {
    }

    public class IntegerLiteral : Literal
    {
        public Int64 Value;
    }

    public class FloatLiteral : Literal
    {
        public Double Value;
    }

    public class StringLiteral : Literal
    {
        public override string ToString()
        {
            return $"\"{Data.Value}\"";
        }
    }

    public class BooleanLiteral : Literal
    {
        public bool Value;
    }

    public class PrefixExpression : Expression
    {
        public TokenData Data;
        public Expression Right;

        public override string ToString()
        {
            return $"{Data.Value} {Right}";
        }

        public override string Dump()
        {
            return $"{GetType().Name}[{Data.Value} {Right.Dump()}]";
        }
    }

    public class InfixExpression : Expression
    {
        public Expression Left;
        public TokenData Data;
        public Expression Right;

        public override string ToString()
        {
            return $"{Left} {Data.Value} {Right}";
        }

        public override string Dump()
        {
            return $"{GetType().Name}[{Left.Dump()} {Data.Value} {Right.Dump()}]";
        }
    }

    public class LocalExpression : Expression
    {
        public Identifier Identifier;
        public Expression Value;

        public override string ToString()
        {
            return $"local {Identifier.Data.Value} = {Value}";
        }

        public override string Dump()
        {
            return $"{GetType().Name}[local {Identifier.Dump()} = {Value.Dump()}]";
        }
    }

    public class FunctionExpression : Expression
    {
        public List<LocalExpression> Parameters;
        public Program Body;

        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            builder.Append("function(");

            bool first = true;

            foreach (LocalExpression parameter in Parameters)
            {
                if (!first)
                    builder.Append(", ");

                first = false;

                builder.Append(parameter.Identifier);

                if (parameter.Value != null)
                {
                    builder.Append(" = ");
                    builder.Append(parameter.Value);
                }
            }

            builder.AppendLine(")");

            builder.Append(Body);
            builder.AppendLine("end");

            return builder.ToString();
        }

        public override string Dump()
        {
            // TODO : implement dump
            return "";
        }
    }

    public class Program : Node
    {
        public List<Expression> Expressions = new List<Expression>();

        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            foreach (Expression expression in Expressions)
                builder.AppendLine(expression.ToString());

            return builder.ToString();
        }

        public override string Dump()
        {
            StringBuilder builder = new StringBuilder();

            builder.AppendLine($"{GetType().Name}[");

            foreach (Expression expression in Expressions)
                builder.AppendLine(expression.Dump());

            builder.AppendLine("]");

            return builder.ToString();
        }
    }

    public class IfExpression : Expression
    {
        public Expression Condition;
        public Program TruePart;
        public Program FalsePart;

        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            builder.Append("if (");
            builder.Append(Condition);
            builder.AppendLine(")");
            builder.Append(TruePart);
            if (FalsePart != null)
            {
                builder.AppendLine("else");
                builder.Append(FalsePart);
            }

            builder.AppendLine("end");

            return builder.ToString();
        }

        public override string Dump()
        {
            // TODO : implement dump
            return "";
        }
    }

    public class CallExpression : Expression
    {
        public Expression Function;
        public List<Expression> Parameters;

        public override string ToString()
        {
            return $"{Function}({String.Join(", ", Parameters)})";
        }

        public override string Dump()
        {
            StringBuilder builder = new StringBuilder();

            bool first = true;

            foreach (Expression parameter in Parameters)
            {
                if (!first)
                    builder.Append(", ");
                first = false;

                builder.Append(parameter.Dump());
            }

            return $"{GetType().Name}[{Function.Dump()}({builder})]";
        }
    }

    public class ReturnExpression : Expression
    {
        public TokenData Data;
        public Expression Value;

        public override string ToString()
        {
            return $"return {Value}";
        }

        public override string Dump()
        {
            string value_dump = string.Empty;

            if (Value != null)
                value_dump = Value.Dump();

            return $"{GetType().Name}[return {value_dump}]";
        }
    }

    public class InstanceGetExpression : Expression
    {
        public Expression Instance;
        public Expression Index;

        public override string ToString()
        {
            return $"{Instance}[{Index}]";
        }

        public override string Dump()
        {
            return $"{GetType().Name}[{Instance.Dump()}[{Index.Dump()}]]";
        }
    }

    public class ArrayExpression : Expression
    {
        public List<Expression> Values;

        public override string ToString()
        {
            return $"[{String.Join(",", Values)}]";
        }

        public override string Dump()
        {
            StringBuilder builder = new StringBuilder();

            bool first = true;

            foreach (Expression parameter in Values)
            {
                if (!first)
                    builder.Append(", ");
                first = false;

                builder.Append(parameter.Dump());
            }

            return $"[{GetType().Name}[{builder}]]";
        }
    }

    public class MapExpression : Expression
    {
        public List<LocalExpression> KeyValues;

        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            builder.Append("{\n");

            foreach (LocalExpression parameter in KeyValues)
            {
                builder.AppendLine($"{parameter.Identifier}: {parameter.Value}");
            }

            builder.AppendLine("}");

            return builder.ToString();
        }

        public override string Dump()
        {
            // TODO : implement dump
            return "";
        }
    }

    public class ClassExpression : Expression
    {
        public Expression SuperClass;
        public List<LocalExpression> Members;
        
        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            builder.Append("class");

            if (SuperClass != null)
            {
                builder.Append($" extends {SuperClass}");
            }

            builder.Append("\n");

            foreach (LocalExpression parameter in Members)
            {
                builder.AppendLine($"{parameter.Identifier} = {parameter.Value}");
            }

            builder.AppendLine("end");

            return builder.ToString();
        }
    }
}