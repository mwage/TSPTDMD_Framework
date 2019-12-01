using System.Collections.Generic;
using System.Linq;

namespace Evaluation.Solver
{
    public abstract class ASolver : ISolver
    {
        public Dictionary<string, List<Result>> Results { get; } = new Dictionary<string, List<Result>>();

        public void AddResult(string name, Result result)
        {
            if (!Results.ContainsKey(name))
            {
                Results.Add(name, new List<Result>());
            }
            Results[name].Add(result);
        }

        public decimal GetTime(string instance)
        {
            return Results[instance].Average(x => (decimal)x.Time);
        }

        public double GetBestVal(string instance)
        {
            return Results[instance].Min(x => x.ObjValue);
        }

        public double GetAverageVal(string instance)
        {
            return Results[instance].Average(x => x.ObjValue);
        }

        public double GetWorstVal(string instance)
        {
            return Results[instance].Max(x => x.ObjValue);
        }

        public double GetFeasiblePercentage(string instance)
        {
            var feasible = Results[instance].Sum(x => x.IsFeasible ? 1: 0);
            return (double) feasible / Results[instance].Count;
        }

        public double GetStandardDeviation(string instance)
        {
            return Results[instance].StdDev();
        }
    }
}
