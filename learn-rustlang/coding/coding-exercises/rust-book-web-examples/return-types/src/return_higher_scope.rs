use crate::Structure;
// Pattern 3: Move Owned Values to a Higher Scope
// Pros:
//
//     Memory - this pattern avoids heap allocating and writing boilerplate code, so it’s the memory efficient and elegant
//     Aroma - this pattern is often the natural result of good code organization that reduces unnecessary work
//             its like a good code “aroma”
//
// Cons:
//
//     Complexity - this pattern requires deep understanding of the application and data flows,
//                  and usually means rewriting several different areas of the code
//     Applicability - many times this pattern can’t be used
//     Rigidness - using this pattern might make refactoring the code more difficult
pub fn connect_higher_scope(conn: &mut Structure) -> &Structure {
    conn.id = 1;
    conn.url = String::from("Change name here");
    conn
}
