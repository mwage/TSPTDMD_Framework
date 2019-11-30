using System;
using System.Collections.Generic;
using System.Text;
using Evaluation.Neighborhoods;

namespace Evaluation.Solver
{
    public class SimulatedAnnealing : ISolver
    {
        public INeighborhood Neighborhood { get; }
        public Dictionary<string, Result> Results { get; } = new Dictionary<string, Result>();

        public SimulatedAnnealing(IReadOnlyList<string> parameters)
        {
            Neighborhood = parameters[1].ToNeighborhood();
        }
    }
}
