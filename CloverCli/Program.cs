using System;
using Clover;
using Clover.Ast;
using Clover.Runtime;
using Object = System.Object;

namespace CloverCli
{
    class Program
    {
        const string Prompt = "clover>";

        static void Repl()
        {
            while (true)
            {
                Console.Write(Prompt);

                string line = Console.ReadLine();

                if (line == "exit")
                    break;

                Parser parser = new Parser();
                Node node = parser.Parse("main", line);
                
                Compiler compiler = new Compiler();
                Context context = compiler.Compile(node);

                VirtualMachine virtual_machine = new VirtualMachine(context); 
                Clover.Runtime.Object result = virtual_machine.Run();
                
                Console.WriteLine(result.Inspect());
            }
        }

        static void Main(string[] args)
        {
            if (args.Length > 0)
            {
                Parser parser = new Parser();
                Node node = parser.Parse(args[0]);
                
                Console.WriteLine(node.ToString());
                
                Compiler compiler = new Compiler();
                Context context = compiler.Compile(node);
                
                Console.WriteLine(context.Bytecode.Dump());
                
                VirtualMachine virtual_machine = new VirtualMachine(context); 
                Clover.Runtime.Object result = virtual_machine.Run();
                
                Console.WriteLine(result.Inspect());

            }
            else
            {
                Repl();
            }


        }
    }
}