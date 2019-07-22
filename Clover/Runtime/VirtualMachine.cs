using System;
using System.Collections.Generic;

namespace Clover.Runtime
{
    public class VirtualMachine
    {
        private Stack<Object> stack = new Stack<Object>();

        private List<Object> constants;

        private List<Object> variables = new List<Object>();
        private List<int> variable_reference = new List<int>();
        private Stack<int> free_variable_indices = new Stack<int>();
        
        private Dictionary<string, Object> globals = new Dictionary<string, Object>();
        
        
        private Stack<Frame> frames = new Stack<Frame>();
        
        public VirtualMachine(Context context)
        {
            ScriptFunction function = new ScriptFunction { Bytecode = context.Bytecode };

            Frame frame = new Frame(function, context.Bytecode.LocalVariableCount);
            
            for (int i = 0; i < context.Bytecode.LocalVariableCount; i += 1)
                frame.SetVariableIndex(i, Allocate());
            
            frames.Push(frame);
            
            
            constants = context.Constants;
        }

        public Int32 Allocate()
        {
            if (free_variable_indices.Count > 0)
            {
                Int32 index = free_variable_indices.Pop();
                variable_reference[index] = 1;
                return index;
            }

            variables.Add(null);
            variable_reference.Add(1);

            return variables.Count - 1;
        }

        public void AddReference(Int32 index)
        {
            variable_reference[index] += 1;
        }

