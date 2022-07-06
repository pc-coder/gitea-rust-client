/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GeneralUiSettings : GeneralUISettings contains global ui settings exposed by API

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralUiSettings {
  #[serde(rename = "allowed_reactions")]
  allowed_reactions: Option<Vec<String>>,
  #[serde(rename = "custom_emojis")]
  custom_emojis: Option<Vec<String>>,
  #[serde(rename = "default_theme")]
  default_theme: Option<String>
}

impl GeneralUiSettings {
  /// GeneralUISettings contains global ui settings exposed by API
  pub fn new() -> GeneralUiSettings {
    GeneralUiSettings {
      allowed_reactions: None,
      custom_emojis: None,
      default_theme: None
    }
  }

  pub fn set_allowed_reactions(&mut self, allowed_reactions: Vec<String>) {
    self.allowed_reactions = Some(allowed_reactions);
  }

  pub fn with_allowed_reactions(mut self, allowed_reactions: Vec<String>) -> GeneralUiSettings {
    self.allowed_reactions = Some(allowed_reactions);
    self
  }

  pub fn allowed_reactions(&self) -> Option<&Vec<String>> {
    self.allowed_reactions.as_ref()
  }

  pub fn reset_allowed_reactions(&mut self) {
    self.allowed_reactions = None;
  }

  pub fn set_custom_emojis(&mut self, custom_emojis: Vec<String>) {
    self.custom_emojis = Some(custom_emojis);
  }

  pub fn with_custom_emojis(mut self, custom_emojis: Vec<String>) -> GeneralUiSettings {
    self.custom_emojis = Some(custom_emojis);
    self
  }

  pub fn custom_emojis(&self) -> Option<&Vec<String>> {
    self.custom_emojis.as_ref()
  }

  pub fn reset_custom_emojis(&mut self) {
    self.custom_emojis = None;
  }

  pub fn set_default_theme(&mut self, default_theme: String) {
    self.default_theme = Some(default_theme);
  }

  pub fn with_default_theme(mut self, default_theme: String) -> GeneralUiSettings {
    self.default_theme = Some(default_theme);
    self
  }

  pub fn default_theme(&self) -> Option<&String> {
    self.default_theme.as_ref()
  }

  pub fn reset_default_theme(&mut self) {
    self.default_theme = None;
  }

}


