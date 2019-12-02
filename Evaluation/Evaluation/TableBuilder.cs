using Evaluation.Solver;
using System;
using System.Collections.Generic;
using System.Linq;
using Evaluation.Neighborhoods;

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

        public void BuildRandomConstructionTable()
        {
            Console.WriteLine("Build random construction table.");
            var table = new Table("random construction");

            // Get algorithms
            var greedy = _results.Single(x => x is Greedy solver && solver.Beta == 1);
            var greedy3 = _results.Single(x => x is Greedy solver && solver.Beta == 3);
            var greedy5 = _results.Single(x => x is Greedy solver && solver.Beta == 5);

            // Add columns
            table.AppendColumn(greedy, greedy.GetBestVal);
            table.AppendColumn(greedy3, greedy3.GetBestVal);
            table.AppendColumn(greedy5, greedy5.GetBestVal);

            table.AppendColumn(greedy, greedy.GetAverageVal);
            table.AppendColumn(greedy3, greedy3.GetAverageVal);
            table.AppendColumn(greedy5, greedy5.GetAverageVal);

            table.AppendColumn(greedy, greedy.GetTime);
            table.AppendColumn(greedy3, greedy3.GetTime);
            table.AppendColumn(greedy5, greedy5.GetTime);

            table.AppendColumn(greedy, greedy.GetNumberOfFeasible);
            table.AppendColumn(greedy3, greedy3.GetNumberOfFeasible);
            table.AppendColumn(greedy5, greedy5.GetNumberOfFeasible);

            _tables.Add(table);
        }

        public void BuildBestImprovementTables()
        {
            Console.WriteLine("Build best improvement tables.");

            // Get algorithms
            var df = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is DriverFlip);
            var de10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "10");
            var te10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "10");
            var comp10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is Compound nh && nh.MaxLength == "10");
            var grasp10 = _results.Single(x => x is Grasp solver && solver.MaxBeta == 10);
            var vnd10 = _results.Single(x => x is VariableNeighborhood solver && solver.Neighborhoods.First() is DoubleEdgeExchange de && de.MaxLength == "10");

            var demax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "max");
            var temax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "max");
            var compmax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.BestImprovement && solver.Neighborhood is Compound nh && nh.MaxLength == "max");
            var graspmax = _results.Single(x => x is Grasp solver && solver.MaxBeta == 10);
            var vndmax = _results.Single(x => x is VariableNeighborhood solver && solver.Neighborhoods.First() is DoubleEdgeExchange de && de.MaxLength == "10");


            var minTable = new Table("best improvement minimum");

            // Add columns
            minTable.AppendColumn(df, df.GetBestVal);
            minTable.AppendColumn(de10, de10.GetBestVal);
            minTable.AppendColumn(te10, te10.GetBestVal);
            minTable.AppendColumn(comp10, comp10.GetBestVal);
            minTable.AppendColumn(grasp10, grasp10.GetBestVal);
            minTable.AppendColumn(vnd10, vnd10.GetBestVal);
            minTable.AppendColumn(demax, demax.GetBestVal);
            minTable.AppendColumn(temax, temax.GetBestVal);
            minTable.AppendColumn(compmax, compmax.GetBestVal);
            minTable.AppendColumn(graspmax, graspmax.GetBestVal);
            minTable.AppendColumn(vndmax, vndmax.GetBestVal);

            _tables.Add(minTable);

            var timeTable = new Table("best improvement time");

            // Add columns
            timeTable.AppendColumn(df, df.GetTime);
            timeTable.AppendColumn(de10, de10.GetTime);
            timeTable.AppendColumn(te10, te10.GetTime);
            timeTable.AppendColumn(comp10, comp10.GetTime);
            timeTable.AppendColumn(grasp10, grasp10.GetTime);
            timeTable.AppendColumn(vnd10, vnd10.GetTime);
            timeTable.AppendColumn(demax, demax.GetTime);
            timeTable.AppendColumn(temax, temax.GetTime);
            timeTable.AppendColumn(compmax, compmax.GetTime);
            timeTable.AppendColumn(graspmax, graspmax.GetTime);
            timeTable.AppendColumn(vndmax, vndmax.GetTime);

            _tables.Add(timeTable);
        }

        public void BuildFirstImprovementTables()
        {
            Console.WriteLine("Build first improvement tables.");

            // Get algorithms
            var df = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is DriverFlip);
            var de10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "10");
            var te10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "10");
            var comp10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is Compound nh && nh.MaxLength == "10");

            var demax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "max");
            var temax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "max");
            var compmax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.FirstImprovement && solver.Neighborhood is Compound nh && nh.MaxLength == "max");


            var minTable = new Table("first improvement minimum");

            // Add columns
            minTable.AppendColumn(df, df.GetBestVal);
            minTable.AppendColumn(de10, de10.GetBestVal);
            minTable.AppendColumn(te10, te10.GetBestVal);
            minTable.AppendColumn(comp10, comp10.GetBestVal);
            minTable.AppendColumn(demax, demax.GetBestVal);
            minTable.AppendColumn(temax, temax.GetBestVal);
            minTable.AppendColumn(compmax, compmax.GetBestVal);

            _tables.Add(minTable);

            var timeTable = new Table("best improvement time");

            // Add columns
            timeTable.AppendColumn(df, df.GetTime);
            timeTable.AppendColumn(de10, de10.GetTime);
            timeTable.AppendColumn(te10, te10.GetTime);
            timeTable.AppendColumn(comp10, comp10.GetTime);
            timeTable.AppendColumn(demax, demax.GetTime);
            timeTable.AppendColumn(temax, temax.GetTime);
            timeTable.AppendColumn(compmax, compmax.GetTime);

            _tables.Add(timeTable);
        }

        public void BuildRandomTables()
        {
            Console.WriteLine("Build random tables.");

            // Get algorithms
            var df = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is DriverFlip);
            var de10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "10");
            var te10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "10");
            var comp10 = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is Compound nh && nh.MaxLength == "10");

            var demax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "max");
            var temax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "max");
            var compmax = _results.Single(x => x is LocalSearch solver && solver.StepFunction == StepFunction.Random && solver.Neighborhood is Compound nh && nh.MaxLength == "max");
            var sa = _results.Single(x => x is SimulatedAnnealing);


            var minTable = new Table("first improvement minimum");

            // Add columns
            minTable.AppendColumn(df, df.GetBestVal);
            minTable.AppendColumn(de10, de10.GetBestVal);
            minTable.AppendColumn(te10, te10.GetBestVal);
            minTable.AppendColumn(comp10, comp10.GetBestVal);
            minTable.AppendColumn(demax, demax.GetBestVal);
            minTable.AppendColumn(temax, temax.GetBestVal);
            minTable.AppendColumn(compmax, compmax.GetBestVal);
            minTable.AppendColumn(sa, sa.GetBestVal);

            _tables.Add(minTable);


            var meanTable = new Table("first improvement mean");

            // Add columns
            meanTable.AppendColumn(df, df.GetAverageVal);
            meanTable.AppendColumn(de10, de10.GetAverageVal);
            meanTable.AppendColumn(te10, te10.GetAverageVal);
            meanTable.AppendColumn(comp10, comp10.GetAverageVal);
            meanTable.AppendColumn(demax, demax.GetAverageVal);
            meanTable.AppendColumn(temax, temax.GetAverageVal);
            meanTable.AppendColumn(compmax, compmax.GetAverageVal);
            meanTable.AppendColumn(sa, sa.GetAverageVal);

            _tables.Add(meanTable);

            var timeTable = new Table("best improvement time");

            // Add columns
            timeTable.AppendColumn(df, df.GetTime);
            timeTable.AppendColumn(de10, de10.GetTime);
            timeTable.AppendColumn(te10, te10.GetTime);
            timeTable.AppendColumn(comp10, comp10.GetTime);
            timeTable.AppendColumn(demax, demax.GetTime);
            timeTable.AppendColumn(temax, temax.GetTime);
            timeTable.AppendColumn(compmax, compmax.GetTime);

            _tables.Add(timeTable);
        }

        public void BuildDeltaEvaluationTable()
        {
            Console.WriteLine("Build delta evaluation table.");
            var table = new Table("delta evaluation");

            // Get algorithms
            var df = _results.Single(x => x is LocalSearch solver && solver.Neighborhood is DriverFlip);
            var de = _results.Single(x => x is LocalSearch solver && solver.Neighborhood is DoubleEdgeExchange nh && nh.MaxLength == "10");
            var te = _results.Single(x => x is LocalSearch solver && solver.Neighborhood is TripleEdgeExchange nh && nh.MaxLength == "10");
            var comp = _results.Single(x => x is LocalSearch solver && solver.Neighborhood is Compound nh && nh.MaxLength == "10");

            // Add columns
            table.AppendColumn(df, df.GetTime);
            table.AppendColumn(de, de.GetTime);
            table.AppendColumn(te, te.GetTime);
            table.AppendColumn(comp, comp.GetTime);

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
