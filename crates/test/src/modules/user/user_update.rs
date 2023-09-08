use std::str::FromStr;

use async_graphql::{Request, Variables};
use playa::user::{
    model::{Email, Password, Username},
    service::CreateUserDto,
};
use serde_json::json;

use crate::TestUtil;

#[tokio::test]
async fn updates_a_user() {
    let test_util = TestUtil::new_cleared().await;
    let (context, schema) = test_util.parts();

    let user = context
        .services
        .user
        .create(CreateUserDto {
            name: "Augustine".into(),
            surname: "Madu".into(),
            email: Email::from_str("augustine@gmail.com").unwrap(),
            username: Username::from_str("Cudi").unwrap(),
            password: Password::from_str("Password12##$$").unwrap(),
        })
        .await
        .unwrap();

    let mutation: &str = "
        mutation UpdateUser($id: Pxid, $payload: UserUpdateInput!) {
            userUpdate(id:$id, input: $payload) {
                user {
                    id
                    name
                    surname
                }
                error {
                    code
                    message
                }
            }
        }
    ";

    let result = schema
        .execute(
            Request::new(mutation).variables(Variables::from_json(json!({
                    "id": user.id.to_string(),
                    "payload": {
                        "name": "John",
                        "surname": "Appleseed",
                }
            }))),
        )
        .await;

    println!("{result:#?}");

    let data = result.data.into_json().unwrap();
    let user_update_user = &data["userUpdate"]["user"];

    assert_eq!(user_update_user["name"], "John");
    assert_eq!(user_update_user["surname"], "Appleseed");
}
