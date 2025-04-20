namespace Learning_C_Sharp;

internal class Program
{
    private static void Main(string[] args)
    {
        // functions below are for examples of strings
        var stringUse = new StringUse();
        stringUse.SimpleString();
        stringUse.StringAlignment();
        stringUse.StringFormating();
        stringUse.StringToInt();
        stringUse.StringInterpolation();
        stringUse.EscapeSequences();
        stringUse.IgnoreEscapeSequences();
        // numbers exmples
        var numbersUse = new NumbersUse();
        numbersUse.MathUse();
        numbersUse.FloatUse();
        numbersUse.IntUse();
        numbersUse.RemainderValue();
        numbersUse.UodateValues();
        // comnsole examples
        var consoleUse = new ConsoleUse();
        consoleUse.ChangeTitle();
        consoleUse.Beep();
        consoleUse.ClearScreen();
        consoleUse.ChangeTextColour();
        // logic examples
        var logic = new Logic();
        logic.BoolUse();
        // if examples
        var ifUse = new IfUse();
        ifUse.SimpleIf(100);
        ifUse.IfElseUse(20);
        ifUse.IfElseIfUse(80);
        ifUse.IfBoolUse(true);
        ifUse.IfAndUse(30, 25);
        ifUse.IfOrUse(12, 8);
        ifUse.IfNestingUse(0, 0);
        ifUse.ConditionalOperatorUse(74);
        // switch examples
        var switchUse = new SwitchUse();
        switchUse.SimpleSwitchUse(45);
        switchUse.SwichExpressionUse(45);
        // loop examples
        var loopUse = new LoopsUse();
        loopUse.SimpleDoUse();
        loopUse.SimpleForUse();
        loopUse.WhileOrUse();
        loopUse.SimpleWhileUse();
        loopUse.SimpleNestingLoopsUse();
        loopUse.NestingLoopUse();
        // array examples
        var arrayUse = new ArrayUse();
        arrayUse.ArrayIntUse();
        arrayUse.SimpleArrayUse();
        arrayUse.RangeArrayUse();
        arrayUse.IndexingLastArrayUse();
        arrayUse.ArrayExampleUse();
        arrayUse.ForEachUse();
        arrayUse.AnotherArrayExampleUse();
        arrayUse.MultiArray1Use();
        arrayUse.MultiArray2Use();
        arrayUse.MultiArray3Use();
        // enum examples
        var enumUse = new EnumUse();
        enumUse.SimpleEnumUse();
        enumUse.Simple1EnumUse();
        enumUse.EnumCheckingUse(EnumUse.Season.Winter);
        // tuples examples
        var tuplesUse = new TuplesUse();
        tuplesUse.SimpleTupleUse();
        tuplesUse.NamingTuplesUse();
        tuplesUse.Naming1TuplesUse();
        tuplesUse.Naming2TuplesUse();
        tuplesUse.MethodWithTupleUse(("R2-D2", 1248, 15));
        tuplesUse.MethodWithNamedTupleUse(("R2-D2", 1248, 15));
        (string, int, int) score0 = tuplesUse.ReturnTupleMethodUse();
        Console.WriteLine(score0.Item1);
        (string Name, int Level, int Points) score1 = tuplesUse.ReturnNamedTupleMethodUse();
        Console.WriteLine(score1.Name);
        tuplesUse.DifferentTypesInTuplesUse();
        tuplesUse.UnpackingTupleUse();
        tuplesUse.Unpacking1TupleUse();
        tuplesUse.DiscardsTupleUse();
        tuplesUse.EqualityTupleUse();
        // class exaples
        var classUse = new ClassUse();
        classUse.SimpleClassScoreUse();
        classUse.ScoreWithConstructorUse();
        classUse.ClassPropertiesUse();
        classUse.ClassAutoPropertiesUse();
        classUse.GettersAndSettersUse();
        classUse.ClassAutoPropertiesWithReadOnly();
        // null examples
        var nullUse = new NullUse();
    }
}