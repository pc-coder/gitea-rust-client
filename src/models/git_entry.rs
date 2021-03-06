/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GitEntry : GitEntry represents a git tree

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitEntry {
  #[serde(rename = "mode")]
  mode: Option<String>,
  #[serde(rename = "path")]
  path: Option<String>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "size")]
  size: Option<i64>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl GitEntry {
  /// GitEntry represents a git tree
  pub fn new() -> GitEntry {
    GitEntry {
      mode: None,
      path: None,
      sha: None,
      size: None,
      _type: None,
      url: None
    }
  }

  pub fn set_mode(&mut self, mode: String) {
    self.mode = Some(mode);
  }

  pub fn with_mode(mut self, mode: String) -> GitEntry {
    self.mode = Some(mode);
    self
  }

  pub fn mode(&self) -> Option<&String> {
    self.mode.as_ref()
  }

  pub fn reset_mode(&mut self) {
    self.mode = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> GitEntry {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> GitEntry {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_size(&mut self, size: i64) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i64) -> GitEntry {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i64> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> GitEntry {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> GitEntry {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



