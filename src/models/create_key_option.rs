/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateKeyOption : CreateKeyOption options when creating a key

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKeyOption {
  /// An armored SSH key to add
  #[serde(rename = "key")]
  key: String,
  /// Describe if the key has only read access or read/write
  #[serde(rename = "read_only")]
  read_only: Option<bool>,
  /// Title of the key to add
  #[serde(rename = "title")]
  title: String
}

impl CreateKeyOption {
  /// CreateKeyOption options when creating a key
  pub fn new(key: String, title: String) -> CreateKeyOption {
    CreateKeyOption {
      key: key,
      read_only: None,
      title: title
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> CreateKeyOption {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> CreateKeyOption {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = title;
  }

  pub fn with_title(mut self, title: String) -> CreateKeyOption {
    self.title = title;
    self
  }

  pub fn title(&self) -> &String {
    &self.title
  }


}



