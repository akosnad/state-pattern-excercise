mod post_with_enum_state;
mod post_with_state_trait;

pub use post_with_enum_state::PostWithEnumState;
pub use post_with_state_trait::PostWithStateTrait;

pub trait Post {
    fn add_text(&mut self, text: &str);
    fn content(&self) -> &str;
    fn request_review(&mut self);
    fn approve(&mut self);
    fn reject(&mut self);
}
