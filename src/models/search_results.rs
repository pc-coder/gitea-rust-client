/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SearchResults : SearchResults results of a successful search

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResults {
  #[serde(rename = "data")]
  data: Option<Vec<::models::Repository>>,
  #[serde(rename = "ok")]
  ok: Option<bool>
}

impl SearchResults {
  /// SearchResults results of a successful search
  pub fn new() -> SearchResults {
    SearchResults {
      data: None,
      ok: None
    }
  }

  pub fn set_data(&mut self, data: Vec<::models::Repository>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Vec<::models::Repository>) -> SearchResults {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Vec<::models::Repository>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_ok(&mut self, ok: bool) {
    self.ok = Some(ok);
  }

  pub fn with_ok(mut self, ok: bool) -> SearchResults {
    self.ok = Some(ok);
    self
  }

  pub fn ok(&self) -> Option<&bool> {
    self.ok.as_ref()
  }

  pub fn reset_ok(&mut self) {
    self.ok = None;
  }

}



