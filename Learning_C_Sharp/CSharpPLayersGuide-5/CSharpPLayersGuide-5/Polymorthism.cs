namespace Learning_C_Sharp;
// keywords 
// virtual  = is used for methods which can be override
// abstract = is used for methods is used for no base implementation
// override = is used for overriding the virtual and abstract methods

class Polymorthism
{
    public class ChessPiece
    {
        public int Row { get; set; }
        public int Column { get; set; }

        // virtual keyword means it can be overridded 
        public virtual bool IsALegalMove(int row, int column)
        {
            return true;
        }

        // protected can only be called form derived classes
        protected bool IsOnBoard(int row, int column)
        {
            return true;
        }

        // protected can only be called form derived classes
        protected bool IsCurrentLocation(int row, int column)
        {
            return true;
        }
    }

    // use ChessPiece as the base case
    public class King : ChessPiece
    {
        // override keyword will override the base method
        public override bool IsALegalMove(int row, int column)
        {
            if (row == 0 && column == 0)
            {
                return true;
            }

            // this would run the base method 
            return base.IsALegalMove(row, column);
        }

        // example of the abstract class which and be overrided
        public abstract class Queen : ChessPiece
        {
            // abstract method with no implementation
            public abstract bool Move(int row, int column);
        }
    }
}