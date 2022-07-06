/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateTeamOption : CreateTeamOption options for creating a team

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamOption {
  #[serde(rename = "can_create_org_repo")]
  can_create_org_repo: Option<bool>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "includes_all_repositories")]
  includes_all_repositories: Option<bool>,
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "permission")]
  permission: Option<String>,
  #[serde(rename = "units")]
  units: Option<Vec<String>>,
  #[serde(rename = "units_map")]
  units_map: Option<::std::collections::HashMap<String, String>>
}

impl CreateTeamOption {
  /// CreateTeamOption options for creating a team
  pub fn new(name: String) -> CreateTeamOption {
    CreateTeamOption {
      can_create_org_repo: None,
      description: None,
      includes_all_repositories: None,
      name: name,
      permission: None,
      units: None,
      units_map: None
    }
  }

  pub fn set_can_create_org_repo(&mut self, can_create_org_repo: bool) {
    self.can_create_org_repo = Some(can_create_org_repo);
  }

  pub fn with_can_create_org_repo(mut self, can_create_org_repo: bool) -> CreateTeamOption {
    self.can_create_org_repo = Some(can_create_org_repo);
    self
  }

  pub fn can_create_org_repo(&self) -> Option<&bool> {
    self.can_create_org_repo.as_ref()
  }

  pub fn reset_can_create_org_repo(&mut self) {
    self.can_create_org_repo = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> CreateTeamOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_includes_all_repositories(&mut self, includes_all_repositories: bool) {
    self.includes_all_repositories = Some(includes_all_repositories);
  }

  pub fn with_includes_all_repositories(mut self, includes_all_repositories: bool) -> CreateTeamOption {
    self.includes_all_repositories = Some(includes_all_repositories);
    self
  }

  pub fn includes_all_repositories(&self) -> Option<&bool> {
    self.includes_all_repositories.as_ref()
  }

  pub fn reset_includes_all_repositories(&mut self) {
    self.includes_all_repositories = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> CreateTeamOption {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_permission(&mut self, permission: String) {
    self.permission = Some(permission);
  }

  pub fn with_permission(mut self, permission: String) -> CreateTeamOption {
    self.permission = Some(permission);
    self
  }

  pub fn permission(&self) -> Option<&String> {
    self.permission.as_ref()
  }

  pub fn reset_permission(&mut self) {
    self.permission = None;
  }

  pub fn set_units(&mut self, units: Vec<String>) {
    self.units = Some(units);
  }

  pub fn with_units(mut self, units: Vec<String>) -> CreateTeamOption {
    self.units = Some(units);
    self
  }

  pub fn units(&self) -> Option<&Vec<String>> {
    self.units.as_ref()
  }

  pub fn reset_units(&mut self) {
    self.units = None;
  }

  pub fn set_units_map(&mut self, units_map: ::std::collections::HashMap<String, String>) {
    self.units_map = Some(units_map);
  }

  pub fn with_units_map(mut self, units_map: ::std::collections::HashMap<String, String>) -> CreateTeamOption {
    self.units_map = Some(units_map);
    self
  }

  pub fn units_map(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.units_map.as_ref()
  }

  pub fn reset_units_map(&mut self) {
    self.units_map = None;
  }

}



