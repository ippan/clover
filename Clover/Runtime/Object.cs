using System.Collections.Generic;
using System.Reflection;

namespace Clover.Runtime
{
    public class Object
    {
        public virtual string Inspect()
        {
            return string.Empty;
        }

        public virtual Object Add(Object right)
        {
            throw new RuntimeError($"can not add {GetClassName()} and {right.GetClassName()}");
        }
        
        public virtual Object Sub(Object right)
        {
            throw new RuntimeError($"can not sub {GetClassName()} and {right.GetClassName()}");
        }

        public virtual Object Multiply(Object right)
        {
            throw new RuntimeError($"can not Multiply {GetClassName()} and {right.GetClassName()}");
        }
        
        public virtual Object Divide(Object right)
        {
            throw new RuntimeError($"can not divide {GetClassName()} and {right.GetClassName()}");
        }
        
        public virtual String AsString()
        {
            throw new RuntimeError($"can not convert {GetClassName()} to String");
        }

        public virtual Boolean AsBoolean()
        {
            throw new RuntimeError($"can not convert {GetClassName()} to Boolean");
        }

        public virtual Boolean Equal(Object right)
        {
            if (this == right)
                return Boolean.True;
            
            return Boolean.False;
        }

        public virtual Boolean NotEqual(Object right)
        {
            return Equal(right).Not();
        }

        public virtual Boolean Not()
        {
            return AsBoolean().Not();
        }

        public virtual Boolean Greater(Object right)
        {
            throw new RuntimeError($"can not compare {GetClassName()} and {right.GetClassName()}");
        }

        public virtual Boolean Smaller(Object right)
        {
            throw new RuntimeError($"can not compare {GetClassName()} and {right.GetClassName()}");
        }
        
        public virtual Boolean GreaterEqual(Object right)
        {
            return Smaller(right).Not();
        }
        
        public virtual Boolean SmallEqual(Object right)
        {
            return Greater(right).Not();
        }

        public virtual Object InstanceGet(Object key)
        {
            if (!(key is String index))
                return Null.Instance;

            return InternalInstanceGet(index.Value);
        }

        protected virtual Object InternalInstanceGet(string key)
        {
            switch (key)
            {
                case "to_string":
                    return new NativeFunction(ScriptAsString);
            }

            return Null.Instance;
        }

        public virtual Object InstanceSet(Object key)
        {
            return null;
        }

        public virtual Object ScriptAsString(Object[] parameters)
        {
            return AsString();
        }

        public virtual string GetClassName()
        {
            return "Object";
        }
    }
}