using System;

namespace Evaluation
{
    internal class Program
    {
        public const char FolderSeparator = '\\';
        public const string OutputSeparator = "\t";

        private static void Main()
        {
            ContentDeleter.DeletePartialLogs();
            var tableBuilder = new TableBuilder(Parser.ParseResults());
            tableBuilder.BuildDeterministicTable();
            tableBuilder.ToFile();
            //tableBuilder.PrintTables();


            //ContentDeleter.DeletePartialLogs();

            Console.WriteLine("Done.");
            //Console.ReadLine();
        }
    }
}

