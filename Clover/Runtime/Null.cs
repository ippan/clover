namespace Clover.Runtime
{
    public class Null : Object
    {
        public static readonly Null Instance = new Null();

        public override string Inspect()
        {
            return "null";
        }

        public override String AsString()
        {
            return String.Empty;
        }

        public override Boolean AsBoolean()
        {
            return Boolean.False;
        }

        public override Boolean Equal(Object right)
        {
            return right is Null ? Boolean.True : Boolean.False;
        }
        
        public override string GetClassName()
        {
            return "Null";
        }
    }
}