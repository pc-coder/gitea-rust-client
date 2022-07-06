/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitUser {
  #[serde(rename = "date")]
  date: Option<String>,
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>
}

impl CommitUser {
  pub fn new() -> CommitUser {
    CommitUser {
      date: None,
      email: None,
      name: None
    }
  }

  pub fn set_date(&mut self, date: String) {
    self.date = Some(date);
  }

  pub fn with_date(mut self, date: String) -> CommitUser {
    self.date = Some(date);
    self
  }

  pub fn date(&self) -> Option<&String> {
    self.date.as_ref()
  }

  pub fn reset_date(&mut self) {
    self.date = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> CommitUser {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> CommitUser {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}


