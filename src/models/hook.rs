/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Hook : Hook a hook is a web hook when one repository changed

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
  #[serde(rename = "active")]
  active: Option<bool>,
  #[serde(rename = "config")]
  config: Option<::std::collections::HashMap<String, String>>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "events")]
  events: Option<Vec<String>>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "updated_at")]
  updated_at: Option<String>
}

impl Hook {
  /// Hook a hook is a web hook when one repository changed
  pub fn new() -> Hook {
    Hook {
      active: None,
      config: None,
      created_at: None,
      events: None,
      id: None,
      _type: None,
      updated_at: None
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> Hook {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_config(&mut self, config: ::std::collections::HashMap<String, String>) {
    self.config = Some(config);
  }

  pub fn with_config(mut self, config: ::std::collections::HashMap<String, String>) -> Hook {
    self.config = Some(config);
    self
  }

  pub fn config(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.config.as_ref()
  }

  pub fn reset_config(&mut self) {
    self.config = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> Hook {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_events(&mut self, events: Vec<String>) {
    self.events = Some(events);
  }

  pub fn with_events(mut self, events: Vec<String>) -> Hook {
    self.events = Some(events);
    self
  }

  pub fn events(&self) -> Option<&Vec<String>> {
    self.events.as_ref()
  }

  pub fn reset_events(&mut self) {
    self.events = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Hook {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> Hook {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> Hook {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

}



