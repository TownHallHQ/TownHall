use async_graphql::{Request, Variables};
use serde_json::json;

use crate::TestUtil;

#[tokio::test]
async fn creates_a_new_user() {
    let test_util = TestUtil::new_cleared().await;
    let (_, schema) = test_util.parts();
    let mutation: &str = "
        mutation RegisterUser($payload: UserRegisterInput!) {
            userRegister(input: $payload) {
                user {
                    id
                    name
                    surname
                    username
                    email
                    createdAt
                    updatedAt
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
                "payload": {
                  "name": "John",
                  "surname": "Appleseed",
                  "username": "john_appleseed",
                  "password": "Root$1234",
                  "email": "john_appleseed@whizzes.io",
            }
              }))),
        )
        .await;
    let data = result.data.into_json().unwrap();
    let user_register_user = &data["userRegister"]["user"];

    assert_eq!(user_register_user["name"], "John");
    assert_eq!(user_register_user["surname"], "Appleseed");
    assert_eq!(user_register_user["email"], "john_appleseed@whizzes.io");
    assert_eq!(user_register_user["username"], "john_appleseed");
    assert!(user_register_user["createdAt"].is_string());
    assert!(user_register_user["updatedAt"].is_string());
}
