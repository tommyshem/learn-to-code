using System.Security.Cryptography;

namespace Learning_C_Sharp;

public class TuplesUse
{
    public void SimpleTupleUse()
    {
        // simple tuple with no names
        // note tuples uses brackets
        (string, int, int) score = ("R2-D2", 1248, 15);
        // access tuple with the default names ie Item1, Item2 and so on
        Console.WriteLine($"Name: {score.Item1} Level: {score.Item3} Score: {score.Item2}");
    }

    // for tuples, names are only cosmetic 
    public void NamingTuplesUse()
    {
        // naming tuple
        (string Name, int Points, int Level) score = ("R2-D2", 1248, 15);
        // use the names as created above
        Console.WriteLine($"Name: {score.Name} Level: {score.Level} Score: {score.Points}");
    }

    public void Naming1TuplesUse()
    {
        // naming tuples with the var keyword and putting the names on the formed side
        var score = (Name: "R2-D2", Points: 1248, Level: 15);
        // use the names as created above
        Console.WriteLine($"Name: {score.Name} Level: {score.Level} Score: {score.Points}");
    }

    public void Naming2TuplesUse()
    {
        // naming tuple on the right side as no effect if you named them
        // on the left side when creating the tuple
        (string N, int P, int L) score = (Name: "R2-D2", Points: 1248, Level: 15);
        // use the names as created above
        Console.WriteLine($"Name: {score.N} Level: {score.L} Score: {score.P}");
    }

    // Method with tuple with no names
    public void MethodWithTupleUse((string, int, int) score)
    {
        Console.WriteLine($"Name: {score.Item1} Level: {score.Item2} Points: {score.Item3}");
    }

    // method with named tuple use
    public void MethodWithNamedTupleUse((string Name, int Level, int Points) score)
    {
        Console.WriteLine($"Name: {score.Item1} Level: {score.Item2} Points: {score.Item3}");
    }

    // return tuple with no names
    public (string, int, int) ReturnTupleMethodUse()
    {
        return ("R2-D2", 12, 56);
    }

    public (string Name, int Level, int Points) ReturnNamedTupleMethodUse()
    {
        return ("R2-D2", 12, 56);
    }

    public enum TileType
    {
        Grass,
        Water,
        Rock
    }

    // tuple with an enum in it
    public void DifferentTypesInTuplesUse()
    {
        var Tile = (Row: 2, Column: 5, Type: TileType.Grass);
        Console.WriteLine(Tile);
    }

    public void UnpackingTupleUse()
    {
        // create tuple
        var score = ("R2-D2", 1248, 15);
        string name;
        int points;
        int level;
        // unpack tuple in to separate variables
        (name, points, level) = score;
        // single variables can now be used
        Console.WriteLine($"Name: {name}level: {level} Pints: {points}");
    }

    public void Unpacking1TupleUse()
    {
        // create tuple
        var score = ("R2-D2", 1248, 15);
        // unpack tuple in to separate variables
        (string name, int points, int level) = score;
        // single variables can now be used
        Console.WriteLine($"Name: {name}level: {level} Pints: {points}");
    }

    public void DiscardsTupleUse()
    {
        var currentscore = ("R2-D2", 1248, 15);
        // the compile will make up an name in the background
        // but unused names will not clutter up the code
        // note _ is used to discard the value
        (string name, int points, int _) score = currentscore;
    }

    public void EqualityTupleUse()
    {
        // create 2 different tuples
        (int, int) a = (1, 2);
        (int, int) b = (1, 2);
        // will be true as tuples only compare values not references
        Console.WriteLine(a == b);
    }

    public void TupleArray()
    {
        // create tuple array
        (int suit, int rank, bool used)[] Deck = new (int, int, bool) [52];
    }
}