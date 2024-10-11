use aws_sdk_dynamodb::Client;
use lambda_http::Error;
use models::user::AddUser;
use serde_dynamo::to_attribute_value;

async fn add_user(client: &Client, user: AddUser) -> Result<(), Error> {
    let uuid = to_attribute_value(user.uuid)?;
    let email = to_attribute_value(user.email)?;
    let password = to_attribute_value(user.password)?;

    let request = client
        .put_item()
        // @TODO: Use the table name from the environment
        .table_name("users")
        .item("uuid", uuid)
        .item("email", email)
        .item("password", password);

    let _resp = request.send().await?;

    Ok(())
}
