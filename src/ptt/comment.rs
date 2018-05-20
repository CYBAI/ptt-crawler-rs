use ptt::user::User;

#[derive(Clone, Debug)]
pub enum CommentType {
  Normal,
  Good,
  Bad
}

#[derive(Clone, Debug)]
pub struct Comment {
  type_: CommentType,
  user: User,
  content: String,
  time: String,
}

impl Comment {
  pub fn new(type_: CommentType, user: User, content: String, time: String) -> Self {
    Comment {
      type_,
      user,
      content,
      time,
    }
  }
}
