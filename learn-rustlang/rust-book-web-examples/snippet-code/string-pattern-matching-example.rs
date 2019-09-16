pub fn reply(message: &str) -> &str {
    // Check for yelling
    let is_yelling = message.contains(char::is_alphabetic) && message == message.to_uppercase();
    // Pattern match mesage
    match message.trim() {
        // Empty message
        m if m.is_empty() => "Fine. Be that way!",
        // Question and yelling
        m if m.ends_with("?") && is_yelling => "Calm down, I know what I'm doing!",
        // Question
        m if m.ends_with("?") => "Sure.",
        // Just yelling
        m if is_yelling => "Whoa, chill out!",
        // Anything else
        _ => "Whatever."
    }
}