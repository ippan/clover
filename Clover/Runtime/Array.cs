using System.Collections.Generic;
using System.Text;

namespace Clover.Runtime
{
    public class Array : Object
    {
        private List<Object> data;

        public Array(List<Object> new_data)
        {
            data = new_data;
        }

        public override Object InstanceGet(Object key)
        {
            if (key is Integer index)
            {
                if (index.Value >= data.Count || index.Value < 0)
                    return Null.Instance;

                return data[(int)index.Value];
            }

            return base.InstanceGet(key);
        }

        public Object Append(Object[] parameters)
        {
            Object value = parameters[0];
            data.Add(value);
            return value;
        }

        public Object Remove(Object[] parameters)
        {
            Integer index = (Integer)parameters[0];
            
            if (index.Value >= data.Count || index.Value < 0)
                return Null.Instance;

            Object value = data[(int)index.Value];
            data.RemoveAt((int)index.Value);
            return value;
        }

        protected override Object InternalInstanceGet(string key)
        {
            switch (key)
            {
                case "size":
                    return new Integer { Value = data.Count };
                case "empty":
                    return new Boolean { Value = data.Count == 0 };
                case "append":
                    return new NativeFunction(Append, 1);
                case "remove":
                    return new NativeFunction(Remove, 1);
            }
            
            return base.InternalInstanceGet(key);
        }

        public override Object InstanceSet(Object key, Object value)
        {
            if (key is Integer index)
            {
                if (index.Value >= data.Count || index.Value < 0)
                {
                    // TODO : raise error
                }

                data[(int)index.Value] = value;
                return value;
            }

            return base.InstanceSet(key, value);
        }
        
        public override string Inspect()
        {
            StringBuilder builder = new StringBuilder();

            builder.Append("[");

            bool first = true;

            foreach (Object value in data)
            {
                if (!first)
                    builder.Append(", ");
                first = false;

                builder.Append(value.Inspect());
            }

            builder.AppendLine("]");

            return builder.ToString();
        }
    }
}