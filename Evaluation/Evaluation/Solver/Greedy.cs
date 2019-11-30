using System.Collections.Generic;

namespace Evaluation.Solver
{
    public class Greedy : ISolver
    {
        public int Beta { get; }
        public Dictionary<string, Result> Results { get; } = new Dictionary<string, Result>();

        public Greedy(IReadOnlyList<string> parameters)
        {
            Beta = int.Parse(parameters[1]);
        }
    }
}
