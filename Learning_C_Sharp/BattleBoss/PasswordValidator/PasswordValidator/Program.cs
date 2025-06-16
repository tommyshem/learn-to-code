namespace PasswordValidator_1;

class Program
{
    static void Main(string[] args)
    {
        var passwordvalidator = new PasswordValidator();
        var password = passwordvalidator.GetPassword();
        var passed = passwordvalidator.CheckPasswordIsValid(password);
        Console.WriteLine($"Password passed {passed}");
    }
}

class PasswordValidator()
{
    // store password
    private int _lower_char_count;
    private int _upper_char_count;
    private int _number_count;


    public bool CheckPasswordIsValid(string password)
    {
        _reset_counts();
        // 6 letters no less than 13 letters
        if (password.Length < 6 && password.Length > 13)
        {
            return false;
        }

        // password must contain 1 upper, 1 lower, 1 number
        // can not contain T or &
        foreach (char letter in password)
        {
            if (char.IsLower(letter))
            {
                _lower_char_count += 1;
            }

            if (char.IsNumber(letter))
            {
                _number_count += 1;
            }

            if (char.IsUpper(letter))
            {
                _upper_char_count += 1;
            }

            if (letter == 'T')
            {
                return false;
            }

            if (letter == '&')
            {
                return false;
            }
        }

        if (_number_count >= 1 && _lower_char_count >= 1 && _upper_char_count >= 1)
        {
            return true;
        }

        return false;
    }

    public string GetPassword()
    {
        Console.WriteLine("Enter Valid Password");
        string password = Console.ReadLine();
        if (password == null)
        {
            password = "";
        }

        return password;
    }

    // reset all the counters
    private void _reset_counts()
    {
        _number_count = 0;
        _upper_char_count = 0;
        _lower_char_count = 0;
    }
}