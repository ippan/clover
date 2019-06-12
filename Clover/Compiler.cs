using System;
using System.Collections.Generic;
using Clover.Ast;
using Clover.Runtime;
using Boolean = System.Boolean;
using Object = Clover.Runtime.Object;

namespace Clover
{
    public class Compiler
    {
        private delegate bool CompilerFunction(Node node, Context context);

        private readonly Dictionary<Type, CompilerFunction> compiler_functions = new Dictionary<Type, CompilerFunction>();
        
        private Scope scope = null;
        
        private TokenData last_token_data;

        public Compiler()
        {
            compiler_functions[typeof(Program)] = CompileProgram;
            compiler_functions[typeof(IntegerLiteral)] = CompileIntegerLiteral;
            compiler_functions[typeof(FloatLiteral)] = CompileFloatLiteral;
            compiler_functions[typeof(StringLiteral)] = CompileStringLiteral;
            compiler_functions[typeof(BooleanLiteral)] = CompileBooleanLiteral;
            compiler_functions[typeof(NullLiteral)] = CompileNullLiteral;
            compiler_functions[typeof(InfixExpression)] = CompileInfixExpression;
            compiler_functions[typeof(IfExpression)] = CompileIfExpression;
            compiler_functions[typeof(LocalExpression)] = CompileLocalExpression;
            compiler_functions[typeof(Identifier)] = CompileIdentifier;
            compiler_functions[typeof(FunctionExpression)] = CompileFunctionExpression;
            compiler_functions[typeof(CallExpression)] = CompileCallExpression;
            compiler_functions[typeof(ReturnExpression)] = CompileReturnExpression;
            compiler_functions[typeof(InstanceGetExpression)] = CompileInstanceGetExpression;

        }

        private bool CompileInstanceGetExpression(Node node, Context context)
        {
            InstanceGetExpression instance_get_expression = (InstanceGetExpression)node;
            Compile(instance_get_expression.Instance, context);
            Compile(instance_get_expression.Index, context);
            context.Bytecode.Add(OpCode.InstanceGet);

            return true;
        }

        private bool CompileReturnExpression(Node node, Context context)
        {
            ReturnExpression return_expression = (ReturnExpression)node;

            if (return_expression.Value != null)
            {
                Compile(return_expression.Value, context);
            }
            else
            {
                context.Bytecode.Add(OpCode.Null, return_expression.Data);
            }

            context.Bytecode.Add(OpCode.Return, return_expression.Data);

            return true;
        }

        private bool CompileCallExpression(Node node, Context context)
        {
            CallExpression call_expression = (CallExpression)node;

            foreach (Expression expression in call_expression.Parameters)
            {
                Compile(expression, context);
            }

            Compile(call_expression.Function, context);

            context.Bytecode.Add(OpCode.Call);
            context.Bytecode.Add(call_expression.Parameters.Count);
            
            return true;
        }

        private bool CompileFunctionExpression(Node node, Context context)
        {
            EnterFrame();

            FunctionExpression function_expression = (FunctionExpression)node;

            foreach (LocalExpression local_expression in function_expression.Parameters)
            {
                scope.DefineLocal(local_expression.Identifier.Data.Value);
            }

            Bytecode bytecode = context.Bytecode;
            context.Bytecode = new Bytecode();
            
            Compile(function_expression.Body, context);

            context.Bytecode.PopLast();

            if (context.Bytecode.LastInstruction != OpCode.Return)
            {
                context.Bytecode.Add(OpCode.Return);
            }

            FrameScope frame_scope = ExitFrame();

            ScriptFunction script_function = new ScriptFunction { Bytecode = context.Bytecode, ParameterCount = function_expression.Parameters.Count };

            int index = context.AddConstant(script_function);

            context.Bytecode.LocalVariableCount = frame_scope.LocalVariableCount;

            context.Bytecode = bytecode;

            foreach (LocalExpression local_expression in function_expression.Parameters)
            {
                if (local_expression.Value != null)
                {
                    Compile(local_expression.Value, context);
                }
                else
                {
                    context.Bytecode.Add(OpCode.Null);
                }
            }
            
            bytecode.Add(OpCode.Closure);
            bytecode.Add(index);
            
            return true;
        }

