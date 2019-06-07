using System;
using System.Collections.Generic;

namespace Clover
{
    public class Scope
    {
        public Scope Parent;

        public virtual Symbol DefineLocal(string name)
        {
            return null;
        }

        public virtual Symbol FindLocal(string name)
        {
            return null;
        }

        public virtual Symbol FindOuter(string name)
        {
            return null;
        }

        public virtual Scope Pop()
        {
            return Parent;
        }

        public virtual bool IsGlobal => false;
    }

    public class Symbol
    {
        public int Index;
        public FrameScope Scope;
    }

    public class BlockScope : Scope
    {
        private Dictionary<String, int> local_variable_indices = new Dictionary<string, int>();

        private FrameScope GetFrameScope()
        {
            Scope scope = Parent;

            while (scope != null)
            {
                if (scope is FrameScope frame_scope)
                    return frame_scope;

                scope = scope.Parent;
            }

            return null;
        }

        public override Symbol FindLocal(string name)
        {
            if (local_variable_indices.ContainsKey(name))
                return GetFrameScope().GetSymbol(local_variable_indices[name]);
            return Parent.FindLocal(name);
        }

        public override Symbol DefineLocal(string name)
        {
            if (local_variable_indices.ContainsKey(name))
            {
                // TODO : raise duplicate define error
                return null;
            }

            Symbol symbol = GetFrameScope().DefineLocal(name);
            local_variable_indices[name] = symbol.Index;

            return symbol;
        }

        public override Scope Pop()
        {
            if (Parent is FrameScope)
                return Parent.Parent;
            return Parent;
        }

        public override bool IsGlobal => Parent is FrameScope && Parent.IsGlobal;
    }

    public class FrameScope : Scope
    {
        private List<Symbol> local_variables = new List<Symbol>();

        public Symbol GetSymbol(int index)
        {
            return local_variables[index];
        }

        public override Symbol DefineLocal(string name)
        {
            Symbol symbol = new Symbol { Index = local_variables.Count, Scope = this };
            local_variables.Add(symbol);
            return symbol;
        }

        public int LocalVariableCount => local_variables.Count;

        public override bool IsGlobal => Parent == null;
    }
}