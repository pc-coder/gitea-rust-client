/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Label : Label a label to an issue or a pr

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
  #[serde(rename = "color")]
  color: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl Label {
  /// Label a label to an issue or a pr
  pub fn new() -> Label {
    Label {
      color: None,
      description: None,
      id: None,
      name: None,
      url: None
    }
  }

  pub fn set_color(&mut self, color: String) {
    self.color = Some(color);
  }

  pub fn with_color(mut self, color: String) -> Label {
    self.color = Some(color);
    self
  }

  pub fn color(&self) -> Option<&String> {
    self.color.as_ref()
  }

  pub fn reset_color(&mut self) {
    self.color = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> Label {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Label {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Label {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Label {
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


