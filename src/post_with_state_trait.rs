use crate::Post;

pub struct PostWithStateTrait {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Default for PostWithStateTrait {
    fn default() -> Self {
        Self {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
}

macro_rules! state_transition_fn {
    ($state:ident) => {
        fn $state(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.$state());
            }
        }
    };

    ($state:ident, $($rest:ident),+) => {
        state_transition_fn!($state);
        state_transition_fn!($($rest),+);
    };
}

impl Post for PostWithStateTrait {
    fn add_text(&mut self, text: &str) {
        if let Some(state) = self.state.take() {
            state.add_text(self, text);
            self.state = Some(state);
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    state_transition_fn!(request_review, approve, reject);
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a PostWithStateTrait) -> &'a str {
        ""
    }
    fn add_text(&self, _post: &mut PostWithStateTrait, _text: &str) {}
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::<PendingReview>::default()
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, post: &mut PostWithStateTrait, text: &str) {
        post.content.push_str(text);
    }
}

#[derive(Default)]
struct PendingReview {
    approvals: u32,
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals += 1;
        if self.approvals >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a PostWithStateTrait) -> &'a str {
        &post.content
    }
}
