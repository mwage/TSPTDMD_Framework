using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace Evaluation
{
    public static class ContentDeleter
    {
        private const string DeletePath = @"C:\Users\wage\Desktop\TSP Delete";
        public static void DeletePartialLogs()
        {
            foreach (var directory in Directory.GetDirectories(DeletePath))
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
    }
}
