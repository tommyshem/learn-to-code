using System.ComponentModel.Design.Serialization;

namespace BattleBoss_Locked_door;

class Program
{
    static void Main(string[] args)
    {
        var door = new Door();
        // get commands
        do
        {
            door.EnterCommand();      
        } while (!door.Exit);
        

    }
}


public enum DoorStatus
{
 Open,
 Closed,
}

public class Door
{
    // properties
    private DoorStatus _status;
    private ulong _passcode;
    private bool _isLocked = false;
    public bool Exit = false;
    public ulong Passcode
    {
        get => _passcode;
        
    }
    public DoorStatus Status
    {
        get => _status;
    }

    // constructor
    public Door()
    {
        do
        {
            UpdatePasscode();
            Console.WriteLine(_passcode.ToString());
        } while (_passcode==0);
        
    }

    // false = entered wrong password
    // true = entered correct password
    private bool _enterPasscode(bool updatePasscode)
    {
        ulong result = 0;
        if (_passcode != 0)
        {
            Console.WriteLine("Enter passcode");
            var input = Console.ReadLine();
            // check input is a number
            result = Convert.ToUInt64(input);
            Console.WriteLine($"Result = {result}");
            if (result == 0)
            {
                return false;
            }
        }

        // check passcode - return true is passcode is the same
        if (result == _passcode)
        {
           if (updatePasscode)
           {
               Console.WriteLine("Enter passcode");
               var newInput = Console.ReadLine();
               // check input is a number
               var newResult = Convert.ToUInt64(newInput);
               if (newResult ==0)
               {
                   Console.WriteLine("Passcode not changed.");
                   return false;
               }

               _passcode = newResult;
               Console.WriteLine("Passcode changed.");
           }
           return true;
       }
       // passcode does not match
       return false;
    }

    bool EnterPasscode()
    {
        return _enterPasscode(false);
    }

    bool UpdatePasscode()
    {
        return _enterPasscode(true);
    }

    public bool EnterCommand()
    {
        string command =GetCommand();
        return CheckCommand(command);
    }
    
    private string GetCommand()
    {
        Console.WriteLine("Enter D for door state\n      O for open\n      C close\n      U for unlock\n      L for lock\n      UP for Update Passcode\n");
        return  Console.ReadLine() ?? string.Empty;
    }

    private bool CheckCommand(string input){
        // check command
        switch (input)
        {
            // quit
            case "q":
                case "Q":
                    Exit = true;  
                    return true;
            // door status
            case "d":
                case "D":
                    Console.WriteLine($"Door status is {_status.ToString()}");
                    return true;
            // open
            case "o":
            case "O":
                return _open();
            // close
            case "c":
            case "C":
                return _close();
            // unlock
            case "u":
            case "U":
                return _unlock();
           // lock
            case "l":
            case "L":
                return _lock();
            case "up":
            case "UP":
                return _updatePasscode();
            default:
                Console.WriteLine("Wrong command entered. Try again!");
                return false;
        }
    }

    private bool _open()
    {
        if (Status == DoorStatus.Closed && _isLocked) return false;
        _status = DoorStatus.Open;
        Console.WriteLine($"Door is now {_status.ToString()})");
        return true;
    }
    private bool _close()
    {
        if (Status == DoorStatus.Open)
        {
            _status = DoorStatus.Closed;
            Console.WriteLine($"Door is now {_status.ToString()})");
            return true;
        }

        return false;
    }
    private bool _unlock()
    {
        if (!_isLocked) return false;
        bool unlocked = EnterPasscode();
        return unlocked;

    }
    private bool _lock()
    {
        if (Status != DoorStatus.Closed) return false;
        _isLocked=true;
        Console.WriteLine($"Door locked {_isLocked.ToString()})");
        return true;

    }

    private bool _updatePasscode()
    {
        if (EnterPasscode())
        {
            
        }
        return false;
    }
    
}

