using System.Collections.Generic;
using System.Text;

namespace Clover.Runtime
{
    public class Map : Object
    {
        private Dictionary<string, Object> data;

        public Map(Dictionary<string, Object> new_data)
        {
            data = new_data;
        }

        public override Object Clone()
        {
            Dictionary<string, Object> clone_data = new Dictionary<string, Object>(data.Count);
            
            foreach (KeyValuePair<string, Object> value in data)
                clone_data.Add(value.Key, value.Value.Clone());
            
            return new Map(clone_data);
        }

        public override Object InstanceGet(Object key)
        {
            if (key is String index)
            {
                if (data.ContainsKey(index.Value))
                    return data[index.Value];
            }

            return base.InstanceGet(key);
        }

        public Object Add(Object[] parameters, VirtualMachine virtual_machine)
        {
            Object key = parameters[0];
            Object value = parameters[1];

            if (key is String index)
            {
                data[index.Value] = value;
                return value;
            }

            // TODO : raise error
            return null;
        }

        public Object Remove(Object[] parameters, VirtualMachine virtual_machine)
        {
            Object key = parameters[0];
            
            if (key is String index)
            {
                if (!data.ContainsKey(index.Value))
                    return Null.Instance;

                Object value = data[index.Value];
                data.Remove(index.Value);
                return value;
            }


            // TODO : raise error
            return null;
        }

        protected override Object InternalInstanceGet(string key)
        {
            switch (key)
            {
                case "size":
                    return new Integer { Value = data.Count };
                case "empty":
                    return new Boolean { Value = data.Count == 0 };
                case "add":
                    return new NativeFunction(Add, 2);
                case "remove":
                    return new NativeFunction(Remove, 1);
            }
            
            return base.InternalInstanceGet(key);
        }

        public override Object InstanceSet(Object key, Object value)
        {
            if (key is String index)
            {
                data[index.Value] = value;
                return value;
            }

            return base.InstanceSet(key, value);
        }
        
        public override string Inspect()
        {
            StringBuilder builder = new StringBuilder();

            builder.AppendLine("{");
            
            foreach (KeyValuePair<string, Object> key_value in data)
            {
                builder.AppendLine($"{key_value.Key}: {key_value.Value.Inspect()}");
            }

            builder.AppendLine("}");

            return builder.ToString();
        }
    }
}