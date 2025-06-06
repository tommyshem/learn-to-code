namespace Learning_C_Sharp;

public class IfUse
{
    public bool SimpleIf(int score)
    {
        if (score == 100)
        {
            Console.WriteLine("score is 100");
            return true;
        }

        return false;
    }

    public bool IfElseUse(int score)
    {
        if (score == 100)
        {
            Console.WriteLine("score is 100");
            return true;
        }

        Console.WriteLine("score is not 100");
        return false;
    }

    public void IfElseIfUse(int score)
    {
        if (score == 100)
            Console.WriteLine("Perject score");
        else if (score < 20)
            Console.WriteLine("Try harder");
        else if (score < 60)
            Console.WriteLine("Good effect");
        else Console.WriteLine("Great Score");
    }

    public void IfBoolUse(bool passed)
    {
        // test if true example
        if (passed) Console.WriteLine("You Have Passed");

        // test if false example
        if (!passed) Console.WriteLine("You failed");
    }

    public void IfAndUse(int health, int armor)
    {
        if (health <= 10 && armor <= 0) Console.WriteLine("You are dead!");
    }

    public void IfOrUse(int health, int armor)
    {
        if (health > 0 || armor > 0) Console.WriteLine("You are still alive keep going!");
    }

    public void IfNestingUse(int shields, int armor)
    {
        if (shields <= 0)
        {
            if (armor <= 0)
                Console.WriteLine("Shields and armor at zero! You`re dead!");
            else
                Console.WriteLine("Shields are gone, but armor is keeping you alive!");
        }
        else
            Console.WriteLine("You Still have shields left. The world is safe!");
    }

    public void ConditionalOperatorUse(int score)
    {
        // also known as the tenary operator
        // test condition ? run if true: run if false
        string textToDisplay = score > 70 ? "You passed!" : "you Failed";
    }
}