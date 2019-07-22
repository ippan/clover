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
                CloverStdlib.Stdlib.Apply(virtual_machine);
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
                
                Compiler compiler = new Compiler();
                Context context = compiler.Compile(node);
                
                VirtualMachine virtual_machine = new VirtualMachine(context);
                CloverStdlib.Stdlib.Apply(virtual_machine);
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