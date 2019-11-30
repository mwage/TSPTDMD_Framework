using System;
using System.Collections.Generic;
using System.Text;

namespace Evaluation.Solver
{
    public class Pilot : ISolver
    {
        public int Beta { get; }
        public Dictionary<string, Result> Results { get; } = new Dictionary<string, Result>();

        public Pilot(IReadOnlyList<string> parameters)
        {
            Beta = int.Parse(parameters[1]);
        }
    }
}
