/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NotificationCount : NotificationCount number of unread notifications

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationCount {
  #[serde(rename = "new")]
  new: Option<i64>
}

impl NotificationCount {
  /// NotificationCount number of unread notifications
  pub fn new() -> NotificationCount {
    NotificationCount {
      new: None
    }
  }

  pub fn set_new(&mut self, new: i64) {
    self.new = Some(new);
  }

  pub fn with_new(mut self, new: i64) -> NotificationCount {
    self.new = Some(new);
    self
  }

  pub fn new(&self) -> Option<&i64> {
    self.new.as_ref()
  }

  pub fn reset_new(&mut self) {
    self.new = None;
  }

}



