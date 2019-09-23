mod closure;
mod non_closure;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // non closure function
    non_closure::generate_workout(simulated_user_specified_value, simulated_random_number);
    // closure function
    closure::generate_workout_closure(simulated_user_specified_value, simulated_random_number);
}
