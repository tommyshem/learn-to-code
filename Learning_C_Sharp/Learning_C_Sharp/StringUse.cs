namespace Learning_C_Sharp;

public class StringUse
{
    public void SimpleString()
    {
        Console.WriteLine("Enter your name");
        var name = Console.ReadLine();
        Console.WriteLine("Name: {0}", name);
    }

    public void IgnoreEscapeSequences()
    {
        // ignore the escape sequence
        var filename = @"c:\testing\filename.bat";
        Console.WriteLine("Filename: {0}", filename);
    }

    public void EscapeSequences()
    {
        // " mark
        Console.WriteLine("Double quotation mark \"");
        // tab
        Console.WriteLine("Tab \t here");
        // new line
        Console.WriteLine("new line\n again new line \n");
        // backslash 
        Console.WriteLine("backslash \\");
    }

    public void StringInterpolation()
    {
        var number = 10;
        Console.WriteLine($"Number {number}");
    }

    public void StringAlignment()
    {
        var name1 = "John";
        var name2 = "Bill";
        // alignment on the beginning
        Console.WriteLine($"#1 {name1,20}");
        Console.WriteLine($"#2 {name2,20}");
        // alignment on the end            
        Console.WriteLine($"{name1,20} -1");
        Console.WriteLine($"{name2,20} -1");
    }

    public void StringFormating()
    {
        Console.WriteLine($"{42.4567:#.##}"); // displays 42.45
        Console.WriteLine($"{42.4567:#.#}"); // displays 42.4
    }

    public void StringToInt()
    {
        var input = Console.ReadLine();
        var score = Convert.ToInt32(input);
        Console.WriteLine($"score: {score}");
    }
}