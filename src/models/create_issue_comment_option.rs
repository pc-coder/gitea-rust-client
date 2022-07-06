/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateIssueCommentOption : CreateIssueCommentOption options for creating a comment on an issue

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueCommentOption {
  #[serde(rename = "body")]
  body: String
}

impl CreateIssueCommentOption {
  /// CreateIssueCommentOption options for creating a comment on an issue
  pub fn new(body: String) -> CreateIssueCommentOption {
    CreateIssueCommentOption {
      body: body
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = body;
  }

  pub fn with_body(mut self, body: String) -> CreateIssueCommentOption {
    self.body = body;
    self
  }

  pub fn body(&self) -> &String {
    &self.body
  }


}



