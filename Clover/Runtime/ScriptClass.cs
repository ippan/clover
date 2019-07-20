using System.Collections.Generic;
using System.Text;

namespace Clover.Runtime
{
    public class ScriptClass : Object
    {
        private ScriptClass parent = null;
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
        
        public ScriptClass Parent
        {
            get { return parent; }
            set { parent = value; }
        }

        public Dictionary<string, Object> Members => members;
        
        public Object FindMember(string member_name)
        {
            if (members.ContainsKey(member_name))
                return members[member_name];

            return parent?.FindMember(member_name);
        }

        private Object CreateInstance(Object[] parameters, VirtualMachine virtual_machine)
        {
            ScriptClassInstance instance = new ScriptClassInstance(this);

            Object member = instance.InstanceGet(new String { Value = "constructor" });

            if (member is MemberFunction member_function)
            {
                foreach (Object parameter in parameters)
                    virtual_machine.PushStack(parameter);
                virtual_machine.CallClosure(member_function.Source, member_function.Self, parameters.Length);
                virtual_machine.RunOneFrame();
                virtual_machine.PopStack();
            }

            return instance;
        }

        protected override Object InternalInstanceGet(string key)
        {
            switch (key)
            {
                case "new":
                    return new NativeFunction(CreateInstance);
            }

            return base.InternalInstanceGet(key);
        }
    }
    
    public class ScriptClassInstance : Object
    {
        private ScriptClass script_class;

        private Dictionary<string, Object> members = new Dictionary<string, Object>();
        
        public ScriptClassInstance(ScriptClass source_script_class)
        {
            script_class = source_script_class;
        }

        protected Object TransformValue(string key, Object value)
        {
            if (value == null)
                return null;
            
            if (value is Closure closure)
                return new MemberFunction { Source = closure, Self = this };
            
            members[key] = value.Clone();
            
            return members[key];
        }

        protected override Object InternalInstanceGet(string key)
        {
            if (members.ContainsKey(key))
                return members[key];
            
            Object member = script_class.FindMember(key);
            
            if (member == null)
                return base.InternalInstanceGet(key);

            return TransformValue(key, member);
        }

        protected override Object InternalInstanceSet(string key, Object value)
        {
            Object member = InternalInstanceGet(key);

            if (member == null)
                return base.InternalInstanceSet(key, value);

            members[key] = value;

            return value;
        }

        public Object BaseGet(Object index)
        {
            if (script_class.Parent == null)
                return null;
            
            if (!(index is String key))
                return null;

            return TransformValue(key.Value, script_class.Parent.FindMember(key.Value));
        }

        public override string Inspect()
        {
            StringBuilder builder = new StringBuilder();

            builder.AppendLine("instance {");

            Dictionary<string, bool> members = new Dictionary<string, bool>();

            ScriptClass current_script_class = script_class;

            while (current_script_class != null)
            {
                foreach (KeyValuePair<string, Object> member in current_script_class.Members)
                {
                    if (members.ContainsKey(member.Key))
                        continue;
                    builder.AppendLine($"{member.Key}= {member.Value.Inspect()}");
                    members.Add(member.Key, true);
                }

                current_script_class = current_script_class.Parent;
            }

            builder.AppendLine("}");

            return builder.ToString();
        }
    }
}