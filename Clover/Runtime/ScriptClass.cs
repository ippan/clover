using System.Collections.Generic;
using System.Text;

namespace Clover.Runtime
{
    public class ScriptClass : Object
    {
        private Object parent = Null.Instance;
        private Dictionary<string, Object> members = new Dictionary<string, Object>();
        
        public override string Inspect()
        {
            StringBuilder builder = new StringBuilder();

            builder.AppendLine($"class extend {parent.Inspect()}");

            foreach (KeyValuePair<string, Object> member in members)
            {
                builder.AppendLine($"{member.Key}= {member.Value.Inspect()}");
            }

            builder.AppendLine("end");

            return builder.ToString();
        }

        public void AddMember(string key, Object value)
        {
            members.Add(key, value);
        }

        public void SetParent(Object value)
        {
            parent = value;
        }
    }
    
    public class ScriptClassInstance : Object
    {
        private ScriptClass script_class;
    }
}