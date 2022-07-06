/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Note : Note contains information related to a git note

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
  #[serde(rename = "commit")]
  commit: Option<::models::Commit>,
  #[serde(rename = "message")]
  message: Option<String>
}

impl Note {
  /// Note contains information related to a git note
  pub fn new() -> Note {
    Note {
      commit: None,
      message: None
    }
  }

  pub fn set_commit(&mut self, commit: ::models::Commit) {
    self.commit = Some(commit);
  }

  pub fn with_commit(mut self, commit: ::models::Commit) -> Note {
    self.commit = Some(commit);
    self
  }

  pub fn commit(&self) -> Option<&::models::Commit> {
    self.commit.as_ref()
  }

  pub fn reset_commit(&mut self) {
    self.commit = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> Note {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

}



