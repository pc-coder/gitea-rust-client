/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GpgKey : GPGKey a user GPG key to sign commit and tag in repository

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GpgKey {
  #[serde(rename = "can_certify")]
  can_certify: Option<bool>,
  #[serde(rename = "can_encrypt_comms")]
  can_encrypt_comms: Option<bool>,
  #[serde(rename = "can_encrypt_storage")]
  can_encrypt_storage: Option<bool>,
  #[serde(rename = "can_sign")]
  can_sign: Option<bool>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "emails")]
  emails: Option<Vec<::models::GpgKeyEmail>>,
  #[serde(rename = "expires_at")]
  expires_at: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "key_id")]
  key_id: Option<String>,
  #[serde(rename = "primary_key_id")]
  primary_key_id: Option<String>,
  #[serde(rename = "public_key")]
  public_key: Option<String>,
  #[serde(rename = "subkeys")]
  subkeys: Option<Vec<::models::GpgKey>>,
  #[serde(rename = "verified")]
  verified: Option<bool>
}

impl GpgKey {
  /// GPGKey a user GPG key to sign commit and tag in repository
  pub fn new() -> GpgKey {
    GpgKey {
      can_certify: None,
      can_encrypt_comms: None,
      can_encrypt_storage: None,
      can_sign: None,
      created_at: None,
      emails: None,
      expires_at: None,
      id: None,
      key_id: None,
      primary_key_id: None,
      public_key: None,
      subkeys: None,
      verified: None
    }
  }

  pub fn set_can_certify(&mut self, can_certify: bool) {
    self.can_certify = Some(can_certify);
  }

  pub fn with_can_certify(mut self, can_certify: bool) -> GpgKey {
    self.can_certify = Some(can_certify);
    self
  }

  pub fn can_certify(&self) -> Option<&bool> {
    self.can_certify.as_ref()
  }

  pub fn reset_can_certify(&mut self) {
    self.can_certify = None;
  }

  pub fn set_can_encrypt_comms(&mut self, can_encrypt_comms: bool) {
    self.can_encrypt_comms = Some(can_encrypt_comms);
  }

  pub fn with_can_encrypt_comms(mut self, can_encrypt_comms: bool) -> GpgKey {
    self.can_encrypt_comms = Some(can_encrypt_comms);
    self
  }

  pub fn can_encrypt_comms(&self) -> Option<&bool> {
    self.can_encrypt_comms.as_ref()
  }

  pub fn reset_can_encrypt_comms(&mut self) {
    self.can_encrypt_comms = None;
  }

  pub fn set_can_encrypt_storage(&mut self, can_encrypt_storage: bool) {
    self.can_encrypt_storage = Some(can_encrypt_storage);
  }

  pub fn with_can_encrypt_storage(mut self, can_encrypt_storage: bool) -> GpgKey {
    self.can_encrypt_storage = Some(can_encrypt_storage);
    self
  }

  pub fn can_encrypt_storage(&self) -> Option<&bool> {
    self.can_encrypt_storage.as_ref()
  }

  pub fn reset_can_encrypt_storage(&mut self) {
    self.can_encrypt_storage = None;
  }

  pub fn set_can_sign(&mut self, can_sign: bool) {
    self.can_sign = Some(can_sign);
  }

  pub fn with_can_sign(mut self, can_sign: bool) -> GpgKey {
    self.can_sign = Some(can_sign);
    self
  }

  pub fn can_sign(&self) -> Option<&bool> {
    self.can_sign.as_ref()
  }

  pub fn reset_can_sign(&mut self) {
    self.can_sign = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> GpgKey {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_emails(&mut self, emails: Vec<::models::GpgKeyEmail>) {
    self.emails = Some(emails);
  }

  pub fn with_emails(mut self, emails: Vec<::models::GpgKeyEmail>) -> GpgKey {
    self.emails = Some(emails);
    self
  }

  pub fn emails(&self) -> Option<&Vec<::models::GpgKeyEmail>> {
    self.emails.as_ref()
  }

  pub fn reset_emails(&mut self) {
    self.emails = None;
  }

  pub fn set_expires_at(&mut self, expires_at: String) {
    self.expires_at = Some(expires_at);
  }

  pub fn with_expires_at(mut self, expires_at: String) -> GpgKey {
    self.expires_at = Some(expires_at);
    self
  }

  pub fn expires_at(&self) -> Option<&String> {
    self.expires_at.as_ref()
  }

  pub fn reset_expires_at(&mut self) {
    self.expires_at = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> GpgKey {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_key_id(&mut self, key_id: String) {
    self.key_id = Some(key_id);
  }

  pub fn with_key_id(mut self, key_id: String) -> GpgKey {
    self.key_id = Some(key_id);
    self
  }

  pub fn key_id(&self) -> Option<&String> {
    self.key_id.as_ref()
  }

  pub fn reset_key_id(&mut self) {
    self.key_id = None;
  }

  pub fn set_primary_key_id(&mut self, primary_key_id: String) {
    self.primary_key_id = Some(primary_key_id);
  }

  pub fn with_primary_key_id(mut self, primary_key_id: String) -> GpgKey {
    self.primary_key_id = Some(primary_key_id);
    self
  }

  pub fn primary_key_id(&self) -> Option<&String> {
    self.primary_key_id.as_ref()
  }

  pub fn reset_primary_key_id(&mut self) {
    self.primary_key_id = None;
  }

  pub fn set_public_key(&mut self, public_key: String) {
    self.public_key = Some(public_key);
  }

  pub fn with_public_key(mut self, public_key: String) -> GpgKey {
    self.public_key = Some(public_key);
    self
  }

  pub fn public_key(&self) -> Option<&String> {
    self.public_key.as_ref()
  }

  pub fn reset_public_key(&mut self) {
    self.public_key = None;
  }

  pub fn set_subkeys(&mut self, subkeys: Vec<::models::GpgKey>) {
    self.subkeys = Some(subkeys);
  }

  pub fn with_subkeys(mut self, subkeys: Vec<::models::GpgKey>) -> GpgKey {
    self.subkeys = Some(subkeys);
    self
  }

  pub fn subkeys(&self) -> Option<&Vec<::models::GpgKey>> {
    self.subkeys.as_ref()
  }

  pub fn reset_subkeys(&mut self) {
    self.subkeys = None;
  }

  pub fn set_verified(&mut self, verified: bool) {
    self.verified = Some(verified);
  }

  pub fn with_verified(mut self, verified: bool) -> GpgKey {
    self.verified = Some(verified);
    self
  }

  pub fn verified(&self) -> Option<&bool> {
    self.verified.as_ref()
  }

  pub fn reset_verified(&mut self) {
    self.verified = None;
  }

}



