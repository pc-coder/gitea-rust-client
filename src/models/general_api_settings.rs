/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GeneralApiSettings : GeneralAPISettings contains global api settings exposed by it

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralApiSettings {
  #[serde(rename = "default_git_trees_per_page")]
  default_git_trees_per_page: Option<i64>,
  #[serde(rename = "default_max_blob_size")]
  default_max_blob_size: Option<i64>,
  #[serde(rename = "default_paging_num")]
  default_paging_num: Option<i64>,
  #[serde(rename = "max_response_items")]
  max_response_items: Option<i64>
}

impl GeneralApiSettings {
  /// GeneralAPISettings contains global api settings exposed by it
  pub fn new() -> GeneralApiSettings {
    GeneralApiSettings {
      default_git_trees_per_page: None,
      default_max_blob_size: None,
      default_paging_num: None,
      max_response_items: None
    }
  }

  pub fn set_default_git_trees_per_page(&mut self, default_git_trees_per_page: i64) {
    self.default_git_trees_per_page = Some(default_git_trees_per_page);
  }

  pub fn with_default_git_trees_per_page(mut self, default_git_trees_per_page: i64) -> GeneralApiSettings {
    self.default_git_trees_per_page = Some(default_git_trees_per_page);
    self
  }

  pub fn default_git_trees_per_page(&self) -> Option<&i64> {
    self.default_git_trees_per_page.as_ref()
  }

  pub fn reset_default_git_trees_per_page(&mut self) {
    self.default_git_trees_per_page = None;
  }

  pub fn set_default_max_blob_size(&mut self, default_max_blob_size: i64) {
    self.default_max_blob_size = Some(default_max_blob_size);
  }

  pub fn with_default_max_blob_size(mut self, default_max_blob_size: i64) -> GeneralApiSettings {
    self.default_max_blob_size = Some(default_max_blob_size);
    self
  }

  pub fn default_max_blob_size(&self) -> Option<&i64> {
    self.default_max_blob_size.as_ref()
  }

  pub fn reset_default_max_blob_size(&mut self) {
    self.default_max_blob_size = None;
  }

  pub fn set_default_paging_num(&mut self, default_paging_num: i64) {
    self.default_paging_num = Some(default_paging_num);
  }

  pub fn with_default_paging_num(mut self, default_paging_num: i64) -> GeneralApiSettings {
    self.default_paging_num = Some(default_paging_num);
    self
  }

  pub fn default_paging_num(&self) -> Option<&i64> {
    self.default_paging_num.as_ref()
  }

  pub fn reset_default_paging_num(&mut self) {
    self.default_paging_num = None;
  }

  pub fn set_max_response_items(&mut self, max_response_items: i64) {
    self.max_response_items = Some(max_response_items);
  }

  pub fn with_max_response_items(mut self, max_response_items: i64) -> GeneralApiSettings {
    self.max_response_items = Some(max_response_items);
    self
  }

  pub fn max_response_items(&self) -> Option<&i64> {
    self.max_response_items.as_ref()
  }

  pub fn reset_max_response_items(&mut self) {
    self.max_response_items = None;
  }

}



