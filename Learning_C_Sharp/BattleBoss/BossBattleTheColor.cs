namespace Learning_C_Sharp.BattleBoss;

public class BossBattleTheColor
{
    // create a color class
    // each channel 0-255
    // properteries red, green, blue

    class Color
    {
        private byte _red;
        private byte _green;
        private byte _blue;

        public static readonly (byte, byte, byte) White = (255, 255, 255);
        public static readonly (byte, byte, byte) Black = (0, 0, 0);
        public static readonly (byte, byte, byte) Red = (255, 0, 0);
        public static readonly (byte, byte, byte) Green = (0, 255, 0);
        public static readonly (byte, byte, byte) Blue = (0, 0, 255);
        public static readonly (byte, byte, byte) Yellow = (255, 255, 0);
        public static readonly (byte, byte, byte) Magenta = (255, 0, 255);
        public static readonly (byte, byte, byte) Cyan = (0, 255, 255);
        public static readonly (byte, byte, byte) Purple = (128, 0, 128);


        public Color(ValueTuple<byte, byte, byte> color)
        {
            this.red = color.Item1;
            this.green = color.Item2;
            this.blue = color.Item3;
        }

        public Color(byte red, byte green, byte blue)
        {
            this.red = red;
            this.green = green;
            this.blue = blue;
        }

        public byte red
        {
            get => _red;
            set => _red = value;
        }

        public byte green
        {
            get => _green;
            set => _green = value;
        }

        public byte blue
        {
            get => _blue;
            set => _blue = value;
        }

        public void Display()
        {
            Console.WriteLine($"Red: {_red} Green: {_green} Blue: {_blue}");
        }
    }

    public void ColorUse()
    {
        // create new color
        var color = new Color(24, 231, 42);
        color.Display();
        // create new color
        var colorWhite = new Color(Color.White);
        colorWhite.Display();
        // create new color
        var colorBlack = new Color(Color.Black);
        colorBlack.Display();
        // create new color
        var colorRed = new Color(Color.Red);
        colorRed.Display();
        // create new color
        var colorGreen = new Color(Color.Green);
        colorGreen.Display();
        // create new color
        var colorBlue = new Color(Color.Blue);
        colorBlue.Display();
        // create new color
        var colorYellow = new Color(Color.Yellow);
        colorYellow.Display();
        // create new color
        var colorMagenta = new Color(Color.Magenta);
        colorMagenta.Display();
        // create new color
        var colorCyan = new Color(Color.Cyan);
        colorCyan.Display();
        // create new color
        var colorPurple = new Color(Color.Purple);
        colorPurple.Display();
    }
}