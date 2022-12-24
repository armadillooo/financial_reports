use serde::{Serialize, Deserialize};

use applications::user::UserData;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UserResponse {
    user_id: String, 
    name: String,
    email: String,
}

impl From<UserData> for UserResponse {
    fn from(value: UserData) -> Self {
        Self {
            user_id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}