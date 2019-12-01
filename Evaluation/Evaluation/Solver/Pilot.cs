using System;
using System.Collections.Generic;
using System.Text;

namespace Evaluation.Solver
{
    public class Pilot : ASolver
    {
        public int Beta { get; }
        public Pilot(IReadOnlyList<string> parameters)
        {
            Beta = int.Parse(parameters[1]);
        }
    }
}
