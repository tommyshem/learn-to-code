namespace BattleBoss_Locked_Door_with_tests;


public class Tests
{
    private DoorProperties door;
    [SetUp]
    public void Setup(){
    door = new DoorProperties(84567);
    
    }

    [Test]
    public void Test_Closed_To_Open_Door()
    {
        door._close();
        Assert.That( door.Status == DoorStatus.Closed);
        door._open();
        Assert.That(door.Status==DoorStatus.Open);
    }
[Test]
    public void Test_Open_To_Close_Door()
    {
        Assert.That(door.Status, Is.EqualTo(DoorStatus.Open));
        door._close();
        Assert.That(door.Status, Is.EqualTo(DoorStatus.Closed));
    }

    [Test]
    public void Test_Unlock_To_Locked_Door()
    {
        door._close(); // close the door
        Assert.Multiple(() =>
        {
            Assert.That(door.Status, Is.EqualTo(DoorStatus.Closed));
            Assert.That(door.IsLocked(), Is.EqualTo(false));
        });
        door._lock(); // lock the door
        Assert.That(door.IsLocked());
    }
}

