/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.18.0+dev-14-g1e2c2edab
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CommitStatusState : CommitStatusState holds the state of a CommitStatus It can be \"pending\", \"success\", \"error\", \"failure\", and \"warning\"

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitStatusState {
}

impl CommitStatusState {
  /// CommitStatusState holds the state of a CommitStatus It can be \"pending\", \"success\", \"error\", \"failure\", and \"warning\"
  pub fn new() -> CommitStatusState {
    CommitStatusState {
    }
  }

}



