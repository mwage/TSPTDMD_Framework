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
        private const string FolderSeperator = "\\";

        private static void Main(string[] args)
        {
            ParseResults(ParsePath);
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

        private static void ParseResults(string parseDirectory)
        {
            foreach (var directory in Directory.GetDirectories(parseDirectory))
            {
                var results = new List<ISolver>();
                var parameters = directory.Split('\\').Last().Split('.');
                Console.WriteLine(string.Join(',', parameters));
                switch (parameters[0])
                {
                    case "greedy":
                        results.Add(new Greedy(parameters));
                        break;
                    case "ls":
                        results.Add(new LocalSearch(parameters));
                        break;
                    case "grasp":
                        results.Add(new Grasp(parameters));
                        break;
                    case "pilot":
                        results.Add(new Pilot(parameters));
                        break;
                    case "vnd":
                        results.Add(new VariableNeighborhood(parameters));
                        break;
                    default:
                        Console.WriteLine("Can't process " + parameters[0]);
                        continue;
                }

                //// Different algorithms
                //foreach (var fileName in Directory.EnumerateFiles(directory, "*.txt"))
                //{


                //    var lines = new List<string[]>();
                //    using (var fileStream = File.OpenRead(fileName))
                //    {
                //        using (var streamReader = new StreamReader(fileStream, Encoding.UTF8, true, 128))
                //        {
                //            string line;
                //            while ((line = streamReader.ReadLine()) != null)
                //            {
                //                if (!string.IsNullOrWhiteSpace(line))
                //                {
                //                    lines.Add(line.Split('\t'));
                //                }
                //            }
                //        }
                //    }

                //    var result = new MCAResult(parameters);
                //    var finished = lines[lines.Count - 1][0].Contains("dont-cares");
                //    if (finished)
                //    {
                //        result.DontCare = int.Parse(lines[lines.Count - 1][0].Split(' ')[1]);
                //    }

                //    result.AddResult(lines[lines.Count - (finished ? 2 : 1)].Select(long.Parse).ToList());
                //    results.Add(result);
                //}

                //Console.WriteLine(directory);
                //Console.WriteLine($"Total: {results.Count}");
            }
        }
    }
}