        private bool CompileIdentifier(Node node, Context context)
        {
            Identifier identifier = (Identifier)node;
            Symbol symbol = scope.FindLocal(identifier.Data.Value);

            if (symbol != null)
            {
                context.Bytecode.Add(OpCode.GetLocal, identifier.Data);
                context.Bytecode.Add(symbol.Index);
                return true;
            }

            Int32 index = AddConstant(new Runtime.String { Value = identifier.Data.Value }, context);
            context.Bytecode.Add(OpCode.Constant, identifier.Data);
            context.Bytecode.Add(index);

            context.Bytecode.Add(OpCode.GetGlobal);
            
            return true;
        }

        public Context Compile(Node node)
        {
            Context context = new Context { Bytecode = new Bytecode() };
            
            EnterFrame();
            
            if (!Compile(node, context))
                return null;

            FrameScope frame_scope = ExitFrame();

            context.Bytecode.LocalVariableCount = frame_scope.LocalVariableCount;
            
            return context;
        }

        private bool Compile(Node node, Context context)
        {
            Type node_type = node.GetType();

            if (!compiler_functions.ContainsKey(node_type))
            {
                Console.WriteLine("error");
                // TODO : raise error
                return false;
            }
            
            return compiler_functions[node_type](node, context);
        }

        private bool CompileProgram(Node node, Context context)
        {
            Program program = node as Program;
            
            foreach (Expression expression in program.Expressions)
            {
                if (!Compile(expression, context))
                    return false;

                context.Bytecode.Add(OpCode.Pop);
            }

            if (program.Expressions.Count == 0)
            {
                context.Bytecode.Instructions.Add(OpCode.Null);
                context.Bytecode.TokenDatas.Add(last_token_data);
                
                context.Bytecode.Instructions.Add(OpCode.Pop);
                context.Bytecode.TokenDatas.Add(last_token_data);
            }
            
            return true;
        }

        private Int32 AddConstant(Object value, Context context)
        {
            // TODO : use a search table
            for (int i = 0; i < context.Constants.Count; i += 1)
                if (value.Equal(context.Constants[i]).Value)
                    return i;
            
            context.Constants.Add(value);
            return context.Constants.Count - 1;
        }

        private bool CompileIntegerLiteral(Node node, Context context)
        {
            context.Bytecode.Instructions.Add(OpCode.Constant);
            context.Bytecode.Instructions.Add(AddConstant(new Integer { Value = ((IntegerLiteral)node).Value }, context));
            
            PushTokenData(((IntegerLiteral)node).Data, context.Bytecode);
            PushTokenData(((IntegerLiteral)node).Data, context.Bytecode);
            
            return true;
        }

        private bool CompileFloatLiteral(Node node, Context context)
        {
            Int32 index = context.Constants.Count;
            
            context.Constants.Add(new Float { Value = ((FloatLiteral)node).Value });
            
            context.Bytecode.Instructions.Add(OpCode.Constant);
            context.Bytecode.Instructions.Add(index);
            
            PushTokenData(((FloatLiteral)node).Data, context.Bytecode);
            PushTokenData(((FloatLiteral)node).Data, context.Bytecode);

            return true;
        }

        private bool CompileBooleanLiteral(Node node, Context context)
        {
            BooleanLiteral boolean_literal = node as BooleanLiteral;

            context.Bytecode.Instructions.Add(boolean_literal.Value ? OpCode.True : OpCode.False);


            PushTokenData(((BooleanLiteral)node).Data, context.Bytecode);

            return true;
        }

        private void PushTokenData(TokenData token_data, Bytecode bytecode)
        {
            last_token_data = token_data;
            
            bytecode.TokenDatas.Add(last_token_data);
        }

        private bool CompileNullLiteral(Node node, Context context)
        {
            context.Bytecode.Instructions.Add(OpCode.Null);
            PushTokenData(((NullLiteral)node).Data, context.Bytecode);
            return true;
        }

        private bool CompileStringLiteral(Node node, Context context)
        {
            Int32 index = context.Constants.Count;
            
            context.Constants.Add(new Runtime.String { Value = ((StringLiteral)node).Data.Value });
            
            context.Bytecode.Instructions.Add(OpCode.Constant);
            context.Bytecode.Instructions.Add(index);
            
            PushTokenData(((StringLiteral)node).Data, context.Bytecode);
            PushTokenData(((StringLiteral)node).Data, context.Bytecode);
            return true;
        }

