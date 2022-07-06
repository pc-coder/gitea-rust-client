/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GenerateRepoOption : GenerateRepoOption options when creating repository using a template

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRepoOption {
  /// include avatar of the template repo
  #[serde(rename = "avatar")]
  avatar: Option<bool>,
  /// Default branch of the new repository
  #[serde(rename = "default_branch")]
  default_branch: Option<String>,
  /// Description of the repository to create
  #[serde(rename = "description")]
  description: Option<String>,
  /// include git content of default branch in template repo
  #[serde(rename = "git_content")]
  git_content: Option<bool>,
  /// include git hooks in template repo
  #[serde(rename = "git_hooks")]
  git_hooks: Option<bool>,
  /// include labels in template repo
  #[serde(rename = "labels")]
  labels: Option<bool>,
  /// Name of the repository to create
  #[serde(rename = "name")]
  name: String,
  /// The organization or person who will own the new repository
  #[serde(rename = "owner")]
  owner: String,
  /// Whether the repository is private
  #[serde(rename = "private")]
  private: Option<bool>,
  /// include topics in template repo
  #[serde(rename = "topics")]
  topics: Option<bool>,
  /// include webhooks in template repo
  #[serde(rename = "webhooks")]
  webhooks: Option<bool>
}

impl GenerateRepoOption {
  /// GenerateRepoOption options when creating repository using a template
  pub fn new(name: String, owner: String) -> GenerateRepoOption {
    GenerateRepoOption {
      avatar: None,
      default_branch: None,
      description: None,
      git_content: None,
      git_hooks: None,
      labels: None,
      name: name,
      owner: owner,
      private: None,
      topics: None,
      webhooks: None
    }
  }

  pub fn set_avatar(&mut self, avatar: bool) {
    self.avatar = Some(avatar);
  }

  pub fn with_avatar(mut self, avatar: bool) -> GenerateRepoOption {
    self.avatar = Some(avatar);
    self
  }

  pub fn avatar(&self) -> Option<&bool> {
    self.avatar.as_ref()
  }

  pub fn reset_avatar(&mut self) {
    self.avatar = None;
  }

  pub fn set_default_branch(&mut self, default_branch: String) {
    self.default_branch = Some(default_branch);
  }

  pub fn with_default_branch(mut self, default_branch: String) -> GenerateRepoOption {
    self.default_branch = Some(default_branch);
    self
  }

  pub fn default_branch(&self) -> Option<&String> {
    self.default_branch.as_ref()
  }

  pub fn reset_default_branch(&mut self) {
    self.default_branch = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> GenerateRepoOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_git_content(&mut self, git_content: bool) {
    self.git_content = Some(git_content);
  }

  pub fn with_git_content(mut self, git_content: bool) -> GenerateRepoOption {
    self.git_content = Some(git_content);
    self
  }

  pub fn git_content(&self) -> Option<&bool> {
    self.git_content.as_ref()
  }

  pub fn reset_git_content(&mut self) {
    self.git_content = None;
  }

  pub fn set_git_hooks(&mut self, git_hooks: bool) {
    self.git_hooks = Some(git_hooks);
  }

  pub fn with_git_hooks(mut self, git_hooks: bool) -> GenerateRepoOption {
    self.git_hooks = Some(git_hooks);
    self
  }

  pub fn git_hooks(&self) -> Option<&bool> {
    self.git_hooks.as_ref()
  }

  pub fn reset_git_hooks(&mut self) {
    self.git_hooks = None;
  }

  pub fn set_labels(&mut self, labels: bool) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: bool) -> GenerateRepoOption {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&bool> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GenerateRepoOption {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_owner(&mut self, owner: String) {
    self.owner = owner;
  }

  pub fn with_owner(mut self, owner: String) -> GenerateRepoOption {
    self.owner = owner;
    self
  }

  pub fn owner(&self) -> &String {
    &self.owner
  }


  pub fn set_private(&mut self, private: bool) {
    self.private = Some(private);
  }

  pub fn with_private(mut self, private: bool) -> GenerateRepoOption {
    self.private = Some(private);
    self
  }

  pub fn private(&self) -> Option<&bool> {
    self.private.as_ref()
  }

  pub fn reset_private(&mut self) {
    self.private = None;
  }

  pub fn set_topics(&mut self, topics: bool) {
    self.topics = Some(topics);
  }

  pub fn with_topics(mut self, topics: bool) -> GenerateRepoOption {
    self.topics = Some(topics);
    self
  }

  pub fn topics(&self) -> Option<&bool> {
    self.topics.as_ref()
  }

  pub fn reset_topics(&mut self) {
    self.topics = None;
  }

  pub fn set_webhooks(&mut self, webhooks: bool) {
    self.webhooks = Some(webhooks);
  }

  pub fn with_webhooks(mut self, webhooks: bool) -> GenerateRepoOption {
    self.webhooks = Some(webhooks);
    self
  }

  pub fn webhooks(&self) -> Option<&bool> {
    self.webhooks.as_ref()
  }

  pub fn reset_webhooks(&mut self) {
    self.webhooks = None;
  }

}



