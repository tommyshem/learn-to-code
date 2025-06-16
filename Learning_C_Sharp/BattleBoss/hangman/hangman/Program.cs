namespace hangman;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");
    }


    class Hangman
    {
        private string _word;

        private int seed = 5;
        // number of max guesses 
        private int _maxGuesses = 5;
        // list of words to guess from
        private string [] _list  = new string[]{
            "summery","hangman","potential"
        };

        public void NewGame()
        {
         // pick word form a list
         var word = new Random(seed);
         word.Next(_list.Length);
         
        }
    }
}