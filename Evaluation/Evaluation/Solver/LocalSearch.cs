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
            StepFunction = GetStepFunction(parameters[2]);
            Neighborhood = GetNeighborhood(parameters[3]);
        }

        private StepFunction GetStepFunction(string name)
        {
            switch (name)
            {
                case "BestImprovement":
                    return StepFunction.BestImprovement;
                case "FirstImprovement":
                    return StepFunction.FirstImprovement;
                case "Random":
                    return StepFunction.Random;
                default:
                    throw new NotImplementedException();
            }
        }

        private INeighborhood GetNeighborhood(string name)
        {
            var parameters = name.Split('-');
            switch (parameters[0])
            {
                case "te":
                    return new TripleEdgeExchange(parameters[1]);
                case "de":
                    return new DoubleEdgeExchange(parameters[1]);
                case "df":
                    return new DriverFlip();
                case "comp":
                    return new Compound(parameters[1]);
                default:
                    throw new NotImplementedException();
            }
        }
    }
}