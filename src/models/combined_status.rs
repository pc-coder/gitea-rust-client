/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CombinedStatus : CombinedStatus holds the combined state of several statuses for a single commit

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinedStatus {
  #[serde(rename = "commit_url")]
  commit_url: Option<String>,
  #[serde(rename = "repository")]
  repository: Option<::models::Repository>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "state")]
  state: Option<::models::CommitStatusState>,
  #[serde(rename = "statuses")]
  statuses: Option<Vec<::models::CommitStatus>>,
  #[serde(rename = "total_count")]
  total_count: Option<i64>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl CombinedStatus {
  /// CombinedStatus holds the combined state of several statuses for a single commit
  pub fn new() -> CombinedStatus {
    CombinedStatus {
      commit_url: None,
      repository: None,
      sha: None,
      state: None,
      statuses: None,
      total_count: None,
      url: None
    }
  }

  pub fn set_commit_url(&mut self, commit_url: String) {
    self.commit_url = Some(commit_url);
  }

  pub fn with_commit_url(mut self, commit_url: String) -> CombinedStatus {
    self.commit_url = Some(commit_url);
    self
  }

  pub fn commit_url(&self) -> Option<&String> {
    self.commit_url.as_ref()
  }

  pub fn reset_commit_url(&mut self) {
    self.commit_url = None;
  }

  pub fn set_repository(&mut self, repository: ::models::Repository) {
    self.repository = Some(repository);
  }

  pub fn with_repository(mut self, repository: ::models::Repository) -> CombinedStatus {
    self.repository = Some(repository);
    self
  }

  pub fn repository(&self) -> Option<&::models::Repository> {
    self.repository.as_ref()
  }

  pub fn reset_repository(&mut self) {
    self.repository = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> CombinedStatus {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_state(&mut self, state: ::models::CommitStatusState) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::CommitStatusState) -> CombinedStatus {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::CommitStatusState> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_statuses(&mut self, statuses: Vec<::models::CommitStatus>) {
    self.statuses = Some(statuses);
  }

  pub fn with_statuses(mut self, statuses: Vec<::models::CommitStatus>) -> CombinedStatus {
    self.statuses = Some(statuses);
    self
  }

  pub fn statuses(&self) -> Option<&Vec<::models::CommitStatus>> {
    self.statuses.as_ref()
  }

  pub fn reset_statuses(&mut self) {
    self.statuses = None;
  }

  pub fn set_total_count(&mut self, total_count: i64) {
    self.total_count = Some(total_count);
  }

  pub fn with_total_count(mut self, total_count: i64) -> CombinedStatus {
    self.total_count = Some(total_count);
    self
  }

  pub fn total_count(&self) -> Option<&i64> {
    self.total_count.as_ref()
  }

  pub fn reset_total_count(&mut self) {
    self.total_count = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> CombinedStatus {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