        private bool CompileInfixExpression(Node node, Context context)
        {
            InfixExpression infix_expression = node as InfixExpression;

            if (infix_expression.Data.Token != Token.Assign)
            {
                Compile(infix_expression.Left, context);
                Compile(infix_expression.Right, context);

                PushTokenData(infix_expression.Data, context.Bytecode);
            }

            switch (infix_expression.Data.Token)
            {
                case Token.Plus:
                    context.Bytecode.Instructions.Add(OpCode.Add);
                    break;
                case Token.Minus:
                    context.Bytecode.Instructions.Add(OpCode.Sub);
                    break;
                case Token.Star:
                    context.Bytecode.Instructions.Add(OpCode.Multiply);
                    break;
                case Token.Slash:
                    context.Bytecode.Instructions.Add(OpCode.Divide);
                    break;
                
                case Token.Equal:
                    context.Bytecode.Instructions.Add(OpCode.Equal);
                    break;
                case Token.NotEqual:
                    context.Bytecode.Instructions.Add(OpCode.NotEqual);
                    break;
                case Token.Assign:
                    CompileAssignExpression(infix_expression, context);
                    break;
            }
            
            return true;
        }

        private bool CompileAssignExpression(InfixExpression node, Context context)
        {
            Compile(node.Right, context);

            if (!(node.Left is Identifier))
            {
                // TODO : expression
                return false;
            }

            Identifier identifier = (Identifier)node.Left;

            Symbol symbol = scope.FindLocal(identifier.Data.Value);

            if (symbol != null)
            {
                context.Bytecode.Add(OpCode.SetLocal, identifier.Data);
                context.Bytecode.Add(symbol.Index);
                return true;
            }
            
            if (scope.IsTopLevel)
            {
                Int32 index = AddConstant(new Runtime.String { Value = identifier.Data.Value }, context);
                context.Bytecode.Add(OpCode.Constant, identifier.Data);
                context.Bytecode.Add(index);
                context.Bytecode.Add(OpCode.SetGlobal);
            }

            return false;
        }

        private bool CompileIfExpression(Node node, Context context)
        {
            IfExpression if_expression = (IfExpression)node;

            PushScope();
            Compile(if_expression.Condition, context);
            
            context.Bytecode.Instructions.Add(OpCode.JumpIf);
            context.Bytecode.TokenDatas.Add(last_token_data);

            Int32 jump_index = context.Bytecode.Instructions.Count;
            context.Bytecode.Instructions.Add(0);
            context.Bytecode.TokenDatas.Add(last_token_data);

            if (if_expression.FalsePart != null)
            {
                PushScope();
                Compile(if_expression.FalsePart, context);
                context.Bytecode.PopLast();
                PopScope();
            }

            context.Bytecode.Instructions.Add(OpCode.Jump);
            context.Bytecode.TokenDatas.Add(last_token_data);
            Int32 end_index = context.Bytecode.Instructions.Count;
            context.Bytecode.Instructions.Add(0);
            context.Bytecode.TokenDatas.Add(last_token_data);
            
            context.Bytecode.Instructions[jump_index] = context.Bytecode.Instructions.Count;
            
            PushScope();
            Compile(if_expression.TruePart, context);
            context.Bytecode.PopLast();
            PopScope();

            context.Bytecode.Instructions[end_index] = context.Bytecode.Instructions.Count;
            
            PopScope();
            
            return true;
        }

        private bool CompileLocalExpression(Node node, Context context)
        {
            LocalExpression local_expression = (LocalExpression)node;

            Symbol symbol = scope.DefineLocal(local_expression.Identifier.Data.Value);

            if (local_expression.Value != null)
            {
                Compile(local_expression.Value, context);
            }
            else
            {
                context.Bytecode.Instructions.Add(OpCode.Null);
                context.Bytecode.TokenDatas.Add(local_expression.Identifier.Data);
            }

            context.Bytecode.Add(OpCode.SetLocal, local_expression.Identifier.Data);
            context.Bytecode.Add(symbol.Index);
            
            return true;
        }

        private void EnterFrame()
        {
            FrameScope frame_scope = new FrameScope { Parent = scope };
            scope = new BlockScope { Parent = frame_scope };
        }

        private FrameScope ExitFrame()
        {
            FrameScope frame_scope = scope.Parent as FrameScope;

            scope = scope.Pop();
            
            return frame_scope;
        }

        private void PushScope()
        {
            if (scope != null)
            {
                BlockScope block_scope = new BlockScope { Parent = scope };
                scope = block_scope;
            }
            else
            {
                scope = new FrameScope();
                PushScope();
            }
        }

        private void PopScope()
        {
            scope = scope.Pop();
        }

    }
}