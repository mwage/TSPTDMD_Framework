using Evaluation.Neighborhoods;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Evaluation
{
    public static class Helper
    {
        public static StepFunction ToStepFunction(this string name)
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

        public static INeighborhood ToNeighborhood(this string name)
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
        public static double StdDev(this IEnumerable<Result> results)
        {
            var count = results.Count();
            if (count <= 1) return 0;

            var avg = results.Average(x => x.ObjValue);
            var sum = results.Sum(d => (d.ObjValue - avg) * (d.ObjValue - avg));

            return Math.Sqrt(sum / count);
        }
    }
}
