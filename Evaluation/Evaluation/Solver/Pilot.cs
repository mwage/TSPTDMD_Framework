using System.Collections.Generic;

namespace Evaluation.Solver
{
    public class Pilot : ASolver
    {
        public int Beta { get; }
        public Pilot(IReadOnlyList<string> parameters)
        {
            Beta = int.Parse(parameters[1]);
        }

        public override string ToString()
        {
            return $"pilot_{Beta}";
        }
    }
}
