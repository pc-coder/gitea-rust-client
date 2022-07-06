/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Milestone : Milestone milestone is a collection of issues on one repository

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
  #[serde(rename = "closed_at")]
  closed_at: Option<String>,
  #[serde(rename = "closed_issues")]
  closed_issues: Option<i64>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "due_on")]
  due_on: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "open_issues")]
  open_issues: Option<i64>,
  #[serde(rename = "state")]
  state: Option<::models::StateType>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "updated_at")]
  updated_at: Option<String>
}

impl Milestone {
  /// Milestone milestone is a collection of issues on one repository
  pub fn new() -> Milestone {
    Milestone {
      closed_at: None,
      closed_issues: None,
      created_at: None,
      description: None,
      due_on: None,
      id: None,
      open_issues: None,
      state: None,
      title: None,
      updated_at: None
    }
  }

  pub fn set_closed_at(&mut self, closed_at: String) {
    self.closed_at = Some(closed_at);
  }

  pub fn with_closed_at(mut self, closed_at: String) -> Milestone {
    self.closed_at = Some(closed_at);
    self
  }

  pub fn closed_at(&self) -> Option<&String> {
    self.closed_at.as_ref()
  }

  pub fn reset_closed_at(&mut self) {
    self.closed_at = None;
  }

  pub fn set_closed_issues(&mut self, closed_issues: i64) {
    self.closed_issues = Some(closed_issues);
  }

  pub fn with_closed_issues(mut self, closed_issues: i64) -> Milestone {
    self.closed_issues = Some(closed_issues);
    self
  }

  pub fn closed_issues(&self) -> Option<&i64> {
    self.closed_issues.as_ref()
  }

  pub fn reset_closed_issues(&mut self) {
    self.closed_issues = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> Milestone {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> Milestone {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_due_on(&mut self, due_on: String) {
    self.due_on = Some(due_on);
  }

  pub fn with_due_on(mut self, due_on: String) -> Milestone {
    self.due_on = Some(due_on);
    self
  }

  pub fn due_on(&self) -> Option<&String> {
    self.due_on.as_ref()
  }

  pub fn reset_due_on(&mut self) {
    self.due_on = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Milestone {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_open_issues(&mut self, open_issues: i64) {
    self.open_issues = Some(open_issues);
  }

  pub fn with_open_issues(mut self, open_issues: i64) -> Milestone {
    self.open_issues = Some(open_issues);
    self
  }

  pub fn open_issues(&self) -> Option<&i64> {
    self.open_issues.as_ref()
  }

  pub fn reset_open_issues(&mut self) {
    self.open_issues = None;
  }

  pub fn set_state(&mut self, state: ::models::StateType) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::StateType) -> Milestone {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::StateType> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> Milestone {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> Milestone {
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



