using System.Text.Unicode;

namespace Learning_C_Sharp;

public class EnumUse
{
    // Create an enum below will be winter =0 and spring =1 etc
    public enum Season
    {
        Winter,
        Spring,
        Summer,
        Fall
    }

    // enum with different numbers
    public enum DifferentSeason
    {
        Winter = 3,
        Spring = 6,
        Sumber = 9,
        Fall = 12
    }

    // simple use of the enum
    public void SimpleEnumUse()
    {
        Season season = Season.Spring;
        Console.WriteLine(season);
    }

    public void Simple1EnumUse()
    {
        var season = Season.Spring;
        Console.WriteLine(season);
    }

    public void EnumCheckingUse(Season current)
    {
        if (current == Season.Summer || current == Season.Winter)
        {
            Console.WriteLine("Happy Solstice!");
        }
        else Console.WriteLine("Happy equinox!");
    }

    public void CastingEnumUse()
    {
        // cast enum to an int
        int number = (int)Season.Fall;
        Console.WriteLine(number);
    }
}