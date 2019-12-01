namespace Evaluation
{
    public class Result
    {
        public double ObjValue { get; }
        public ulong Time { get; }
        public bool IsFeasible { get; }

        public Result(double objValue, ulong time, bool isFeasible)
        {
            ObjValue = objValue;
            Time = time;
            IsFeasible = isFeasible;
        }
    }
}