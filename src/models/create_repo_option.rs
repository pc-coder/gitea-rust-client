/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateRepoOption : CreateRepoOption options when creating repository

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepoOption {
  /// Whether the repository should be auto-initialized?
  #[serde(rename = "auto_init")]
  auto_init: Option<bool>,
  /// DefaultBranch of the repository (used when initializes and in template)
  #[serde(rename = "default_branch")]
  default_branch: Option<String>,
  /// Description of the repository to create
  #[serde(rename = "description")]
  description: Option<String>,
  /// Gitignores to use
  #[serde(rename = "gitignores")]
  gitignores: Option<String>,
  /// Label-Set to use
  #[serde(rename = "issue_labels")]
  issue_labels: Option<String>,
  /// License to use
  #[serde(rename = "license")]
  license: Option<String>,
  /// Name of the repository to create
  #[serde(rename = "name")]
  name: String,
  /// Whether the repository is private
  #[serde(rename = "private")]
  private: Option<bool>,
  /// Readme of the repository to create
  #[serde(rename = "readme")]
  readme: Option<String>,
  /// Whether the repository is template
  #[serde(rename = "template")]
  template: Option<bool>,
  /// TrustModel of the repository
  #[serde(rename = "trust_model")]
  trust_model: Option<String>
}

impl CreateRepoOption {
  /// CreateRepoOption options when creating repository
  pub fn new(name: String) -> CreateRepoOption {
    CreateRepoOption {
      auto_init: None,
      default_branch: None,
      description: None,
      gitignores: None,
      issue_labels: None,
      license: None,
      name: name,
      private: None,
      readme: None,
      template: None,
      trust_model: None
    }
  }

  pub fn set_auto_init(&mut self, auto_init: bool) {
    self.auto_init = Some(auto_init);
  }

  pub fn with_auto_init(mut self, auto_init: bool) -> CreateRepoOption {
    self.auto_init = Some(auto_init);
    self
  }

  pub fn auto_init(&self) -> Option<&bool> {
    self.auto_init.as_ref()
  }

  pub fn reset_auto_init(&mut self) {
    self.auto_init = None;
  }

  pub fn set_default_branch(&mut self, default_branch: String) {
    self.default_branch = Some(default_branch);
  }

  pub fn with_default_branch(mut self, default_branch: String) -> CreateRepoOption {
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

  pub fn with_description(mut self, description: String) -> CreateRepoOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_gitignores(&mut self, gitignores: String) {
    self.gitignores = Some(gitignores);
  }

  pub fn with_gitignores(mut self, gitignores: String) -> CreateRepoOption {
    self.gitignores = Some(gitignores);
    self
  }

  pub fn gitignores(&self) -> Option<&String> {
    self.gitignores.as_ref()
  }

  pub fn reset_gitignores(&mut self) {
    self.gitignores = None;
  }

  pub fn set_issue_labels(&mut self, issue_labels: String) {
    self.issue_labels = Some(issue_labels);
  }

  pub fn with_issue_labels(mut self, issue_labels: String) -> CreateRepoOption {
    self.issue_labels = Some(issue_labels);
    self
  }

  pub fn issue_labels(&self) -> Option<&String> {
    self.issue_labels.as_ref()
  }

  pub fn reset_issue_labels(&mut self) {
    self.issue_labels = None;
  }

  pub fn set_license(&mut self, license: String) {
    self.license = Some(license);
  }

  pub fn with_license(mut self, license: String) -> CreateRepoOption {
    self.license = Some(license);
    self
  }

  pub fn license(&self) -> Option<&String> {
    self.license.as_ref()
  }

  pub fn reset_license(&mut self) {
    self.license = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> CreateRepoOption {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_private(&mut self, private: bool) {
    self.private = Some(private);
  }

  pub fn with_private(mut self, private: bool) -> CreateRepoOption {
    self.private = Some(private);
    self
  }

  pub fn private(&self) -> Option<&bool> {
    self.private.as_ref()
  }

  pub fn reset_private(&mut self) {
    self.private = None;
  }

  pub fn set_readme(&mut self, readme: String) {
    self.readme = Some(readme);
  }

  pub fn with_readme(mut self, readme: String) -> CreateRepoOption {
    self.readme = Some(readme);
    self
  }

  pub fn readme(&self) -> Option<&String> {
    self.readme.as_ref()
  }

  pub fn reset_readme(&mut self) {
    self.readme = None;
  }

  pub fn set_template(&mut self, template: bool) {
    self.template = Some(template);
  }

  pub fn with_template(mut self, template: bool) -> CreateRepoOption {
    self.template = Some(template);
    self
  }

  pub fn template(&self) -> Option<&bool> {
    self.template.as_ref()
  }

  pub fn reset_template(&mut self) {
    self.template = None;
  }

  pub fn set_trust_model(&mut self, trust_model: String) {
    self.trust_model = Some(trust_model);
  }

  pub fn with_trust_model(mut self, trust_model: String) -> CreateRepoOption {
    self.trust_model = Some(trust_model);
    self
  }

  pub fn trust_model(&self) -> Option<&String> {
    self.trust_model.as_ref()
  }

  pub fn reset_trust_model(&mut self) {
    self.trust_model = None;
  }

}



