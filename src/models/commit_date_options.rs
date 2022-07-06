/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CommitDateOptions : CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDateOptions {
  #[serde(rename = "author")]
  author: Option<String>,
  #[serde(rename = "committer")]
  committer: Option<String>
}

impl CommitDateOptions {
  /// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
  pub fn new() -> CommitDateOptions {
    CommitDateOptions {
      author: None,
      committer: None
    }
  }

  pub fn set_author(&mut self, author: String) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: String) -> CommitDateOptions {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&String> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_committer(&mut self, committer: String) {
    self.committer = Some(committer);
  }

  pub fn with_committer(mut self, committer: String) -> CommitDateOptions {
    self.committer = Some(committer);
    self
  }

  pub fn committer(&self) -> Option<&String> {
    self.committer.as_ref()
  }

  pub fn reset_committer(&mut self) {
    self.committer = None;
  }

}



