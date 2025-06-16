namespace RockPaperScissors;

class Program
{
    static void Main(string[] args)
    {
        var game = new RockPaperScissors();
        while (true)
        {
            game.NewGame();
        }
    }
}

enum Pick
{
    rock,
    paper,
    scissors
}

enum Player
{
    one,
    two,
    draw,
}

class RockPaperScissors
{
    // rock beats scissors
    // scissors beats paper
    // paper beats rock

    private int _player1Wins;
    private int _player2Wins;
    private int _draws;

    public void NewGame()
    {
        // player 1 picks
        Console.WriteLine("Player 1");
        var player1 = _player_choose();
        // player 2 picks
        Console.WriteLine("Player 2");
        var player2 = _player_choose();
        // winner is displayed
        var winner = _checkWinner(player1, player2);
        Console.WriteLine($"Winner is {winner.ToString()}");
        _displayScores();
    }

    private Player _checkWinner(Pick player1Answer, Pick player2Answer)
    {
        // check for draw
        if (player1Answer == player2Answer)
        {
            _draws += 1;
            return Player.draw;
        }

        // who wins
        if (player1Answer == Pick.paper && player2Answer == Pick.rock)
        {
            _player1Wins += 1;
            return Player.one;
        }

        if (player1Answer == Pick.rock && player2Answer == Pick.scissors)
        {
            _player1Wins += 1;
            return Player.one;
        }

        if (player1Answer == Pick.scissors && player2Answer == Pick.paper)
        {
            _player1Wins += 1;
            return Player.one;
        }

        _player2Wins += 1;
        return Player.two;
    }

    private void _displayScores()
    {
        Console.WriteLine("Scores are");
        Console.WriteLine($"Player 1 won {_player1Wins}");
        Console.WriteLine($"Player 2 won {_player2Wins}");
        Console.WriteLine($"Number of draws {_draws}");
    }

    private Pick _player_choose()
    {
        while (true)
        {
            Console.WriteLine("Pick Rock, Paper or Scissors (r,p,s)");
            var choosen = Console.ReadLine();
            if (choosen == "r" || choosen == "R")
            {
                return Pick.rock;
            }

            if (choosen == "p" || choosen == "P")
            {
                return Pick.paper;
            }

            if (choosen == "s" || choosen == "S")
            {
                return Pick.scissors;
            }

            Console.WriteLine("Please only type r,p or s for your answer");
        }
    }
}