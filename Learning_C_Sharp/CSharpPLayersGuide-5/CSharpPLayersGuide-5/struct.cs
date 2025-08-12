namespace Learning_C_Sharp;

// structs are value types not reference types like classes
// structs are primarily useful for small data-related concepts
// structs are automatically initialized to default values
// structs should be immutable ie no setters
// primary short hand for struct if only 1 constructor is used
public struct Point(float x, float y)
{
    public float X { get; } = x;
    public float Y { get; } = y;
}

// same as above but long hand way and can have different constructors
public struct Point1
{
    public float X { get; }
    public float Y { get; }

    public Point1(float x, float y)
    {
        X = x;
        Y = y;
    }
}

public class structExample()
{
    private Point _p1 = new Point(2, 4);
    //Console.WriteLine($"({p1.X}, {p1.Y}");
    // boxing
    static Object _o = 3;
    // unboxing
    int p = (int)_o;
}
