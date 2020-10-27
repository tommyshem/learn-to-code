use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// struct using a closure and the pattern lazy evaluation
struct Cacher<T>
// generics used
where
    T: Fn(u32) -> u32, // T is bound to a closure using the Fn trait
{
    calculation: T,     // calculation must have same signature as above defined
    value: Option<u32>, // holding the value returned from the closure
}
// implement the new function to the structure above
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(args); // calls closure passed in
                self.value = Some(v); // store return value
                v // return value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // the expensive_result pass cached value after first run
    // pass in a closure to Cacher::new to use
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}
