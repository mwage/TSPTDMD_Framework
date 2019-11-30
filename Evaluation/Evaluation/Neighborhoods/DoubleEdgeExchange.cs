using System;
using System.Collections.Generic;
using System.Text;

namespace Evaluation.Neighborhoods
{
    public class DoubleEdgeExchange : INeighborhood
    {
        public string MaxLength { get; }
        public DoubleEdgeExchange(string maxLength)
        {
            MaxLength = maxLength;
        }
    }
}
