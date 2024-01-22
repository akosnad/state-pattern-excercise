use crate::Post;

pub struct PostWithEnumState {
    content: String,
    state: State,
}

impl Default for PostWithEnumState {
    fn default() -> Self {
        Self {
            content: String::new(),
            state: State::Draft,
        }
    }
}

impl Post for PostWithEnumState {
    fn add_text(&mut self, text: &str) {
        if let State::Draft = self.state {
            self.content.push_str(text);
        }
    }

    fn content(&self) -> &str {
        match self.state {
            State::Published => &self.content,
            _ => "",
        }
    }

    fn request_review(&mut self) {
        if let State::Draft = self.state {
            self.state = State::PendingReview(ReviewCount::default())
        }
    }

    fn approve(&mut self) {
        if let State::PendingReview(mut review_count) = self.state {
            if review_count.is_enough_to_publish() {
                self.state = State::Published
            } else {
                review_count.increment();
                self.state = State::PendingReview(review_count)
            }
        }
    }

    fn reject(&mut self) {
        if let State::PendingReview(_) = self.state {
            self.state = State::Draft
        }
    }
}

enum State {
    Draft,
    PendingReview(ReviewCount),
    Published,
}

#[derive(Default, Clone, Copy)]
struct ReviewCount {
    count: u32,
}

impl ReviewCount {
    fn increment(&mut self) {
        self.count += 1;
    }

    fn is_enough_to_publish(&self) -> bool {
        self.count >= 1
    }
}
