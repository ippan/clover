using System.Collections.Generic;
using Clover.Runtime;

namespace Clover
{
    public class Context
    {
        public List<Object> Constants = new List<Object>();
        public Bytecode Bytecode;

        public int AddConstant(Object value)
        {
            Constants.Add(value);
            return Constants.Count - 1;
        }
    }
}