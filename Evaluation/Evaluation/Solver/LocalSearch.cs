using System;
using System.Collections.Generic;
using System.Text;
using Evaluation.Neighborhoods;

namespace Evaluation.Solver
{
    public class LocalSearch : ISolver
    {
        public int IterationLimit { get; }
        public StepFunction StepFunction { get; }
        public INeighborhood Neighborhood { get; }
        public Dictionary<string, Result> Results { get; } = new Dictionary<string, Result>();

        public LocalSearch(IReadOnlyList<string> parameters)
        {
            IterationLimit = int.Parse(parameters[1]);
            StepFunction = parameters[2].ToStepFunction();
            Neighborhood = parameters[3].ToNeighborhood();
        }
    }
}