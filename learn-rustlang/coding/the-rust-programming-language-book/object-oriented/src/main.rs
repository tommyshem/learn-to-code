/// AverageCollection structure with functions attached
pub struct AverageCollection {
    // public struct but fields are private
    list: Vec<isize>, // private list of i32 numbers
    average: f64,     // private average f64 number
}
/// attach functions to the AverageCollection struct
impl AverageCollection {
    /// create new object average collection
    pub fn new() -> AverageCollection {
        AverageCollection {
            list: vec![],
            average: 0.0,
        }
    }

    /// add number to the end of the list and calculate the average field
    pub fn add(&mut self, value: isize) {
        self.list.push(value); // put value on the end of the list
        self.update_average(); // calculate average field
    }

    /// remove last item from the list and recalculate the average field
    pub fn remove(&mut self) -> Option<isize> {
        let result = self.list.pop(); // remove last number of the list
        match result {
            // check if a number was removed and calculate average field
            Some(value) => {
                self.update_average();
                Some(value) // return Option i32 value
            }
            None => None, // if no number removed do nothing and return Option None
        }
    }
    /// return the average field
    pub fn average(&self) -> f64 {
        self.average // return the average field value
    }
    /// calculate the average of all the numbers in the list
    fn update_average(&mut self) {
        let total: isize = self.list.iter().sum(); // add all the numbers in the list
        self.average = total as f64 / self.list.len() as f64; // calculate average - total value of numbers divide by how many numbers are in the list
    }
}

/// main run function
fn main() {
    let mut list: AverageCollection = AverageCollection::new(); // create the object
    list.add(5); // use the add method on the object
    list.add(28);  // use the add method on the object
    println!("List: 5 , 28");
    println!("Average {}", list.average());  // print the auto calculated average
    list.remove(); // use the remove method on the object
    println!("Removed last number from the list");
    println!("List: 5");
    println!("Average {}",list.average());  // print the auto calculated average
}
