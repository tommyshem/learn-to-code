namespace Learning_C_Sharp;

public class ConsoleUse
{
    public void ChangeTitle()
    {
        // change title on the console
        Console.Title = "Testing console ";
    }

    public void ChangeTextColour()
    {
        Console.BackgroundColor = ConsoleColor.Yellow;
        Console.ForegroundColor = ConsoleColor.Red;
        Console.WriteLine("test colours used");
    }

    public void ClearScreen()
    {
        Console.WriteLine("press a key to clear the screen");
        Console.ReadKey();
        Console.Clear();
    }

    public void Beep()
    {
        Console.WriteLine("press key to beep");
        Console.ReadKey();
        Console.Beep();
        // windows only beep frequency and duration 
        Console.Beep(440, 1000);
    }
}