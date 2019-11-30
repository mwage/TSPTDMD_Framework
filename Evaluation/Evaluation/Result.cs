namespace Evaluation
{
    public class Result
    {
        public decimal ObjValue { get; }
        public ulong Time { get; }
        public bool IsFeasible { get; }

        public Result(decimal objValue, ulong time, bool isFeasible)
        {
            ObjValue = objValue;
            Time = time;
            IsFeasible = isFeasible;
        }
    }
}