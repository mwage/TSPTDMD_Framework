namespace Evaluation.Neighborhoods
{
    public class TripleEdgeExchange : INeighborhood
    {
        public string MaxLength { get; }
        public TripleEdgeExchange(string maxLength)
        {
            MaxLength = maxLength;
        }

        public override string ToString()
        {
            return $"te_{MaxLength}";
        }
    }
}
