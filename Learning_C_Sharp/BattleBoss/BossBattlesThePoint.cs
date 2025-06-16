namespace Learning_C_Sharp.BattleBoss;

public class BossBattlesThePoint
{
    // Boss Battle --- The Point ---
    // define a new point class with properties for x and y

    class Point
    {
        public int X;
        public int Y;

        public Point()
        {
            this.X = 0;
            this.Y = 0;
        }

        public Point(int x, int y)
        {
            this.X = x;
            this.Y = y;
        }
    }

    public void PointUse()
    {
        // create new point
        var pointUse = new Point(2, 3);
        // create another point
        var pointUse1 = new Point(-4, 0);
        // create point with no parameters
        var pointUse2 = new Point();
    }
}