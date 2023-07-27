use std::str::FromStr;

use async_graphql::{Request, Variables};
use pxid::Pxid;
use serde_json::json;

use libserver::graphql::guard::{GENERIC_FORBIDDEN_ERROR, GENERIC_FORBIDDEN_ERROR_CODE};

use crate::TestUtil;

#[tokio::test]
async fn create_post() {
    let test_util = TestUtil::new_cleared().await;
    let (_, schema) = test_util.parts();
    let user_create_mutation: &str = "
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

    let result_user = schema
        .execute(
            Request::new(user_create_mutation).variables(Variables::from_json(json!({
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

    let user_data = result_user.data.into_json().unwrap();
    let user_uid = user_data["userRegister"]["user"]["id"].as_str().unwrap();
    let token = TestUtil::token_create(&test_util, Pxid::from_str(user_uid).unwrap()).await;
    let mutation = "
      mutation ($payload: PostCreateInput!) {
        postCreate(input: $payload) {
          post {
            id
            authorId
            parentId
            head
            title
            content
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
            Request::new(mutation)
                .data(token)
                .variables(Variables::from_json(json!({
                  "payload":{
                    "title":"Hello World!",
                    "content":"Hello, new post!"
                  }
                }))),
        )
        .await;

    let data = result.data.into_json().unwrap();
    let post_data = &data["postCreate"]["post"];

    assert_eq!(post_data["title"], "Hello World!");
    assert_eq!(post_data["content"], "Hello, new post!");
    assert_eq!(post_data["authorId"], user_uid);
    assert_eq!(post_data["head"], true);
    assert!(post_data["parentId"].is_null());
    assert!(post_data["createdAt"].is_string());
    assert!(post_data["updatedAt"].is_string())
}

#[tokio::test]
async fn creates_post_with_parent() {
    let test_util = TestUtil::new_cleared().await;
    let (_, schema) = test_util.parts();

    let user_create_mutation: &str = "
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

    let result_user = schema
        .execute(
            Request::new(user_create_mutation).variables(Variables::from_json(json!({
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

    let user_data = result_user.data.into_json().unwrap();
    let user_uid = user_data["userRegister"]["user"]["id"].as_str().unwrap();

    let token = TestUtil::token_create(&test_util, Pxid::from_str(user_uid).unwrap()).await;

    let mutation = "
      mutation ($payload: PostCreateInput!) {
        postCreate(input: $payload) {
          post {
            id
            authorId
            parentId
            head
            title
            content
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

    let result_parent = schema
        .execute(
            Request::new(mutation)
                .data(token)
                .variables(Variables::from_json(json!({
                  "payload":{
                    "title":"Hello World!",
                    "content":"Hello, new post!",
                  }
                }))),
        )
        .await;

    let post_parent_data = result_parent.data.into_json().unwrap();
    let post_parent_id = &post_parent_data["postCreate"]["post"]["id"];
    let token = TestUtil::token_create(&test_util, Pxid::from_str(user_uid).unwrap()).await;
    let result = schema
        .execute(
            Request::new(mutation)
                .data(token)
                .variables(Variables::from_json(json!({
                  "payload":{
                    "title":"Hello again!",
                    "content":"Hello, new post again!",
                    "parentId":post_parent_id
                  }
                }))),
        )
        .await;

    let data = result.data.into_json().unwrap();
    let post_data = &data["postCreate"]["post"];

    assert_eq!(post_data["title"].as_str().unwrap(), "Hello again!");
    assert_eq!(
        post_data["content"].as_str().unwrap(),
        "Hello, new post again!"
    );
    assert_eq!(post_data["authorId"], user_uid);
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

    let mutation = "
      mutation ($payload: PostCreateInput!) {
        postCreate(input: $payload) {
          post {
            id
            authorId
            parentId
            head
            title
            content
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
