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

        public virtual Symbol DefineFree(string name, Symbol free_symbol)
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

        public virtual bool IsTopLevel => false;
    }

    public class Symbol
    {
        public int Index;
        public FrameScope Scope;
    }

    public class FreeSymbol
    {
        public int ParentIndex;
        public int Index;
    }

    public class BlockScope : Scope
    {
        private Dictionary<String, int> local_variable_indices = new Dictionary<string, int>();

        public FrameScope GetFrameScope()
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

        public override Symbol DefineFree(string name, Symbol free_symbol)
        {
            if (local_variable_indices.ContainsKey(name))
            {
                // TODO : raise duplicate define error
                return null;
            }

            Symbol symbol = GetFrameScope().DefineFree(name, free_symbol);
            local_variable_indices[name] = symbol.Index;

            return symbol;
        }

        public override Scope Pop()
        {
            if (Parent is FrameScope)
                return Parent.Parent;
            return Parent;
        }

        public override Symbol FindOuter(string name)
        {
            return Parent.FindOuter(name);
        }

        public override bool IsTopLevel => Parent is FrameScope && Parent.IsTopLevel;
    }

    public class FrameScope : Scope
    {
        private List<Symbol> local_variables = new List<Symbol>();

        private List<FreeSymbol> free_variables = new List<FreeSymbol>();
        
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

        public override Symbol DefineFree(string name, Symbol free_symbol)
        {
            Symbol symbol = new Symbol { Index = local_variables.Count, Scope = this };
            local_variables.Add(symbol);
            
            free_variables.Add(new FreeSymbol { ParentIndex = free_symbol.Index, Index = symbol.Index });

            return symbol;
        }
        
        public override Symbol FindOuter(string name)
        {
            Symbol symbol = Parent?.FindLocal(name);

            if (symbol == null)
                return null;

            FrameScope parent_frame_scope = ((BlockScope)Parent).GetFrameScope();

            // find 1 layer only, so check is it parent scope
            return symbol.Scope != parent_frame_scope ? null : symbol;
        }
        
        public int LocalVariableCount => local_variables.Count;

        public override bool IsTopLevel => Parent == null;
        
        public List<FreeSymbol> FreeVariables => free_variables;
        
    }

    public class ClassScope : Scope
    {
        public override bool IsTopLevel => false;
    }

}