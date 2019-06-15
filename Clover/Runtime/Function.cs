using System;
using System.Collections.Generic;
using System.Reflection;
using System.Text;

namespace Clover.Runtime
{
    public class Function : Object
    {
        
    }

    public class ScriptFunction : Function
    {
        public Bytecode Bytecode;
        public int ParameterCount;
    }

    
    
    public class NativeFunction : Function
    {
        public delegate Object NativeFunctionDefine(Object[] parameters);

        public NativeFunctionDefine Function;
        public int ParameterCount;

        public NativeFunction(NativeFunctionDefine function, int parameter_count = 0)
        {
            Function = function;
            ParameterCount = parameter_count;
        }
    }

    public class Closure : Function
    {
        public Closure(ScriptFunction source)
        {
            Source = source;
            DefaultValues = new Object[source.ParameterCount];
        }

        public ScriptFunction Source;

        public Object[] DefaultValues;

        public override string Inspect()
        {
            StringBuilder builder = new StringBuilder();

            builder.Append("closure(");

            bool first = true;

            foreach (Object default_value in DefaultValues)
            {
                if (!first)
                    builder.Append(", ");
                first = false;

                builder.Append(default_value.Inspect());
            }

            builder.AppendLine(")");

            builder.Append(Source.Bytecode.Dump());
            
            builder.Append("end");
            
            return builder.ToString();
        }
    }
}