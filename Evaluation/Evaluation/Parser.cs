using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using Evaluation.Solver;

namespace Evaluation
{
    public static class Parser
    {
        private const string ParsePath = @"C:\Users\mwagner\Desktop\TSP Results";

        public static List<ISolver> ParseResults()
        {
            Console.WriteLine("Parsing results...");
            var results = new List<ISolver>();
            foreach (var directory in Directory.GetDirectories(ParsePath))
            {
                ISolver solver;
                var parameters = directory.Split(Program.FolderSeparator).Last().Split('.');
                switch (parameters[0])
                {
                    case "greedy":
                        solver = new Greedy(parameters);
                        break;
                    case "ls":
                        solver = new LocalSearch(parameters);
                        break;
                    case "grasp":
                        solver = new Grasp(parameters);
                        break;
                    case "pilot":
                        solver = new Pilot(parameters);
                        break;
                    case "vnd":
                        solver = new VariableNeighborhood(parameters);
                        break;
                    case "sa":
                        solver = new SimulatedAnnealing(parameters);
                        break;
                    default:
                        Console.WriteLine("Can't process " + parameters[0]);
                        continue;
                }

                // Different algorithms
                foreach (var fileName in Directory.EnumerateFiles(directory, "*.txt"))
                {
                    var lines = new List<string>();
                    using (var fileStream = File.OpenRead(fileName))
                    {
                        using (var streamReader = new StreamReader(fileStream, Encoding.UTF8, true, 128))
                        {
                            string line;
                            while ((line = streamReader.ReadLine()) != null)
                            {
                                if (!string.IsNullOrWhiteSpace(line))
                                {
                                    lines.Add(line);
                                }
                            }
                        }
                    }

                    if (lines.Count < 6) continue;

                    var splitName = lines[0].Split('_');
                    var drivers = double.Parse(splitName[1].Trim('k'));
                    for (var i = 0; i < splitName.Length - 1; i++)
                    {
                        splitName[i] = splitName[i] + "\\";
                    }
                    lines[0] = string.Join('_', splitName);

                    var val = Math.Sqrt(double.Parse(lines[3].Split(':')[1].Trim()) / drivers);
                    var time = ulong.Parse(lines[4].Trim('s').Trim('m'));
                    var isFeasible = lines[5] == "FEASIBLE";
                    var result = new Result(val, time, isFeasible);
                    solver.AddResult(lines[0], result);
                }

                if (solver.Results.Count == 0) continue;

                results.Add(solver);
            }

            return results;
        }
    }
}
