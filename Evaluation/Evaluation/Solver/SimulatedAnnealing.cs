using System;
using System.Collections.Generic;
using System.Text;
using Evaluation.Neighborhoods;

namespace Evaluation.Solver
{
    public class SimulatedAnnealing : ASolver
    {
        public INeighborhood Neighborhood { get; }

        public SimulatedAnnealing(IReadOnlyList<string> parameters)
        {
            Neighborhood = parameters[1].ToNeighborhood();
        }

        public override string ToString()
        {
            return "sa";
        }
    }
}
