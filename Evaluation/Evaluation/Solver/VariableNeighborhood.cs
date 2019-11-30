using System;
using System.Collections.Generic;
using System.Text;
using Evaluation.Neighborhoods;

namespace Evaluation.Solver
{
    public class VariableNeighborhood : ISolver
    {
        public Dictionary<string, Result> Results { get; } = new Dictionary<string, Result>();
        public List<INeighborhood> Neighborhoods { get; } = new List<INeighborhood>();

        public VariableNeighborhood(IReadOnlyList<string> parameters)
        {
            for (var i = 1; i < parameters.Count; i++)
            {
                Neighborhoods.Add(parameters[i].ToNeighborhood());
            }
        }
    }
}
