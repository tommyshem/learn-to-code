namespace Learning_C_Sharp;

public class LoopsUse
{
    public void SimpleWhileUse()
    {
        int x = 1;
        // check value first before running the loop
        while (x < 5)
        {
            Console.WriteLine(x.ToString());
            x++;
        }
    }

    public void WhileOrUse()
    {
        int playerNumber = -1;
        while (playerNumber is < 0 or > 10)
        {
            Console.WriteLine("Enter a Number between 0 to 10");
            string response = Console.ReadLine();
            playerNumber = Convert.ToInt32(response);
        }
    }

    public void SimpleDoUse()
    {
        int playerNumber;
        // will loop at least once before changing condition.
        do
        {
            Console.WriteLine("Enter a Number between 0 to 10");
            string response = Console.ReadLine();
            playerNumber = Convert.ToInt32(response);
        } while (playerNumber is < 0 or > 10);
    }

    public void SimpleForUse()
    {
        for (int x = 1; x <= 5; x++)
        {
            Console.WriteLine(x.ToString());
        }
    }

    public void WhileForeverLoop()
    {
        // This will loop forever unless you use break to exit loop
        // using continue to skip current loop and go onto the next loop
        while (true)
        {
            Console.WriteLine("Think of a number and type it here");
            string input = Console.ReadLine();

            if (input == "quit" || input == "exit")
            {
                break;
            }

            int number = Convert.ToInt32(input);
            if (number == 12)
            {
                Console.WriteLine("I don't like that number. Pick again");
                continue;
            }

            Console.WriteLine($"I like {number}. It's the one before {number + 1}!");
        }
    }

    public void SimpleNestingLoopsUse()
    {
        for (int a = 1; a <= 10; a++)
        for (int b = 1; b <= 10; b++)
            Console.WriteLine($"{a} * {b} = {a * b}");
    }

    public void NestingLoopUse()
    {
        int totalRows = 10;
        int totalColumns = 10;

        for (int currentRow = 1; currentRow <= totalRows; currentRow++)
        {
            for (int currentColumn = 1; currentColumn <= totalColumns; currentColumn++)
            {
                Console.WriteLine("*");
            }

            Console.WriteLine();
        }
    }

    // used for looping over collection, runs slightly slower than a for loop
    public void ForEachUse()
    {
        int[] scores = new int[10];
        foreach (var score in scores)
        {
            Console.WriteLine($"Score {score}");
        }
    }
}