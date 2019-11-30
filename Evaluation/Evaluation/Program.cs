using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace Evaluation
{
    internal class Program
    {
        private const string ParsePath = @"C:\Users\mwagner\Documents\Michael\Results\HigherIndex\mca";
        private const string DeletePath = @"C:\Users\wage\Documents\Projects\Uni\ProgrammingAssignment_1\results";

        private static void Main(string[] args)
        {
            //var results = ParseResults(ParsePath);
            DeletePartialLogs(DeletePath);

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

        //private static List<IResult> ParseResults(string parseDirectory)
        //{
        //    var results = new List<IResult>();


        //    foreach (var directory in Directory.GetDirectories(path))
        //    {
        //        // Different SUTs
        //        foreach (var subDirectory in Directory.GetDirectories(directory))
        //        {
        //            // Different algorithms
        //            foreach (var fileName in Directory.EnumerateFiles(subDirectory, "*.txt"))
        //            {
        //                var splitName = fileName.Split('\\');
        //                var parameters = splitName[splitName.Length - 1].Split('.')[0].Split('_');

        //                var lines = new List<string[]>();
        //                using (var fileStream = File.OpenRead(fileName))
        //                {
        //                    using (var streamReader = new StreamReader(fileStream, Encoding.UTF8, true, 128))
        //                    {
        //                        string line;
        //                        while ((line = streamReader.ReadLine()) != null)
        //                        {
        //                            if (!string.IsNullOrWhiteSpace(line))
        //                            {
        //                                lines.Add(line.Split('\t'));
        //                            }
        //                        }
        //                    }
        //                }

        //                var result = new MCAResult(parameters);
        //                var finished = lines[lines.Count - 1][0].Contains("dont-cares");
        //                if (finished)
        //                {
        //                    result.DontCare = int.Parse(lines[lines.Count - 1][0].Split(' ')[1]);
        //                }

        //                result.AddResult(lines[lines.Count - (finished ? 2 : 1)].Select(long.Parse).ToList());
        //                results.Add(result);
        //            }
        //        }

        //        Console.WriteLine(directory);
        //        Console.WriteLine($"Total: {results.Count}");
        //    }

        //    return results;
        //}
    }
}

