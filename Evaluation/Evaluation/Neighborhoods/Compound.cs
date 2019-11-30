namespace Evaluation.Neighborhoods
{
    public class Compound : INeighborhood
    {
        public int MaxLength { get; }
        public Compound(string maxLength)
        {
            MaxLength = int.Parse(maxLength);
        }
    }
}