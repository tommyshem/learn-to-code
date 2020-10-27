use crate::Structure;
// Pattern 2: Return Boxed Values (heap)
// Pros:
//
//    Memory - The only additional memory (beyond the boxed value) is the pointer to the heap, and pointers are only a few bytes
//    Applicability - Almost any code which uses std can leverage this technique
// Cons:
//
//    Indirection - we’ll need to write more code in our type annotations,
//                  and we may need to understand how to leverage the Deref trait to work with boxed values
//    Overhead - It’s more complicated to allocate memory on the heap and this may incur a runtime penalty.

// Notes Vec, String, and HashMap, these types already use the heap internally.

#[allow(dead_code)]
pub fn connect_box(url: &str) -> Box<Structure> {
    let connection: Structure = Structure {
        id: 1,
        url: String::from(url),
    };
    Box::new(connection)
}