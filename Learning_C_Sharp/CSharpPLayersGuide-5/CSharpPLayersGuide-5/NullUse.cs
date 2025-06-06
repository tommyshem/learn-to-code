namespace Learning_C_Sharp;

public class NullUse
{
    // ? is used for could contain a null value
    // ?[] is used array could be null
    // ?? use default if null

    // example of the string could contain a null
    private string? name;

    // example of the reference string passed in could be null
    private void GetSomething(string? name)
    {
        // check for null and write the name if not null
        if (name != null)
        {
            Console.WriteLine(name);
        }
    }

    // example of null coalescing operator ??
    private string PassInSomeThing(string? name)
    {
        // if name is null return "None" else return the name passed in
        // also known as the fallback value
        return name ?? "None";
    }

    // example of return might be null
    static string? PassBackSomeThing(string name)
    {
        // pass out null
        return null;
    }

    // null forgiving operator !
    // tells the compiler to ignore warnings of not check for null
    // use only when needed and you know that null will never be
    // passed in. This code might crash as it will return null.
    // and you have turn null checking off for this part of the code
    private string message = PassBackSomeThing("Bob")!;
}