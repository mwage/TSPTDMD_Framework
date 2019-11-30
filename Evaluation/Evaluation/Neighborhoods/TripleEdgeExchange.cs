using System;
using System.Collections.Generic;
using System.Text;

namespace Evaluation.Neighborhoods
{
    public class TripleEdgeExchange : INeighborhood
    {
        public int MaxLength { get; }
        public TripleEdgeExchange(string maxLength)
        {
            MaxLength = int.Parse(maxLength);
        }
    }
}
