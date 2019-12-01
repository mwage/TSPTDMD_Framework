namespace Evaluation.Neighborhoods
{
    public class Compound : INeighborhood
    {
        public string MaxLength { get; }
        public Compound(string maxLength)
        {
            MaxLength = maxLength;
        }

        public override string ToString()
        {
            return $"comp-{MaxLength}";
        }
    }
}