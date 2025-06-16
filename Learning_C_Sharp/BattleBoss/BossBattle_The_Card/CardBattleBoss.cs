using System.Diagnostics;

namespace BossBattle_The_Card;

public class CardBattleBoss

{
    private const int _maxCards = 52;    // fields
    public Suit CardSuit { get; set; }

    public Rank CardRank { get; set; }

    // todo array create static cards list
    private static (int suit, int rank, bool used)[] _deck = new (int, int, bool) [52];

    public enum Rank
    {
        None,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    public enum Suit
    {
        None = 0,
        Red,
        Green,
        Blue,
        Yellow,
    }

    public string DisplayCard()
    {
        if (this.CardSuit == 0)
        {
            return "No Card Passed in";
        }

        const string start = "Suit: ";
        string returnString = "";
        // build up suit string
        returnString += this.CardSuit switch
        {
            Suit.None => start + "No Card",
            Suit.Blue => start + "Blue ",
            Suit.Green => start + "Green ",
            Suit.Red => start + "Red ",
            Suit.Yellow => start + "Yellow ",
            _ => start + "Card is not valid"
        };

        // build Rank string up
        returnString += "Rank: ";
        returnString += this.CardRank switch
        {
            Rank.None => "No Card",
            Rank.One => "One",
            Rank.Two => "Two",
            Rank.Three => "Three",
            Rank.Four => "Four",
            Rank.Five => "Five",
            Rank.Six => "Six",
            Rank.Seven => "Seven",
            Rank.Eight => "Eight",
            Rank.Nine => "Nine",
            Rank.Ten => "Ten",
            Rank.Jack => "Dollar",
            Rank.Queen => "Percentage",
            Rank.King => "Carret",
            Rank.Ace => "Ampersand",
            _ => "Card is not valid"
        };

        // return the built-up string for the card
        return returnString;
    }

    // Init deck of cards
    public void InitDeck()
    {
        for (int i = 1; i < 4; i++)
        {
            // rank
            for (int j = 1; j < 14; j++)
            {
                _deck[j].rank = j;
                _deck[j].used = false;
            }

            _deck[i].rank = i;
        }
    }

//todo get random number for card
    public CardBattleBoss()
    {
        // create random card
        var randomNumber = new Random();
        int rangeNumber = randomNumber.Next(0, _maxCards);
        Debug.WriteLine($"Range Number: {rangeNumber}");
        // get card details
        if (_deck[rangeNumber].used == false)
        {
            CardRank = (Rank)_deck[rangeNumber].rank;
            CardSuit = (Suit)_deck[rangeNumber].suit;
            _deck[rangeNumber].used = true;
            return;
        }
        else
        {
            // check for un used cards
            for (int index = 0; index < _maxCards; index++)
            {
                rangeNumber += index;
                if (rangeNumber == 53)
                {
                    rangeNumber = 0;}
                if (_deck[rangeNumber].used != false) continue;
                CardRank = (Rank)_deck[rangeNumber].rank;
                CardSuit = (Suit)_deck[rangeNumber].suit;
                _deck[rangeNumber].used = true;
                return;


            }
        }
        // no cards left
        this.CardSuit = 0;
        this.CardRank = 0;
    }

    // check is card is a face card 11-14
    public bool IsFaceCard(CardBattleBoss card)
    {
        int value = (int)card.CardRank;
        if (value > 10)
        {
            return true;
        }
        else return false;
    }

    // check is the card is a nunmber card 1-10
    public bool IsNumberCard(CardBattleBoss card)
    {
        return !IsFaceCard(card);
    }

// end class
}