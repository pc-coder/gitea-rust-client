/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateUserOption : CreateUserOption create user options

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserOption {
  #[serde(rename = "email")]
  email: String,
  #[serde(rename = "full_name")]
  full_name: Option<String>,
  #[serde(rename = "login_name")]
  login_name: Option<String>,
  #[serde(rename = "must_change_password")]
  must_change_password: Option<bool>,
  #[serde(rename = "password")]
  password: String,
  #[serde(rename = "restricted")]
  restricted: Option<bool>,
  #[serde(rename = "send_notify")]
  send_notify: Option<bool>,
  #[serde(rename = "source_id")]
  source_id: Option<i64>,
  #[serde(rename = "username")]
  username: String,
  #[serde(rename = "visibility")]
  visibility: Option<String>
}

impl CreateUserOption {
  /// CreateUserOption create user options
  pub fn new(email: String, password: String, username: String) -> CreateUserOption {
    CreateUserOption {
      email: email,
      full_name: None,
      login_name: None,
      must_change_password: None,
      password: password,
      restricted: None,
      send_notify: None,
      source_id: None,
      username: username,
      visibility: None
    }
  }

  pub fn set_email(&mut self, email: String) {
    self.email = email;
  }

  pub fn with_email(mut self, email: String) -> CreateUserOption {
    self.email = email;
    self
  }

  pub fn email(&self) -> &String {
    &self.email
  }


  pub fn set_full_name(&mut self, full_name: String) {
    self.full_name = Some(full_name);
  }

  pub fn with_full_name(mut self, full_name: String) -> CreateUserOption {
    self.full_name = Some(full_name);
    self
  }

  pub fn full_name(&self) -> Option<&String> {
    self.full_name.as_ref()
  }

  pub fn reset_full_name(&mut self) {
    self.full_name = None;
  }

  pub fn set_login_name(&mut self, login_name: String) {
    self.login_name = Some(login_name);
  }

  pub fn with_login_name(mut self, login_name: String) -> CreateUserOption {
    self.login_name = Some(login_name);
    self
  }

  pub fn login_name(&self) -> Option<&String> {
    self.login_name.as_ref()
  }

  pub fn reset_login_name(&mut self) {
    self.login_name = None;
  }

  pub fn set_must_change_password(&mut self, must_change_password: bool) {
    self.must_change_password = Some(must_change_password);
  }

  pub fn with_must_change_password(mut self, must_change_password: bool) -> CreateUserOption {
    self.must_change_password = Some(must_change_password);
    self
  }

  pub fn must_change_password(&self) -> Option<&bool> {
    self.must_change_password.as_ref()
  }

  pub fn reset_must_change_password(&mut self) {
    self.must_change_password = None;
  }

  pub fn set_password(&mut self, password: String) {
    self.password = password;
  }

  pub fn with_password(mut self, password: String) -> CreateUserOption {
    self.password = password;
    self
  }

  pub fn password(&self) -> &String {
    &self.password
  }


  pub fn set_restricted(&mut self, restricted: bool) {
    self.restricted = Some(restricted);
  }

  pub fn with_restricted(mut self, restricted: bool) -> CreateUserOption {
    self.restricted = Some(restricted);
    self
  }

  pub fn restricted(&self) -> Option<&bool> {
    self.restricted.as_ref()
  }

  pub fn reset_restricted(&mut self) {
    self.restricted = None;
  }

  pub fn set_send_notify(&mut self, send_notify: bool) {
    self.send_notify = Some(send_notify);
  }

  pub fn with_send_notify(mut self, send_notify: bool) -> CreateUserOption {
    self.send_notify = Some(send_notify);
    self
  }

  pub fn send_notify(&self) -> Option<&bool> {
    self.send_notify.as_ref()
  }

  pub fn reset_send_notify(&mut self) {
    self.send_notify = None;
  }

  pub fn set_source_id(&mut self, source_id: i64) {
    self.source_id = Some(source_id);
  }

  pub fn with_source_id(mut self, source_id: i64) -> CreateUserOption {
    self.source_id = Some(source_id);
    self
  }

  pub fn source_id(&self) -> Option<&i64> {
    self.source_id.as_ref()
  }

  pub fn reset_source_id(&mut self) {
    self.source_id = None;
  }

  pub fn set_username(&mut self, username: String) {
    self.username = username;
  }

  pub fn with_username(mut self, username: String) -> CreateUserOption {
    self.username = username;
    self
  }

  pub fn username(&self) -> &String {
    &self.username
  }


  pub fn set_visibility(&mut self, visibility: String) {
    self.visibility = Some(visibility);
  }

  pub fn with_visibility(mut self, visibility: String) -> CreateUserOption {
    self.visibility = Some(visibility);
    self
  }

  pub fn visibility(&self) -> Option<&String> {
    self.visibility.as_ref()
  }

  pub fn reset_visibility(&mut self) {
    self.visibility = None;
  }

}



