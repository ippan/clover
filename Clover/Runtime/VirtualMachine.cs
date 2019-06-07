using System;
using System.Collections.Generic;

namespace Clover.Runtime
{
    public class VirtualMachine
    {
        private Stack<Object> stack = new Stack<Object>();

        private List<Object> constants;

        private Stack<Frame> frames = new Stack<Frame>();
        
        public VirtualMachine(Context context)
        {
            ScriptFunction function = new ScriptFunction { Bytecode = context.Bytecode };
            
            frames.Push(new Frame(function, 0, context.Bytecode.LocalVariableCount));
            
            constants = context.Constants;
        }

        public Object PopStack()
        {
            return stack.Pop();
        }

        public int PushStack(Object target)
        {
            stack.Push(target);
            return stack.Count;
        }


        public Object Run()
        {
            while (!frames.Peek().Finished)
                Step();
            
            return frames.Peek().LastPop;
        }

        public void Step()
        {
            Frame frame = frames.Peek();

            int operation_code = frame.CurrentInstruction;
            
            frame.MoveInstructionPointer(1);

            try
            {
                switch (operation_code)
                {
                    case OpCode.Constant:
                        PushStack(constants[frame.CurrentInstruction]);
                        frame.MoveInstructionPointer(1);
                        break;
                    case OpCode.Pop:
                        frame.LastPop = PopStack();
                        break;
                    case OpCode.Add: 
                    case OpCode.Sub:
                    case OpCode.Multiply:
                    case OpCode.Divide:
                    case OpCode.Equal:
                    case OpCode.NotEqual:
                        Object right = PopStack();
                        Object left = PopStack();
                        switch (operation_code)
                        {
                            case OpCode.Add: PushStack(left.Add(right)); break;
                            case OpCode.Sub: PushStack(left.Sub(right)); break;
                            case OpCode.Multiply: PushStack(left.Multiply(right)); break;
                            case OpCode.Divide: PushStack(left.Divide(right)); break;
                        
                            case OpCode.Equal: PushStack(left.Equal(right)); break;
                            case OpCode.NotEqual: PushStack(left.NotEqual(right)); break;
                        }

                        break;
                    case OpCode.True:
                        PushStack(Boolean.True);
                        break;
                    case OpCode.False:
                        PushStack(Boolean.False);
                        break;
                    case OpCode.Null:
                        PushStack(Null.Instance);
                        break;

                    case OpCode.Jump:
                    {
                        Int32 target = frame.CurrentInstruction;
                        frame.SetInstructionPointer(target);
                        break;
                    }

                    case OpCode.JumpIf:
                    {
                        Object condition = PopStack();

                        if (condition.AsBoolean().Value)
                        {
                            Int32 target = frame.CurrentInstruction;
                            frame.SetInstructionPointer(target);
                        }
                        else
                        {
                            frame.MoveInstructionPointer(1);
                        }

                        break;
                    }
                    case OpCode.SetLocal:
                        frame.Set(frame.CurrentInstruction, stack.Peek());
                        frame.MoveInstructionPointer(1);
                        break;
                    case OpCode.GetLocal:
                        PushStack(frame.Get(frame.CurrentInstruction));
                        frame.MoveInstructionPointer(1);
                        break;

                    case OpCode.Closure:
                    {
                        Closure closure = new Closure((ScriptFunction)constants[frame.CurrentInstruction]);
                        frame.MoveInstructionPointer(1);

                        for (int i = closure.Source.ParameterCount - 1; i >= 0; i -= 1)
                        {
                            closure.DefaultValues[i] = PopStack();
                        }

                        PushStack(closure);
                        break;
                    }

                    case OpCode.Call:
                    {
                        Int32 parameter_count = frame.CurrentInstruction;
                        frame.MoveInstructionPointer(1);

                        Closure closure = (Closure)PopStack();

                        if (closure.DefaultValues.Length < parameter_count)
                        {
                            // TODO : raise error
                        }

                        Frame call_frame = new Frame(closure.Source, 0, closure.Source.Bytecode.LocalVariableCount);
                        frames.Push(call_frame);
                        for (int i = 0; i < closure.DefaultValues.Length; i += 1)
                        {
                            if (i < parameter_count)
                            {
                                call_frame.Set(parameter_count - i - 1, PopStack());
                            }
                            else
                            {
                                call_frame.Set(i, closure.DefaultValues[i]);
                            }
                        }

                        break;
                    }
                }

            }
            catch (RuntimeError runtime_error)
            {
                TokenData token_data = frame.LastToken;
                Console.WriteLine($"{runtime_error.Message} At: {token_data.Filename} - Line: {token_data.Line}");
            }
            
        }

    }
}