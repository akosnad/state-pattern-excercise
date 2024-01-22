use state_pattern_excercise::{Post, PostWithEnumState, PostWithStateTrait};

fn main() {
    let mut post = PostWithEnumState::default();
    test_post_impl(&mut post);

    let mut post = PostWithStateTrait::default();
    test_post_impl(&mut post);
}

fn test_post_impl(post: &mut dyn Post) {
    post.add_text("Hello world!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Hello world!", post.content());

    post.add_text("test");
    assert_eq!("Hello world!", post.content());
}
