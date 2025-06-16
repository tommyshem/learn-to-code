namespace BossBattle_The_Card;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Battle Boss The Card");
        // create card 
        var card = new CardBattleBoss();
        card.InitDeck();
        Console.WriteLine( card.DisplayCard());
        var card1 = new CardBattleBoss();
            Console.WriteLine(card1.DisplayCard());

    }
}