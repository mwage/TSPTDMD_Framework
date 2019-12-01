using System.Collections.Generic;

namespace Evaluation.Solver
{
    public interface ISolver
    {
        Dictionary<string, List<Result>> Results { get; }
        void AddResult(string name, Result result);
        decimal GetTime(string instance);
        double GetBestVal(string instance);
        double GetAverageVal(string instance);
        double GetWorstVal(string instance);
        double GetFeasiblePercentage(string instance);
        double GetStandardDeviation(string instance);
    }
}