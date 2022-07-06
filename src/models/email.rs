/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Email : Email an email address belonging to a user

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "primary")]
  primary: Option<bool>,
  #[serde(rename = "verified")]
  verified: Option<bool>
}

impl Email {
  /// Email an email address belonging to a user
  pub fn new() -> Email {
    Email {
      email: None,
      primary: None,
      verified: None
    }
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> Email {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_primary(&mut self, primary: bool) {
    self.primary = Some(primary);
  }

  pub fn with_primary(mut self, primary: bool) -> Email {
    self.primary = Some(primary);
    self
  }

  pub fn primary(&self) -> Option<&bool> {
    self.primary.as_ref()
  }

  pub fn reset_primary(&mut self) {
    self.primary = None;
  }

  pub fn set_verified(&mut self, verified: bool) {
    self.verified = Some(verified);
  }

  pub fn with_verified(mut self, verified: bool) -> Email {
    self.verified = Some(verified);
    self
  }

  pub fn verified(&self) -> Option<&bool> {
    self.verified.as_ref()
  }

  pub fn reset_verified(&mut self) {
    self.verified = None;
  }

}



