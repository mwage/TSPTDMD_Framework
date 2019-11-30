using System.Collections.Generic;
using System.Linq;

namespace Evaluation.Solver
{
    public class Grasp : ISolver
    {
        public int MaxBeta { get; }
        public int IterationLimit { get; }
        public LocalSearch LocalSearch { get; }
        public Dictionary<string, Result> Results { get; } = new Dictionary<string, Result>();

        public Grasp(IReadOnlyList<string> parameters)
        {
            MaxBeta = int.Parse(parameters[1]);
            IterationLimit = int.Parse(parameters[2]);
            LocalSearch = new LocalSearch(parameters.ToList().GetRange(3, 4));
        }
    }
}
