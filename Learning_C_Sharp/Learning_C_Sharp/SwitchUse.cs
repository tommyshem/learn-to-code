namespace Learning_C_Sharp;

public class SwitchUse
{
    public void SimpleSwitchUse(int choice)
    {
        // if esle can be used instead of switch statements
        // pick the best one for the job
        switch (choice)
        {
            case 1:
                Console.WriteLine("Recover");
                break;
            case 2:
                Console.WriteLine("Riding");
                break;
            case 3:
                Console.WriteLine("Sleep");
                break;
            case 4:
                Console.WriteLine("Eat");
                break;
            default:
                Console.WriteLine("Apologies");
                break;
        }
    }

    public void SwichExpressionUse(int choice)
    {
        // same checks as above but in a switch expression format.
        string response = choice switch
        {
            1 => "Recover",
            2 => "Riding",
            3 => "Sleep",
            4 => "Eat",
            _ => "Apologies"
        };
        Console.WriteLine(response);
    }
}