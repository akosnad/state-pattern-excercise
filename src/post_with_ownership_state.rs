pub struct PostWithOwnershipState {
    content: String,
}
impl PostWithOwnershipState {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
    approvals: u32,
}
impl PendingReviewPost {
    pub fn approve(&mut self) {
        self.approvals += 1;
    }

    pub fn publish(self) -> Result<PostWithOwnershipState, NotEnoughApprovalsError> {
        const MIN_APPROVALS: u32 = 2;

        if self.approvals < MIN_APPROVALS {
            return Err(NotEnoughApprovalsError(self));
        }

        Ok(PostWithOwnershipState {
            content: self.content,
        })
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[derive(Debug)]
pub struct NotEnoughApprovalsError(PendingReviewPost);
impl NotEnoughApprovalsError {
    pub fn into_inner(self) -> PendingReviewPost {
        self.0
    }
}
