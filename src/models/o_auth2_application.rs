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
pub struct OAuth2Application {
  #[serde(rename = "client_id")]
  client_id: Option<String>,
  #[serde(rename = "client_secret")]
  client_secret: Option<String>,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "redirect_uris")]
  redirect_uris: Option<Vec<String>>
}

impl OAuth2Application {
  pub fn new() -> OAuth2Application {
    OAuth2Application {
      client_id: None,
      client_secret: None,
      created: None,
      id: None,
      name: None,
      redirect_uris: None
    }
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> OAuth2Application {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_client_secret(&mut self, client_secret: String) {
    self.client_secret = Some(client_secret);
  }

  pub fn with_client_secret(mut self, client_secret: String) -> OAuth2Application {
    self.client_secret = Some(client_secret);
    self
  }

  pub fn client_secret(&self) -> Option<&String> {
    self.client_secret.as_ref()
  }

  pub fn reset_client_secret(&mut self) {
    self.client_secret = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> OAuth2Application {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> OAuth2Application {
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

  pub fn with_name(mut self, name: String) -> OAuth2Application {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_redirect_uris(&mut self, redirect_uris: Vec<String>) {
    self.redirect_uris = Some(redirect_uris);
  }

  pub fn with_redirect_uris(mut self, redirect_uris: Vec<String>) -> OAuth2Application {
    self.redirect_uris = Some(redirect_uris);
    self
  }

  pub fn redirect_uris(&self) -> Option<&Vec<String>> {
    self.redirect_uris.as_ref()
  }

  pub fn reset_redirect_uris(&mut self) {
    self.redirect_uris = None;
  }

}



