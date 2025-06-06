namespace Inheritance;

class Program
{
    static void Main(string[] args)
    {
        // create a list of different gameObjects
        GameObject[] gameObjects = new GameObject[]
        {
            new Ship(), new Rock(), new Rock()
        };
    // you can loop through all baseObjects as they are guaranteed 
    // to have an update method.
        foreach (GameObject item in gameObjects)
        {
            item.Update();
         // get object type and compare with a known class type
         if (item.GetType() == typeof(Rock))
         {
             // Object is a rock
             Console.WriteLine("Object is a rock");
         }
        }
    }
    // Base Object used for all the game objects
    public class GameObject
    {
        // protected is any class derived from this class 
        // can access the fields
        protected float PositionX { get; set; }
        protected float PoistionY { get; set; }
        public float VelocityX { get;protected set; }
        public float VelocityY { get;protected set; }

        // constructor
        protected GameObject()
        {
            PositionX = 2;
            PoistionY = 4;
        }
// constructor with parameters
protected GameObject(float x, float y)
        {
            PositionX = x;
            PoistionY = y;
        }
        public void Update()
        {
            PositionX += VelocityX;
            PoistionY += VelocityY;
        }
    }

    // inherit from BaseClass.
    // csharp only allows one base class 
    // but that base class can have a base class and so on.
    // sealed class is nothing can be derived from this class
    sealed class Ship : GameObject
    {
        public float Size { get; }
        public float RotationAngle { get; }

        // constructor
        public Ship() 
        {
            
        }
// contructor with parameters which pass them down to the base
// class with the keyword base.
        public Ship(float x, float y) : base(x,y)
        {
            
        }
    }

   public class Rock : GameObject
    {
        public int ID { get; }
    }
    
    
}