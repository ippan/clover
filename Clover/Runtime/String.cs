namespace Clover.Runtime
{
    public class String : Object
    {
        public static readonly String Empty = new String { Value = string.Empty };

        public string Value; 
        
        public override string Inspect()
        {
            return $"\"{Value}\"";
        }
        
        public override Object Add(Object right)
        {
            return new String { Value = Value + right.AsString().Value };
        }

        public override String AsString()
        {
            return this;
        }
        
        public override Boolean AsBoolean()
        {
            return Value == string.Empty ? Boolean.False : Boolean.True;
        }

        public override Boolean Equal(Object right)
        {
            if (right is String string_value)
                return Value == string_value.Value ? Boolean.True : Boolean.False;
            return Boolean.False;
        }
        
        public override string GetClassName()
        {
            return "String";
        }
    }
}