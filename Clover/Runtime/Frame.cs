using System;
using System.Collections.Generic;

namespace Clover.Runtime
{
    public class Frame
    {
        public ScriptFunction Function;
        public Int32 InstructionPointer = 0;
        public Int32 BasePointer;

        public Object[] local_variables;
        
        public Object LastPop;
        
        public Frame(ScriptFunction function, Int32 base_pointer, Int32 local_variable_count)
        {
            Function = function;
            BasePointer = base_pointer;
            local_variables = new Object[local_variable_count];
        }

        public Int32 CurrentInstruction => Function.Bytecode.Instructions[InstructionPointer];

        public Int32 NextInstruction => Function.Bytecode.Instructions[InstructionPointer + 1];
        
        public void MoveInstructionPointer(int value)
        {
            InstructionPointer += value;
        }

        public void SetInstructionPointer(Int32 value)
        {
            InstructionPointer = value;
        }

        public Object Set(int index, Object value)
        {
            local_variables[index] = value;
            return value;
        }

        public Object Get(int index)
        {
            return local_variables[index];
        }

        public TokenData LastToken => Function.Bytecode.TokenDatas[InstructionPointer - 1];

        public bool Finished => InstructionPointer >= Function.Bytecode.Instructions.Count;
    }
}