using Evaluation.Solver;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Evaluation
{
    public class TableBuilder
    {
        private readonly List<Table> _tables = new List<Table>();
        private readonly List<ISolver> _results;

        public TableBuilder(List<ISolver> results)
        {
            _results = results;
        }
        public void BuildDeterministicTable()
        {
            Console.WriteLine("Build deterministic construction table.");
            var table = new Table("deterministic construction");

            // Get algorithms
            var greedy = _results.Single(x => x is Greedy solver && solver.Beta == 1);
            var pilot1 = _results.Single(x => x is Pilot solver && solver.Beta == 1);
            var pilot5 = _results.Single(x => x is Pilot solver && solver.Beta == 5);
            var pilot15 = _results.Single(x => x is Pilot solver && solver.Beta == 15);
            var pilot50 = _results.Single(x => x is Pilot solver && solver.Beta == 50);

            // Add columns
            table.AppendColumn(greedy, greedy.GetBestVal);
            table.AppendColumn(pilot1, pilot1.GetBestVal);
            table.AppendColumn(pilot5, pilot5.GetBestVal);
            table.AppendColumn(pilot15, pilot15.GetBestVal);
            table.AppendColumn(pilot50, pilot50.GetBestVal);

            table.AppendColumn(greedy, greedy.GetTime);
            table.AppendColumn(pilot1, pilot1.GetTime);
            table.AppendColumn(pilot5, pilot5.GetTime);
            table.AppendColumn(pilot15, pilot15.GetTime);
            table.AppendColumn(pilot50, pilot50.GetTime);

            table.AppendColumn(greedy, greedy.GetNumberOfFeasible);
            table.AppendColumn(pilot1, pilot1.GetNumberOfFeasible);
            table.AppendColumn(pilot5, pilot5.GetNumberOfFeasible);
            table.AppendColumn(pilot15, pilot15.GetNumberOfFeasible);
            table.AppendColumn(pilot50, pilot50.GetNumberOfFeasible);

            _tables.Add(table);
        }

        public void PrintTables()
        {
            foreach (var table in _tables)
            {
                Console.WriteLine($"........................ {table.Name} ........................");
                table.Print();
                Console.WriteLine();
            }
        }

        public void ToFile()
        {
            foreach (var table in _tables)
            {
                table.ToFile();
            }
        }
    }
}
