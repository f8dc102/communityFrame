// libs/services/src/auth/register.rs
use libs::models::auth::register::{RegisterRequest, RegisterResponse};
use libs::repositories::user::add_user;

pub async fn register_user(email: String, password: Strring) -> Result<(), Error> {
    // Add user to the database
    add_user().await
}
