/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FileLinksResponse : FileLinksResponse contains the links for a repo's file

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileLinksResponse {
  #[serde(rename = "git")]
  git: Option<String>,
  #[serde(rename = "html")]
  html: Option<String>,
  #[serde(rename = "self")]
  _self: Option<String>
}

impl FileLinksResponse {
  /// FileLinksResponse contains the links for a repo's file
  pub fn new() -> FileLinksResponse {
    FileLinksResponse {
      git: None,
      html: None,
      _self: None
    }
  }

  pub fn set_git(&mut self, git: String) {
    self.git = Some(git);
  }

  pub fn with_git(mut self, git: String) -> FileLinksResponse {
    self.git = Some(git);
    self
  }

  pub fn git(&self) -> Option<&String> {
    self.git.as_ref()
  }

  pub fn reset_git(&mut self) {
    self.git = None;
  }

  pub fn set_html(&mut self, html: String) {
    self.html = Some(html);
  }

  pub fn with_html(mut self, html: String) -> FileLinksResponse {
    self.html = Some(html);
    self
  }

  pub fn html(&self) -> Option<&String> {
    self.html.as_ref()
  }

  pub fn reset_html(&mut self) {
    self.html = None;
  }

  pub fn set__self(&mut self, _self: String) {
    self._self = Some(_self);
  }

  pub fn with__self(mut self, _self: String) -> FileLinksResponse {
    self._self = Some(_self);
    self
  }

  pub fn _self(&self) -> Option<&String> {
    self._self.as_ref()
  }

  pub fn reset__self(&mut self) {
    self._self = None;
  }

}



