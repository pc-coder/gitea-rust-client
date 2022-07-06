/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateFileOptions : CreateFileOptions options for creating files Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileOptions {
  #[serde(rename = "author")]
  author: Option<::models::Identity>,
  /// branch (optional) to base this file from. if not given, the default branch is used
  #[serde(rename = "branch")]
  branch: Option<String>,
  #[serde(rename = "committer")]
  committer: Option<::models::Identity>,
  /// content must be base64 encoded
  #[serde(rename = "content")]
  content: String,
  #[serde(rename = "dates")]
  dates: Option<::models::CommitDateOptions>,
  /// message (optional) for the commit of this file. if not supplied, a default message will be used
  #[serde(rename = "message")]
  message: Option<String>,
  /// new_branch (optional) will make a new branch from `branch` before creating the file
  #[serde(rename = "new_branch")]
  new_branch: Option<String>,
  /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
  #[serde(rename = "signoff")]
  signoff: Option<bool>
}

impl CreateFileOptions {
  /// CreateFileOptions options for creating files Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
  pub fn new(content: String) -> CreateFileOptions {
    CreateFileOptions {
      author: None,
      branch: None,
      committer: None,
      content: content,
      dates: None,
      message: None,
      new_branch: None,
      signoff: None
    }
  }

  pub fn set_author(&mut self, author: ::models::Identity) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: ::models::Identity) -> CreateFileOptions {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&::models::Identity> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_branch(&mut self, branch: String) {
    self.branch = Some(branch);
  }

  pub fn with_branch(mut self, branch: String) -> CreateFileOptions {
    self.branch = Some(branch);
    self
  }

  pub fn branch(&self) -> Option<&String> {
    self.branch.as_ref()
  }

  pub fn reset_branch(&mut self) {
    self.branch = None;
  }

  pub fn set_committer(&mut self, committer: ::models::Identity) {
    self.committer = Some(committer);
  }

  pub fn with_committer(mut self, committer: ::models::Identity) -> CreateFileOptions {
    self.committer = Some(committer);
    self
  }

  pub fn committer(&self) -> Option<&::models::Identity> {
    self.committer.as_ref()
  }

  pub fn reset_committer(&mut self) {
    self.committer = None;
  }

  pub fn set_content(&mut self, content: String) {
    self.content = content;
  }

  pub fn with_content(mut self, content: String) -> CreateFileOptions {
    self.content = content;
    self
  }

  pub fn content(&self) -> &String {
    &self.content
  }


  pub fn set_dates(&mut self, dates: ::models::CommitDateOptions) {
    self.dates = Some(dates);
  }

  pub fn with_dates(mut self, dates: ::models::CommitDateOptions) -> CreateFileOptions {
    self.dates = Some(dates);
    self
  }

  pub fn dates(&self) -> Option<&::models::CommitDateOptions> {
    self.dates.as_ref()
  }

  pub fn reset_dates(&mut self) {
    self.dates = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> CreateFileOptions {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_new_branch(&mut self, new_branch: String) {
    self.new_branch = Some(new_branch);
  }

  pub fn with_new_branch(mut self, new_branch: String) -> CreateFileOptions {
    self.new_branch = Some(new_branch);
    self
  }

  pub fn new_branch(&self) -> Option<&String> {
    self.new_branch.as_ref()
  }

  pub fn reset_new_branch(&mut self) {
    self.new_branch = None;
  }

  pub fn set_signoff(&mut self, signoff: bool) {
    self.signoff = Some(signoff);
  }

  pub fn with_signoff(mut self, signoff: bool) -> CreateFileOptions {
    self.signoff = Some(signoff);
    self
  }

  pub fn signoff(&self) -> Option<&bool> {
    self.signoff.as_ref()
  }

  pub fn reset_signoff(&mut self) {
    self.signoff = None;
  }

}



