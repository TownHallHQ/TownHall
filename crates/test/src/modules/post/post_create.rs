use std::str::FromStr;

use async_graphql::{Request, Variables};
use gabble::user::{
    model::{Email, Password, Username},
    service::CreateUserDto,
};
use serde_json::json;

use libserver::graphql::guard::{GENERIC_FORBIDDEN_ERROR, GENERIC_FORBIDDEN_ERROR_CODE};

use crate::TestUtil;

const CREATE_POST_MUTATION: &str = r#"
mutation ($payload: PostCreateInput!) {
  postCreate(input: $payload) {
    post {
      id
      authorId
      parentId
      head
      title
      content
      author {
        id
        name
        surname
        username
        email
      }
      createdAt
      updatedAt
    }
    error {
      code
      message
    }
  }
}
"#;

#[tokio::test]
async fn create_post() {
    let test_util = TestUtil::new_cleared().await;
    let (context, schema) = test_util.parts();
    let username = Username::from_str("john_appleseed").unwrap();
    let email = Email::from_str("john_appleseed@whizzes.io").unwrap();
    let password = Password::from_str("Root$1234").unwrap();
    let user = context
        .services
        .user
        .create(CreateUserDto {
            name: "John".to_string(),
            surname: "Appleseed".to_string(),
            username,
            password,
            email,
        })
        .await
        .unwrap();
    let token = TestUtil::token_create(&test_util, user.id).await;
    let result =
        schema
            .execute(Request::new(CREATE_POST_MUTATION).data(token).variables(
                Variables::from_json(json!({
                  "payload":{
                    "title":"Hello World!",
                    "content":"Hello, new post!"
                  }
                })),
            ))
            .await;
    let data = result.data.into_json().unwrap();
    let post_data = &data["postCreate"]["post"];

    assert_eq!(post_data["title"], "Hello World!");
    assert_eq!(post_data["content"], "Hello, new post!");
    assert_eq!(post_data["authorId"], user.id.to_string());
    assert_eq!(post_data["author"]["id"], user.id.to_string());
    assert_eq!(post_data["author"]["name"], user.name);
    assert_eq!(post_data["author"]["surname"], user.surname);
    assert_eq!(post_data["author"]["username"], user.username.to_string());
    assert_eq!(post_data["author"]["email"], user.email.to_string());
    assert_eq!(post_data["head"], true);
    assert!(post_data["parentId"].is_null());
    assert!(post_data["createdAt"].is_string());
    assert!(post_data["updatedAt"].is_string())
}

#[tokio::test]
async fn creates_post_with_parent() {
    let test_util = TestUtil::new_cleared().await;
    let (context, schema) = test_util.parts();
    let username = Username::from_str("john_appleseed").unwrap();
    let email = Email::from_str("john_appleseed@whizzes.io").unwrap();
    let password = Password::from_str("Root$1234").unwrap();
    let user = context
        .services
        .user
        .create(CreateUserDto {
            name: "John".to_string(),
            surname: "Appleseed".to_string(),
            username,
            password,
            email,
        })
        .await
        .unwrap();
    let token = TestUtil::token_create(&test_util, user.id).await;
    let result_parent =
        schema
            .execute(Request::new(CREATE_POST_MUTATION).data(token).variables(
                Variables::from_json(json!({
                  "payload":{
                    "title":"Hello World!",
                    "content":"Hello, new post!",
                  }
                })),
            ))
            .await;

    let post_parent_data = result_parent.data.into_json().unwrap();
    let post_parent_id = &post_parent_data["postCreate"]["post"]["id"];
    let token = TestUtil::token_create(&test_util, user.id).await;
    let result =
        schema
            .execute(Request::new(CREATE_POST_MUTATION).data(token).variables(
                Variables::from_json(json!({
                  "payload":{
                    "title":"Hello again!",
                    "content":"Hello, new post again!",
                    "parentId":post_parent_id
                  }
                })),
            ))
            .await;

    let data = result.data.into_json().unwrap();
    let post_data = &data["postCreate"]["post"];

    assert_eq!(post_data["title"].as_str().unwrap(), "Hello again!");
    assert_eq!(
        post_data["content"].as_str().unwrap(),
        "Hello, new post again!"
    );
    assert_eq!(post_data["authorId"], user.id.to_string());
    assert_eq!(
        post_data["head"], false,
        "should not be head given that is a comment"
    );
    assert_eq!(&post_data["parentId"], post_parent_id);
    assert!(post_data["createdAt"].is_string());
    assert!(post_data["updatedAt"].is_string())
}

#[tokio::test]
async fn create_post_without_a_token() {
    let test_util = TestUtil::new_cleared().await;
    let (_, schema) = test_util.parts();
    let result = schema
        .execute(
            Request::new(CREATE_POST_MUTATION).variables(Variables::from_json(json!({
              "payload":{
                "title":"Hello World!",
                "content":"Hello, new post!"
              }
            }))),
        )
        .await;
    let errors = result.errors;
    let first_error = errors.first().unwrap();
    let error_code = first_error.extensions.clone().unwrap();
    let error_code = error_code.get("code").unwrap().to_owned();

    assert_eq!(
        first_error.message, GENERIC_FORBIDDEN_ERROR,
        "unexpected error message received"
    );
    assert_eq!(
        error_code.into_json().unwrap(),
        GENERIC_FORBIDDEN_ERROR_CODE,
        "unexpected error code received"
    );
}
