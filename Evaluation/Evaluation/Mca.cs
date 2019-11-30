//using System;
//using System.Collections.Generic;
//using System.IO;
//using System.Linq;
//using System.Text;

//namespace Evaluation
//{
//    public class Mca
//    {
//        private readonly char _separator;
//        private readonly bool _getTime;

//        public Mca(char separator, bool getTime)
//        {
//            _separator = separator;
//            _getTime = getTime;
//        }

//        public void Get()
//        {
      

//            var resultLines = new List<StringBuilder>();
//            var suts = Program.GetSUTs();

//            // Header
//            resultLines.Add(new StringBuilder($"{_separator}{_separator}"));
//            resultLines.Add(new StringBuilder($"{_separator}t{_separator}"));

//            foreach (var sut in suts)
//            {
//                resultLines.Add(new StringBuilder($"{$"\\multirow{{3}}{{*}}{{{sut}}}"}{_separator}${2}${_separator}"));
//                for (var t = 3; t <= 5; t++)
//                {
//                    resultLines.Add(new StringBuilder($" {_separator}${t}${_separator}"));
//                }
//            }

//            // Results
//            GetResults(results, resultLines);

//            var resType = _getTime ? "Time" : "Rows";
//            var fileEnd = _separator == '\t' ? ".tsv" : _separator == ',' ? ".csv" : ".txt";
//            File.WriteAllLines(path + $@"\MCA_{resType}{fileEnd}", resultLines.Select(x => x.ToString()).ToArray());
//        }

//        private void GetResults(IReadOnlyCollection<MCAResult> results, IReadOnlyList<StringBuilder> resultLines)
//        {
//            var algs = new[] { "ipog", "ipog-f", "ipog-f2", "Density" };
//            var suts = Program.GetSUTs();

//            for (var i = 1; i <= 4; i++)
//            {
//                var sameIndexResults = results.Where(r => r.Index == i).ToList();

//                foreach (var alg in algs)
//                {
//                    resultLines[0].Append($"{i}{_separator}");
//                    resultLines[1].Append($"{alg}{_separator}");
//                    var sameAlgResults = sameIndexResults.Where(r => r.Algorithm == alg).ToList();

//                    var counter = 2;
//                    foreach (var sut in suts)
//                    {
//                        for (var t = 2; t <= 5; t++)
//                        {
//                            var result = sameAlgResults.SingleOrDefault(r => r.SUT == sut && r.T == t);
//                            resultLines[counter].Append(result != null ? _getTime ? result.RunTimeToString(_separator) : result.RowsToString(_separator) : $"-{_separator}");
//                            counter++;
//                        }
//                    }
//                }
//            }
//        }
//    }
//}
