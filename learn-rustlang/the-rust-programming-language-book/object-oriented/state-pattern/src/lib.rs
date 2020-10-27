/// Post struct
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
/// Post methods
impl Post {
    /// new method creates the object
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    /// add_text to the post
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    /// return content from the post
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // request_review
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    // approve method
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

/// state trait
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
/// draft struct
struct Draft {}
/// methods for the trait
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
/// PendingReview struct
struct PendingReview {}
/// add methods form state trait
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
/// Published struct
struct Published {}
/// add methods from state trait
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
