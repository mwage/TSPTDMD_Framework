using System.Collections.Generic;

namespace Evaluation.Solver
{
    public class Greedy : ASolver
    {
        public int Beta { get; }

        public Greedy(IReadOnlyList<string> parameters)
        {
            Beta = int.Parse(parameters[1]);
        }
    }
}
