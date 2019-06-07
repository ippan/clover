namespace Clover.Runtime
{
    public class Boolean : Object
    {
        public bool Value;

        public static readonly Boolean True = new Boolean { Value = true };
        public static readonly Boolean False = new Boolean { Value = false };
        
        public override string Inspect()
        {
            return AsString().Value;
        }
        
        public override String AsString()
        {
            return new String { Value = Value ? "true" : "false" };
        }
        
        public override Boolean AsBoolean()
        {
            return this;
        }

        public override Boolean Equal(Object right)
        {
            if (right is Boolean boolean_value)
                return Value == boolean_value.Value ? True : False;
            return False;
        }

        public override Boolean Not()
        {
            return Value ? False : True;
        }

        public override string GetClassName()
        {
            return "Boolean";
        }
    }
}