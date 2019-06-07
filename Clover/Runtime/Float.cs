using System;

namespace Clover.Runtime
{
    public class Float : Object
    {
        public double Value;

        public override string Inspect()
        {
            return Value.ToString();
        }

        public override Object Add(Object right)
        {
            if (right is Float)
            {
                return new Float { Value = Value + ((Float)right).Value };
            }
            
            if (right is Integer)
            {
                return new Float { Value = Value + ((Integer)right).Value };
            }

            if (right is String)
            {
                return AsString().Add(right);
            }
            
            return base.Add(right);
        }
        
        public override Object Sub(Object right)
        {
            if (right is Float)
            {
                return new Float { Value = Value - ((Float)right).Value };
            }
            
            if (right is Integer)
            {
                return new Float { Value = Value - ((Integer)right).Value };
            }
            
            return base.Sub(right);
        }

        public override Object Multiply(Object right)
        {
            if (right is Float)
            {
                return new Float { Value = Value * ((Float)right).Value };
            }
            
            if (right is Integer)
            {
                return new Float { Value = Value * ((Integer)right).Value };
            }
            
            return base.Multiply(right);
        }

        public override Object Divide(Object right)
        {
            if (right is Float)
            {
                return new Float { Value = Value / ((Float)right).Value };
            }
            
            if (right is Integer)
            {
                return new Float { Value = Value / ((Integer)right).Value };
            }
            
            return base.Divide(right);
        }

        public override String AsString()
        {
            return new String { Value = Value.ToString() };
        }
        
        public override Boolean AsBoolean()
        {
            return Value == 0.0 ? Boolean.False : Boolean.True;
        }

        public override Boolean Equal(Object right)
        {
            if (right is Float float_value)
                return Value == float_value.Value ? Boolean.True : Boolean.False;
            
            return Boolean.False;
        }
        
        public override Boolean Greater(Object right)
        {
            if (right is Integer)
            {
                return new Boolean { Value = Value > ((Integer)right).Value };
            }
            
            if (right is Float)
            {
                return new Boolean { Value = Value > ((Float)right).Value };
            }

            return base.Greater(right);
        }

        public override Boolean Smaller(Object right)
        {
            if (right is Integer)
            {
                return new Boolean { Value = Value < ((Integer)right).Value };
            }
            
            if (right is Float)
            {
                return new Boolean { Value = Value < ((Float)right).Value };
            }

            return base.Smaller(right);
        }
        
        public override string GetClassName()
        {
            return "Float";
        }
    }
}