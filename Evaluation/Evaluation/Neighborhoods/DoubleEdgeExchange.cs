namespace Evaluation.Neighborhoods
{
    public class DoubleEdgeExchange : INeighborhood
    {
        public string MaxLength { get; }
        public DoubleEdgeExchange(string maxLength)
        {
            MaxLength = maxLength;
        }

        public override string ToString()
        {
            return $"de_{MaxLength}";
        }
    }
}
