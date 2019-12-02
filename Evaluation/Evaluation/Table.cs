using Evaluation.Solver;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Evaluation
{
    public class Table
    {
        private const string LogPath = @"C:\Users\mwagner\Desktop\TSP Results\";
        public string Name { get; }
        private readonly List<string> _heading = new List<string>();
        private readonly Dictionary<string, List<string>> _data;
        private int _columnCounter;

        public Table(string name)
        {
            Name = name;
            _data = new Dictionary<string, List<string>>();
            _heading.Add("instance");
        }

        public void AppendColumn(ISolver solver, Func<string, string> action)
        {
            _heading.Add(solver.ToString());
            foreach (var instance in solver.Results.Keys)
            {
                if (!_data.ContainsKey(instance))
                {
                    _data[instance] = new List<string>(_columnCounter + 1);
                }
                for (var i = _data[instance].Count; i < _columnCounter; i++)
                {
                    _data[instance].Add("-");
                }

                _data[instance].Add(action(instance));
            }

            _columnCounter++;
        }

        public void Print()
        {
            Console.WriteLine(string.Join(Program.OutputSeparator, _heading));
            foreach (var (instance, resultList) in _data)
            {
                Console.WriteLine(instance + Program.OutputSeparator + string.Join(Program.OutputSeparator, resultList));
            }
        }

        public void ToFile()
        {
            var lines = new List<string>
            {
                string.Join(Program.OutputSeparator, _heading) + GetLineBreak()
            };
            foreach (var (instance, resultList) in _data)
            {
                lines.Add(instance + Program.OutputSeparator + string.Join(Program.OutputSeparator, resultList) + GetLineBreak());
            }


            File.WriteAllLines($"{LogPath}{Name}{GetFileEnd()}", lines);
        }

        private string GetLineBreak()
        {
            switch (Program.OutputSeparator)
            {
                case " & ":
                    return "\\\\";

                default:
                    return "";
            }
        }
        private string GetFileEnd()
        {
            switch (Program.OutputSeparator)
            {
                case "\t":
                    return ".tsv";
                case ",":
                    return ".csv";
                default:
                    return ".txt";
            }
        }
    }
}
