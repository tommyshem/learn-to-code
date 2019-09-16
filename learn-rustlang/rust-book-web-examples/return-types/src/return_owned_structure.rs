use crate::Structure;
// Pattern 1: Return Owned Values (copied values)
// Pros:
//    Low Effort - generally, converting a return value from a shared reference to an owned value is really easy.
//                 I can usually get something to compile quickly and refactor later using this technique.
//    Widely Applicable - almost anywhere we can return a reference, we could also return an owned copy of that value
//    Safe - a new copy of the value cannot corrupt memory elsewhere, even when moving the data between threads
// Cons:
//
//    Synchronization - this value won’t be changed if we change the original value.
//    Memory - we’ll possibly be wasting memory by making identical copies of some piece of data (usually won’t happen)

#[allow(dead_code)]
pub fn connect_owned(url: &str) -> Structure {
    let connection: Structure = Structure {
        id: 1,
        url: String::from(url),
    };
    connection
}
