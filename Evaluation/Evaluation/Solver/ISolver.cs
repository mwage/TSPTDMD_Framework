using System.Collections.Generic;

namespace Evaluation.Solver
{
    public interface ISolver
    {
        Dictionary<string, List<Result>> Results { get; }
        void AddResult(string name, Result result);
        string GetTime(string instance);
        string GetBestVal(string instance);
        string GetAverageVal(string instance);
        string GetNumberOfFeasible(string instance);
    }
}