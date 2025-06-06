namespace Learning_C_Sharp;

public struct Point
{
    public float X { get; }
    public float Y { get; }

    public Point(float x, float y)
    {
        X = x;
        Y = y;
    }
}

public class structExample()
{
    private Point p1 = new Point(2, 4);
   // Console.WriteLine($"({p1.X}, {p1.Y}");
}