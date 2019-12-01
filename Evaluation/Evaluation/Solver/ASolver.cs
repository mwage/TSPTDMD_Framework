using System;
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

        public string GetTime(string instance)
        {
            var average = Math.Round(Results[instance].Average(x => (double)x.Time), 2);

            return "$" + average + "_{\\pm" + GetTimeStd(instance) + "}$";
        }

        private double GetTimeStd(string instance)
        {
            return Math.Round(Results[instance].Where(x => x.IsFeasible).StdDev(), 2);
        }

        public string GetBestVal(string instance)
        {
            return !Results[instance].Any(x => x.IsFeasible)
                ? "inf" 
                : "$" + Math.Round(Results[instance].Where(x => x.IsFeasible).Min(x => x.ObjValue), 2) + "$";
        }

        public string GetAverageVal(string instance)
        {
            if (!Results[instance].Any(x => x.IsFeasible))
            {
                return "inf";
            }

            var average = Math.Round(Results[instance].Where(x => x.IsFeasible)
                .Average(x => x.ObjValue), 2);

            return "$" + average + "_{\\pm" + GetStandardDeviation(instance) + "}$";
        }

        public string GetNumberOfFeasible(string instance)
        {
            return Results[instance].Count > 1 
                ? "$" + Results[instance].Sum(x => x.IsFeasible ? 1 : 0) + "$"
                : Results[instance].Single().IsFeasible.ToString();
        }

        private double GetStandardDeviation(string instance)
        {
            return Math.Round(Results[instance].Where(x => x.IsFeasible).StdDev(), 2);
        }
    }
}
