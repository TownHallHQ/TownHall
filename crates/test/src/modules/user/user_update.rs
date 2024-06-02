use std::str::FromStr;

use async_graphql::{Request, Variables};
use serde_json::json;

use townhall::user::model::{Email, Password, Username};
use townhall::user::service::CreateUserDto;

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
        mutation UpdateUser($payload: UserUpdateInput!) {
            userUpdate(input: $payload) {
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

    let token = TestUtil::token_create(&test_util, user.id).await;
    let result = schema
        .execute(
            Request::new(mutation)
                .data(token)
                .variables(Variables::from_json(json!({
                        "payload": {
                            "name": "John",
                            "surname": "Appleseed",
                    }
                }))),
        )
        .await;

    let data = result.data.into_json().unwrap();
    let user_update_user = &data["userUpdate"]["user"];

    assert_eq!(user_update_user["name"], "John");
    assert_eq!(user_update_user["surname"], "Appleseed");
}
