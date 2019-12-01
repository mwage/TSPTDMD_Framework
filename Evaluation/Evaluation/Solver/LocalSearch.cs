using System;
using System.Collections.Generic;
using System.Text;
using Evaluation.Neighborhoods;

namespace Evaluation.Solver
{
    public class LocalSearch : ASolver
    {
        public int IterationLimit { get; }
        public StepFunction StepFunction { get; }
        public INeighborhood Neighborhood { get; }

        public LocalSearch(IReadOnlyList<string> parameters)
        {
            IterationLimit = int.Parse(parameters[1]);
            StepFunction = parameters[2].ToStepFunction();
            Neighborhood = parameters[3].ToNeighborhood();
        }

        public override string ToString()
        {
            return $"{Neighborhood}";
        }
    }
}