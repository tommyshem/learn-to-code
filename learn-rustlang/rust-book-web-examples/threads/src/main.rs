mod basic_thread;
mod channel_multi_values;
mod thread_channels;
mod thread_pass_data;

fn main() {
    // Threads
    println!("Basic Thread");
    basic_thread::basic_thread();
    println!("\n");
    println!("Passing data to a thread");
    thread_pass_data::thread_pass_data();
    println!("\n");
    // Channels with Treads
    println!("Channel pass multi values");
    channel_multi_values::channel_multi_values();
    println!("\n");
    println!("Thread with a channel");
    thread_channels::thread_channel();
    println!("\n");
}
