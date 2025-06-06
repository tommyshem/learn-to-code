using System.Security.Cryptography.X509Certificates;

namespace Learning_C_Sharp;

public class ClassUse
{
    // simple class with 1 method and no constructors defined
    class Score
    {
        public string name;
        public int level;
        public int points;

        public bool EarnedStar() => (level * points) > 1000;
    }

    // same as above but inlined values used
    class ScoreInline
    {
        public string name = "Unknown";
        public int level = 1;
        public int points = 0;

        public bool EarnedStar() => (level * points) > 1000;
    }

    // ********************************************************************
    // class with constructors
    class ScoreWithConstructor
    {
        public string name;
        public int level;
        public int points;

        // constructor must be the same name as the class name
        public ScoreWithConstructor(string _name, int _level, int _points)
        {
            name = _name;
            level = _level;
            points = _points;
        }

        // can have multi constructors with different signatures
        public ScoreWithConstructor()
        {
            name = "Unnown";
            level = 1;
            points = 0;
        }

        public bool EarnedStar() => (level * points) > 1000;
    }

    // ******************************************************************
    class ScoreCallingOtherConstructors
    {
        private string name;
        private int level;
        private int points;

        // call this other constructor with default values
        public ScoreCallingOtherConstructors() : this("Unknown", 1, 0)
        {
        }

        public ScoreCallingOtherConstructors(string name, int level, int points)
        {
            this.name = name;
            this.level = level;
            this.points = points;
        }

        public bool EarnedStar() => (level * points) > 1000;
    }

    // *******************************************************************
    // using getters and setters for fields to keep the encapsulation
    class Rectangle
    {
        private float _width;
        private float _height;

        public Rectangle(float width, float height)
        {
            _width = width;
            _height = height;
        }

        // getters
        public float GetWidth() => _width;
        public float GetHeight() => _height;

        public float GetArea() => _width * _height;

        // setters
        public void SetWidth(float value) => _width = value;
        public void SetHeight(float value) => _height = value;
    }

    // *************************************************************
    // same as above but using properties instead
    class RectangleUsingProperties
    {
        private float _width = 0;
        private float _height = 0;

        // width property
        public float Width
        {
            get => _width;
            set => _width = value;
        }

        // height property same as above but this scope so more than onle liner
        // can be used
        public float Height
        {
            get { return _height; }
            set { _height = value; }
        }

        // another property but only a getter
        public float Area => _height * _width;
    }

    // *****************************************************************
    // same as above but using auto properties instead
    class RectangleUsingAutoProperties
    {
        private float _width = 0;
        private float _height = 0;

        // width property
        public float Width { get; set; }

        // height property same as above 
        public float Height { get; set; }

        // another property but only a getter
        public float Area => _height * _width;
    }

    // *******************************************************************
    // same as above but using auto properties instead
    class RectangleUsingAutoPropertiesAndReadOnly(string name)
    {
        // readonly can not be changed when the init value has been set
        private float _width = 0;
        private float _height = 0;

        // get name of the object
        public string GetName()
        {
            return name;
        }

        // width property
        public float Width { get; set; }

        // height property same as above 
        public float Height { get; set; }

        // another property but only a getter
        public float Area => _height * _width;
    }

    // static class means you can not make more with the new keyword
    static class StaticClass
    {
        // static fields only have 1 instance
        private static readonly int MaxLevel = 10000;

        // static properties only have 1 instance
        public static int LevelMax { get; } = 10000;

        // static methods
        public static int CoutForPlayers()
        {
            // run code on many objects
            // return result
            return 0;
        }
    }

    // ****************************************************************
    // *****************************************************************
    /// <summary>
    /// Functions for using the classes above
    /// </summary>
    // use score class and access fields
    public void SimpleClassScoreUse()
    {
        var score = new Score
        {
            level = 1,
            points = 100,
            name = "Bob"
        };
    }

    public void ScoreWithConstructorUse()
    {
        var scoreWithConstructor = new ScoreWithConstructor("Bob", 1, 100);
    }

    public void GettersAndSettersUse()
    {
        // create a class with getters and setters
        var rectangle = new Rectangle(4.0f, 7.3f);
        // use getter
        Console.WriteLine(rectangle.GetArea());
        // use setter
        rectangle.SetHeight(4.5f);
    }

    public void ClassPropertiesUse()
    {
        var rectangle = new RectangleUsingProperties
        {
            Width = 43f,
            Height = 23f
        };
        Console.WriteLine(rectangle.Area);
    }

    public void ClassAutoPropertiesUse()
    {
        var rectangle = new RectangleUsingAutoProperties()
            // object initializer
            {
                Width = 43f,
                Height = 23f
            };
        Console.WriteLine(rectangle.Area);
    }

    public void ClassAutoPropertiesWithReadOnly()
    {
        var rectangle = new RectangleUsingAutoPropertiesAndReadOnly("Bob");
        Console.WriteLine(rectangle.GetName());
        rectangle.Height = 5f;
        rectangle.Width = 6f;
    }
}