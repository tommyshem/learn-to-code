namespace Learning_C_Sharp;

// a class can have only one base class
// but can implement many interfaces.
// interfaces define a contract but do not implement it.
public interface ILevelBuilder
{
    bool BuildLevel(int levelNumber);
}

// class which uses the interface above
public class FixedLwevelBuilder : ILevelBuilder
{
    public bool BuildLevel(int LevelNumber)
    {
        // build level code goes here
        // return true if level id built else false
        return true;
    }
}