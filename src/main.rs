use state_pattern_excercise::{
    Post, PostWithEnumState, PostWithOwnershipState, PostWithStateTrait,
};

fn main() {
    let mut post = PostWithEnumState::default();
    test_post_impl(&mut post);

    let mut post = PostWithStateTrait::default();
    test_post_impl(&mut post);

    let mut post = PostWithOwnershipState::new();
    post.add_text("Hello world!");
    let post = post.request_review();
    let post = post.reject();
    let mut post = post.request_review();
    post.approve();
    let result = post.publish();
    assert!(result.is_err());
    let mut post = result.err().unwrap().into_inner();
    post.approve();
    let result = post.publish();
    assert!(result.is_ok());
    let post = result.unwrap();
    assert_eq!("Hello world!", post.content());
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
