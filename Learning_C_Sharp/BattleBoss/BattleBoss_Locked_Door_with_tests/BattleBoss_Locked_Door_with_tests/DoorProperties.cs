namespace BattleBoss_Locked_Door_with_tests;


    public enum DoorStatus
    {
        Open,
        Closed,
    }

    public class DoorProperties
    {
        // properties
        private DoorStatus _status = DoorStatus.Open;
        private ulong _passcode = 0;
        private bool _isLocked = false;
        public bool Exit = false;

        public DoorStatus Status
        {
            get => _status;
        }

        // constructor
        public DoorProperties()
        {
            do
            {
                UpdatePasscode();
                Console.WriteLine(_passcode.ToString());
            } while (_passcode == 0);

        }
        
        public DoorProperties(ulong passcode)
        {
            _passcode = passcode;

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
                    if (newResult == 0)
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

        public bool EnterPasscode()
        {
            return _enterPasscode(false);
        }

        public bool UpdatePasscode()
        {
            return _enterPasscode(true);
        }

        /// <summary>
        /// Prompts the user to enter a command and checks if that command is valid.
        /// </summary>
        /// <returns>True if the command is valid, false if not.</returns>
        public bool EnterCommand()
        {
            string command = GetCommand();
            return CheckCommand(command);
        }

        private string GetCommand()
        {
            Console.WriteLine(
                "Enter D for door state\n      O for open\n      C close\n      U for unlock\n      L for lock\n      UP for Update Passcode\n");
            return Console.ReadLine() ?? string.Empty;
        }

        protected internal bool CheckCommand(string input)
        {
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

        protected internal bool _open()
        {
            if (Status == DoorStatus.Closed && _isLocked) return false;
            _status = DoorStatus.Open;
            Console.WriteLine($"Door is now {_status.ToString()})");
            return true;
        }

        protected internal bool _close()
        {
            if (Status == DoorStatus.Open)
            {
                _status = DoorStatus.Closed;
                Console.WriteLine($"Door is now {_status.ToString()})");
                return true;
            }

            return false;
        }

        protected internal bool _unlock()
        {
            if (!_isLocked) return false;
            bool unlocked = EnterPasscode();
            return unlocked;

        }

        protected internal bool _lock()
        {
            if (Status != DoorStatus.Closed) return false;
            _isLocked = true;
            Console.WriteLine($"Door locked {_isLocked.ToString()})");
            return true;

        }

        protected internal bool _updatePasscode()
        {
            if (EnterPasscode())
            {

            }

            return false;
        }

        public bool IsLocked()
        {
            return _isLocked;
        }
    }
    