        public void Release(Int32 index)
        {
            variable_reference[index] -= 1;

            if (variable_reference[index] > 0)
                return;

            Object variable = variables[index];
            
            variables[index] = null;
            free_variable_indices.Push(index);

            // TODO : release closure's free variable, otherwise will have a memory leak
            /*
            if (variable is Closure closure)
            {
                foreach (int variable_index in closure.FreeVariableIndices.Values)
                    Release(variable_index);
            }
            */
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

        public void CallClosure(Closure closure, Object Self, int parameter_count)
        {
            if (closure.DefaultValues.Length < parameter_count)
            {
                // TODO : raise error
            }

            Frame call_frame = new Frame(closure.Source, closure.Source.Bytecode.LocalVariableCount);
            call_frame.Self = Self;
            frames.Push(call_frame);
            

            for (int i = 0; i < closure.Source.Bytecode.LocalVariableCount; i += 1)
            {
                if (i < parameter_count)
                {
                    call_frame.SetVariableIndex(parameter_count - i - 1, Allocate());
                    Int32 index = call_frame.GetVariableIndex(parameter_count - i - 1);
                    variables[index] = PopStack();
                }
                else
                {
                    if (closure.FreeVariableIndices.ContainsKey(i))
                    {
                        int variable_index = closure.FreeVariableIndices[i];
                        call_frame.SetVariableIndex(i, variable_index);
                        AddReference(variable_index);
                        continue;
                    }
                    
                    call_frame.SetVariableIndex(i, Allocate());
                    Int32 index = call_frame.GetVariableIndex(i);
                    variables[index] = i < closure.DefaultValues.Length ? closure.DefaultValues[i] : Null.Instance;
                }
                
            }
        }

        private void CallNative(NativeFunction native, int parameter_count)
        {
            if (parameter_count > native.ParameterCount && native.ParameterCount != -1)
            {
                // TODO : raise error
            }

            Object[] parameters = new Object[parameter_count];

            for (int i = parameter_count - 1; i >= 0; i -= 1)
            {
                parameters[i] = PopStack();
            }

            PushStack(native.Function(parameters, this));
        }

        public void RunOneFrame()
        {
            int frame_count = frames.Count;

            if (frame_count < 2)
            {
                // TODO : raise error
            }

            while (frame_count <= frames.Count)
                Step();
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

                    case OpCode.LocalSet:
                    {
                        
                        Int32 index = frame.GetVariableIndex(frame.CurrentInstruction);
                        frame.MoveInstructionPointer(1);
                        variables[index] = stack.Peek();
                        break;
                    }

                    case OpCode.LocalGet:
                    {
                        Int32 index = frame.GetVariableIndex(frame.CurrentInstruction);
                        PushStack(variables[index]);
                        frame.MoveInstructionPointer(1);
                        break;
                    }

                    case OpCode.GlobalSet:
                    {
                        String global_name = (String)PopStack();
                        globals[global_name.Value] = stack.Peek();
                        break;
                    }

                    case OpCode.GlobalGet:
                    {
                        String global_name = (String)PopStack();
                        PushStack(globals[global_name.Value]);

                        break;
                    }

                    case OpCode.Closure:
                    {
                        Closure closure = new Closure((ScriptFunction)constants[frame.CurrentInstruction]);
                        frame.MoveInstructionPointer(1);

                        Int32 free_variable_count = frame.CurrentInstruction;
                        frame.MoveInstructionPointer(1);

                        for (int i = 0; i < free_variable_count; i += 1)
                        {
                            Object index = PopStack();
                            Object variable_index = PopStack();
                            closure.FreeVariableIndices.Add((Int32)((Integer)index).Value, (Int32)((Integer)variable_index).Value);
                            AddReference((Int32)((Integer)variable_index).Value);
                        }

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

                        Object function = PopStack();

                        if (function is Closure closure)
                        {
                            CallClosure(closure, Null.Instance, parameter_count);
                        }
                        else if (function is NativeFunction native)
                        {
                            CallNative(native, parameter_count);
                        }
                        else if (function is MemberFunction member_function)
                        {
                            CallClosure(member_function.Source, member_function.Self, parameter_count);
                        }
                        else
                        {
                            // TODO : raise error
                        }

                        break;
                    }

                    case OpCode.FreeVariable:
                    {
                        int parent_index = frame.CurrentInstruction;
                        int index = frame.NextInstruction;
                        frame.MoveInstructionPointer(2);

                        PushStack(new Integer { Value = frame.GetVariableIndex(parent_index) });
                        PushStack(new Integer { Value = index });
                            
                        break;
                    }
                    
                    case OpCode.Return:
                    {
                        Frame function_frame = frames.Pop();
                        
                        for (int i = 0; i < function_frame.LocalVariableCount; i += 1)
                            Release(function_frame.GetVariableIndex(i));
                        
                        break;
                    }

                    case OpCode.NewArray:
                    {
                        int value_count = frame.CurrentInstruction;
                        frame.MoveInstructionPointer(1);

                        List<Object> values = new List<Object>();
                        for (int i = 0; i < value_count; i += 1)
                            values.Add(PopStack());
                        
                        values.Reverse();
                        
                        PushStack(new Array(values));
                        break;
                    }

                    case OpCode.NewMap:
                    {
                        int key_value_count = frame.CurrentInstruction;
                        frame.MoveInstructionPointer(1);
                        
                        Dictionary<string, Object> key_values = new Dictionary<string, Object>();

                        for (int i = 0; i < key_value_count; i += 1)
                        {
                            Object value = PopStack();
                            String key = (String)PopStack();
                            key_values[key.Value] = value;
                        }

                        PushStack(new Map(key_values));
                        
                        break;
                    }

                    case OpCode.NewClass:
                    {
                        int member_count = frame.CurrentInstruction;
                        frame.MoveInstructionPointer(1);

                        ScriptClass script_class = new ScriptClass();

                        Object parent = PopStack();
                        
                        if (parent != Null.Instance)
                            script_class.Parent = (ScriptClass)parent;

                        for (int i = 0; i < member_count; i += 1)
                        {
                            Object value = PopStack();
                            String key = (String)PopStack();
                            script_class.AddMember(key.Value, value);
                        }

                        PushStack(script_class);
                        
                        break;
                    }

                    case OpCode.EnvironmentGet:
                    {
                        Object index = PopStack();
                        Object environment_value = null;

                        if (frame.Self != null)
                            environment_value = frame.Self.InstanceGet(index);

                        if (environment_value == null && index is String global_key)
                        {
                            if (globals.ContainsKey(global_key.Value))
                                environment_value = globals[global_key.Value];
                        }

                        if (environment_value == null)
                        {
                            // TODO : raise error
                        }

                        PushStack(environment_value);
                        
                        break;
                    }

                    case OpCode.EnvironmentSet:
                    {
                        Object index = PopStack();
                        Object value = PopStack();
                        Object environment_value = null;

                        if (frame.Self != null)
                            environment_value = frame.Self.InstanceSet(index, value);

                        if (environment_value == null && index is String global_key)
                        {
                            if (globals.ContainsKey(global_key.Value))
                            {
                                globals[global_key.Value] = value;
                                environment_value = value;
                            }
                        }

                        if (environment_value == null)
                        {
                            // TODO : raise error
                        }

                        PushStack(environment_value);
                        
                        break;
                    }

                    case OpCode.BaseGet:
                    {
                        Object index = PopStack();

                        if (!(frame.Self is ScriptClassInstance script_class_instance))
                        {
                            // TODO : raise error
                            break;
                        }

                        Object value = script_class_instance.BaseGet(index);

                        if (value == null)
                        {
                            // TODO : raise error
                        }

                        PushStack(value);
                        
                        break;
                    }

                    case OpCode.InstanceGet:
                    case OpCode.InstanceSet:
                    {
                        Object index = PopStack();
                        Object instance = PopStack();

                        switch (operation_code)
                        {
                            case OpCode.InstanceGet:
                            {
                                Object instance_value = instance.InstanceGet(index);

                                if (instance_value == null)
                                {
                                    // TODO : raise error
                                }

                                PushStack(instance_value);
                                break;
                            }

                            case OpCode.InstanceSet:
                            {
                                Object value = PopStack();

                                Object return_value = instance.InstanceSet(index, value);

                                if (return_value == null)
                                {
                                    // TODO : raise error
                                }

                                PushStack(return_value);
                                
                                break;
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

        public void AddGlobal(string key, Object value)
        {
            globals.Add(key, value);
        }
    }
}