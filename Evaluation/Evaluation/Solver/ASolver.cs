using System;
using System.Collections.Generic;
using System.Text;

namespace Evaluation.Solver
{
    public abstract class ASolver : ISolver
    {
        public Dictionary<string, List<Result>> Results { get; } = new Dictionary<string, List<Result>>();

        public void AddResult(string name, Result result)
        {
            if (!Results.ContainsKey(name))
            {
                Results.Add(name, new List<Result>());
            }
            Results[name].Add(result);
        }
    }
}
