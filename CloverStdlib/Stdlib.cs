using System;
using System.Collections.Generic;
using System.Text;
using Clover.Runtime;
using Object = Clover.Runtime.Object;
using String = Clover.Runtime.String;

namespace CloverStdlib
{
    public class Stdlib : Object
    {
        private readonly Dictionary<string, NativeFunction> functions;
        public Stdlib()
        {
            functions = new Dictionary<string, NativeFunction>
            {
                { "print", new NativeFunction(Print, -1) }
            };
        }

        public Object Print(Object[] parameters, VirtualMachine virtual_machine)
        {
            StringBuilder builder = new StringBuilder();
            foreach (Object parameter in parameters)
                builder.Append(parameter.AsString().Value);

            string value = builder.ToString();
            
            Console.WriteLine(value);
            
            return new String { Value = value };
        }

        protected override Object InternalInstanceGet(string key)
        {
            if (functions.ContainsKey(key))
                return functions[key];
            
            return base.InternalInstanceGet(key);
        }

        public static void Apply(VirtualMachine virtual_machine)
        {
            virtual_machine.AddGlobal("std", new Stdlib());
        }
    }
}