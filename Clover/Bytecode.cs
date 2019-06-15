using System;
using System.Collections.Generic;
using System.Text;

namespace Clover
{
    public class OpCode
    {
        public const Int32 Constant = 0x000002;
        
        public const Int32 Pop = 0x000101;

        public const Int32 Closure = 0x000202;
        public const Int32 Call = 0x000302;
        public const Int32 Return = 0x000401;
        
        public const Int32 SetLocal = 0x000402;
        public const Int32 GetLocal = 0x000502;
        
        public const Int32 SetGlobal = 0x000601;
        public const Int32 GetGlobal = 0x000701;

        public const Int32 InstanceGet = 0x000801;
        public const Int32 InstanceSet = 0x000901;

        public const Int32 InstanceGlobalGet = 0x000A01;
        public const Int32 InstanceGlobalSet = 0x000B01;

        public const Int32 NewArray = 0x001002;
        
        public const Int32 Add = 0x010201;
        public const Int32 Sub = 0x010301;
        public const Int32 Multiply = 0x010401;
        public const Int32 Divide = 0x010501;
        

        public const Int32 Equal = 0x011001;
        public const Int32 NotEqual = 0x011101;
        
        public const Int32 True = 0x020001;
        public const Int32 False = 0x020101;
        public const Int32 Null = 0x021001;

        public const Int32 Jump = 0x100002;
        public const Int32 JumpIf = 0x100102;
    }

    public class Bytecode
    {
        public List<Int32> Instructions = new List<Int32>();
        public List<TokenData> TokenDatas = new List<TokenData>();
        public int LocalVariableCount;
        
        private TokenData last_token_data;

        public void Reposition(Int32 start, Int32 offset)
        {
            Int32 position = start;

            while (position < Instructions.Count)
            {
                switch (Instructions[position])
                {
                    case OpCode.Jump:
                    case OpCode.JumpIf:
                        Instructions[position + 1] += offset;
                        break;
                }

                position += Instructions[position] & 0xFF;
            }

        }

        public Int32 Add(Int32 instruction, TokenData? token_data = null)
        {
            Instructions.Add(instruction);
            if (token_data.HasValue)
                last_token_data = token_data.Value;
            
            TokenDatas.Add(last_token_data);

            return Instructions.Count - 1;
        }
        
        public Int32 Insert(Int32 index, Int32 instruction, TokenData? token_data = null)
        {
            Instructions.Insert(index, instruction);
            if (token_data.HasValue)
                last_token_data = token_data.Value;
            
            TokenDatas.Insert(index, last_token_data);

            return Instructions.Count - 1;
        }

        public Int32 LastInstruction => Instructions[Instructions.Count - 1];
        
        public Int32 PopLast()
        {
            Instructions.RemoveAt(Instructions.Count - 1);
            TokenDatas.RemoveAt(TokenDatas.Count - 1);
            
            return Instructions.Count - 1;
        }

        public string Dump()
        {
            StringBuilder builder = new StringBuilder();

            int position = 0;

            while (position < Instructions.Count)
            {
                builder.Append(Instructions[position].ToString("x8"));

                int instruction_length = Instructions[position] & 0xFF;
                
                for (int i = 0; i < instruction_length - 1; i += 1)
                {
                    builder.Append($" {Instructions[position + i + 1]}");
                }

                builder.AppendLine();

                position += instruction_length;
            }
            
            return builder.ToString();
        }
    }
}