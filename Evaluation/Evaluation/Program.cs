using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using Evaluation.Solver;

namespace Evaluation
{
    internal class Program
    {
        private const string ParsePath = @"C:\Users\wage\Documents\Projects\Uni\ProgrammingAssignment_1\results";
        private const string DeletePath = @"C:\Users\wage\Documents\Projects\Uni\ProgrammingAssignment_1\results";
        private const char FolderSeparator = '\\';

        private static void Main(string[] args)
        {
            var results = ParseResults(ParsePath);
            //DeletePartialLogs(DeletePath);

            Console.WriteLine("Done.");
        }

        private static void DeletePartialLogs(string deletionDirectory)
        {
            foreach (var directory in Directory.GetDirectories(deletionDirectory))
            {
                Console.WriteLine("Deleting in " + directory);
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

                    File.WriteAllLines(fileName, lines.GetRange(0, 3));
                }
            }
        }

        private static List<ISolver> ParseResults(string parseDirectory)
        {
            var results = new List<ISolver>();
            foreach (var directory in Directory.GetDirectories(parseDirectory))
            {
                ISolver solver;
                var parameters = directory.Split(FolderSeparator).Last().Split('.');
                Console.WriteLine(string.Join(',', parameters));
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

                    var val = Math.Sqrt(double.Parse(lines[3].Split(':')[1].Trim()));
                    var time = ulong.Parse(lines[4].Trim('s').Trim('m'));
                    var isFeasible = lines[5] == "FEASIBLE";
                    var result = new Result(val, time, isFeasible);
                    solver.AddResult(lines[0], result);
                    Console.WriteLine("Done adding to " + fileName);
                }

                results.Add(solver);
            }

            return results;
        }
    }
}

