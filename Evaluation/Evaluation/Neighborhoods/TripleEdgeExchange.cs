using System;
using System.Collections.Generic;
using System.Text;

namespace Evaluation.Neighborhoods
{
    public class TripleEdgeExchange : INeighborhood
    {
        public string MaxLength { get; }
        public TripleEdgeExchange(string maxLength)
        {
            MaxLength = maxLength;
        }
    }
}
