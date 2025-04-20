using System.Globalization;

namespace Learning_C_Sharp;

public class ArrayUse
{
    private static int[] CreateTestArray()
    {
        var scores = new int[] { 56, 47, 23, 89, 0 };
        return scores;
    }

    public void SimpleArrayUse()
    {
        // simple int array holding 5 entries
        int[] scores = new int[5];
    }

    public void ArrayIntUse()
    {
        Console.WriteLine("Enter number of entries to make");
        int length = Convert.ToInt32(Console.ReadLine());
        int[] array = new int[length];
        // arrays start at zero
        for (int index = 0; index <= array.Length; index++)
        {
            array[index] = 1;
        }
    }

    public void IndexingLastArrayUse()
    {
        // create an array and fill it
        var scores = CreateTestArray();
        // note first array is zero but getting last array is 1
        // get last array without knowing the length of the array
        var lastscore = scores[^1];
    }

    public int[] RangeArrayUse()
    {
        // create an array and fill it
        var scores = CreateTestArray();

        // as it gets the first range value but stops on the
        // next range number but does not include it
        // so the range will get array 0,1,2 from below
        var firstThreeScores = scores[0..3];
        return firstThreeScores;
    }

    public void ArrayExampleUse()
    {
        int[] array = [4, 51, -7, 13, -99, 15, -8, 45, 90];

        var currentSmallest = int.MaxValue; // put largest number to start
        for (int index = 0; index < array.Length; index++)
        {
            if (array[index] < currentSmallest)
            {
                currentSmallest = array[index];
            }
        }

        Console.WriteLine(currentSmallest.ToString());
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

    public void AnotherArrayExampleUse()
    {
        int[] array = new int[] { 4, 51, 13, -99, 15, -8, 45, 90 };
        int total = 0;
        for (int index = 0; index < array.Length; index++)
        {
            total += array[index];
        }

        float average = (float)total / array.Length;
        Console.WriteLine(average);
    }

    public void MultiArray1Use()
    {
        // create array
        int[][] matrix = new int[3][];
        // fill in multi array
        matrix[0] = new int[] { 1, 2 };
        matrix[1] = new int[] { 3, 4 };
        matrix[2] = new int[] { 5, 6 };
        Console.WriteLine(matrix[0][1]);
    }

    public void MultiArray2Use()
    {
        // create array
        int[][] matrix = new int[3][];
        // fill in multi array
        matrix[0] = [1, 2];
        matrix[1] = [3, 4];
        matrix[2] = [5, 6];
        Console.WriteLine(matrix[0][1]);
    }

    public void MultiArray3Use()
    {
        // create array
        int[,] matrix = new int[3, 2]
        {
            {
                1, 2
            },
            {
                3, 4
            },
            {
                5, 6
            }
        };
        Console.WriteLine(matrix[0, 1]);
    }
}