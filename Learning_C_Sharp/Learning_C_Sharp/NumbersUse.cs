namespace Learning_C_Sharp;

public class NumbersUse
{
    public void FloatUse()
    {
        var floatNumber = 0.7f;
        Console.WriteLine("Float number: {0}", floatNumber);
    }

    public void IntUse()
    {
        var intNumber = 5;
        var anotherIntNumber = 10;
        Console.WriteLine("Int numbers {0} and another int {1}", intNumber, anotherIntNumber);
    }

    public void UodateValues()
    {
        var x = 10;
        x += 5;
        var y = 15;
        y -= 10;
        var z = 20;
        z *= 2;
        var total = x + y + z;

        Console.WriteLine("Total value {0}", total);
    }

    public void RemainderValue()
    {
        var total = 23 % 2;
        Console.WriteLine("Remainder Value: {0}", total);
    }

    public void MathUse()
    {
        // returns lowest number
        var smallest = Math.Min(10, 453);
        var largest = Math.Max(23, 654);
        var health = 110;
        // returns the number in the range 0 to 100
        var totalHealth = Math.Clamp(health, 0, 100);
        Console.WriteLine("Total health: {0}", totalHealth);
        // absolute value returns only a positive number
        var abs = Math.Abs(-2);
        Console.WriteLine("Absolute value: {0}", abs);
    }
}