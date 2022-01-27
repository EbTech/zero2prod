mod get;
mod post;

pub use get::change_password_form;
pub use post::{change_password, PASSWORD_MAX_LEN, PASSWORD_MIN_LEN};
