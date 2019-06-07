using System;

namespace Clover.Runtime
{
    public class RuntimeError : Exception
    {
        public RuntimeError(string message) : base(message)
        {
        }
    }
}