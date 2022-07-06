/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditHookOption : EditHookOption options when modify one hook

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditHookOption {
  #[serde(rename = "active")]
  active: Option<bool>,
  #[serde(rename = "branch_filter")]
  branch_filter: Option<String>,
  #[serde(rename = "config")]
  config: Option<::std::collections::HashMap<String, String>>,
  #[serde(rename = "events")]
  events: Option<Vec<String>>
}

impl EditHookOption {
  /// EditHookOption options when modify one hook
  pub fn new() -> EditHookOption {
    EditHookOption {
      active: None,
      branch_filter: None,
      config: None,
      events: None
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> EditHookOption {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_branch_filter(&mut self, branch_filter: String) {
    self.branch_filter = Some(branch_filter);
  }

  pub fn with_branch_filter(mut self, branch_filter: String) -> EditHookOption {
    self.branch_filter = Some(branch_filter);
    self
  }

  pub fn branch_filter(&self) -> Option<&String> {
    self.branch_filter.as_ref()
  }

  pub fn reset_branch_filter(&mut self) {
    self.branch_filter = None;
  }

  pub fn set_config(&mut self, config: ::std::collections::HashMap<String, String>) {
    self.config = Some(config);
  }

  pub fn with_config(mut self, config: ::std::collections::HashMap<String, String>) -> EditHookOption {
    self.config = Some(config);
    self
  }

  pub fn config(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.config.as_ref()
  }

  pub fn reset_config(&mut self) {
    self.config = None;
  }

  pub fn set_events(&mut self, events: Vec<String>) {
    self.events = Some(events);
  }

  pub fn with_events(mut self, events: Vec<String>) -> EditHookOption {
    self.events = Some(events);
    self
  }

  pub fn events(&self) -> Option<&Vec<String>> {
    self.events.as_ref()
  }

  pub fn reset_events(&mut self) {
    self.events = None;
  }

}



