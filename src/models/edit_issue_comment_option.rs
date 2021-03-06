/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditIssueCommentOption : EditIssueCommentOption options for editing a comment

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditIssueCommentOption {
  #[serde(rename = "body")]
  body: String
}

impl EditIssueCommentOption {
  /// EditIssueCommentOption options for editing a comment
  pub fn new(body: String) -> EditIssueCommentOption {
    EditIssueCommentOption {
      body: body
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = body;
  }

  pub fn with_body(mut self, body: String) -> EditIssueCommentOption {
    self.body = body;
    self
  }

  pub fn body(&self) -> &String {
    &self.body
  }


}



