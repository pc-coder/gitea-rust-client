/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AnnotatedTag : AnnotatedTag represents an annotated tag

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotatedTag {
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "object")]
  object: Option<::models::AnnotatedTagObject>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "tag")]
  tag: Option<String>,
  #[serde(rename = "tagger")]
  tagger: Option<::models::CommitUser>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "verification")]
  verification: Option<::models::PayloadCommitVerification>
}

impl AnnotatedTag {
  /// AnnotatedTag represents an annotated tag
  pub fn new() -> AnnotatedTag {
    AnnotatedTag {
      message: None,
      object: None,
      sha: None,
      tag: None,
      tagger: None,
      url: None,
      verification: None
    }
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> AnnotatedTag {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_object(&mut self, object: ::models::AnnotatedTagObject) {
    self.object = Some(object);
  }

  pub fn with_object(mut self, object: ::models::AnnotatedTagObject) -> AnnotatedTag {
    self.object = Some(object);
    self
  }

  pub fn object(&self) -> Option<&::models::AnnotatedTagObject> {
    self.object.as_ref()
  }

  pub fn reset_object(&mut self) {
    self.object = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> AnnotatedTag {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_tag(&mut self, tag: String) {
    self.tag = Some(tag);
  }

  pub fn with_tag(mut self, tag: String) -> AnnotatedTag {
    self.tag = Some(tag);
    self
  }

  pub fn tag(&self) -> Option<&String> {
    self.tag.as_ref()
  }

  pub fn reset_tag(&mut self) {
    self.tag = None;
  }

  pub fn set_tagger(&mut self, tagger: ::models::CommitUser) {
    self.tagger = Some(tagger);
  }

  pub fn with_tagger(mut self, tagger: ::models::CommitUser) -> AnnotatedTag {
    self.tagger = Some(tagger);
    self
  }

  pub fn tagger(&self) -> Option<&::models::CommitUser> {
    self.tagger.as_ref()
  }

  pub fn reset_tagger(&mut self) {
    self.tagger = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> AnnotatedTag {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_verification(&mut self, verification: ::models::PayloadCommitVerification) {
    self.verification = Some(verification);
  }

  pub fn with_verification(mut self, verification: ::models::PayloadCommitVerification) -> AnnotatedTag {
    self.verification = Some(verification);
    self
  }

  pub fn verification(&self) -> Option<&::models::PayloadCommitVerification> {
    self.verification.as_ref()
  }

  pub fn reset_verification(&mut self) {
    self.verification = None;
  }

}



