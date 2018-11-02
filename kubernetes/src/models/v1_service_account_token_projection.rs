/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ServiceAccountTokenProjection : ServiceAccountTokenProjection represents a projected service account token volume. This projection can be used to insert a service account token into the pods runtime filesystem for use against APIs (Kubernetes API Server or otherwise).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ServiceAccountTokenProjection {
  /// Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
  #[serde(rename = "audience")]
  audience: Option<String>,
  /// ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
  #[serde(rename = "expirationSeconds")]
  expiration_seconds: Option<i64>,
  /// Path is the path relative to the mount point of the file to project the token into.
  #[serde(rename = "path")]
  path: String
}

impl V1ServiceAccountTokenProjection {
  /// ServiceAccountTokenProjection represents a projected service account token volume. This projection can be used to insert a service account token into the pods runtime filesystem for use against APIs (Kubernetes API Server or otherwise).
  pub fn new(path: String) -> V1ServiceAccountTokenProjection {
    V1ServiceAccountTokenProjection {
      audience: None,
      expiration_seconds: None,
      path: path
    }
  }

  pub fn set_audience(&mut self, audience: String) {
    self.audience = Some(audience);
  }

  pub fn with_audience(mut self, audience: String) -> V1ServiceAccountTokenProjection {
    self.audience = Some(audience);
    self
  }

  pub fn audience(&self) -> Option<&String> {
    self.audience.as_ref()
  }

  pub fn reset_audience(&mut self) {
    self.audience = None;
  }

  pub fn set_expiration_seconds(&mut self, expiration_seconds: i64) {
    self.expiration_seconds = Some(expiration_seconds);
  }

  pub fn with_expiration_seconds(mut self, expiration_seconds: i64) -> V1ServiceAccountTokenProjection {
    self.expiration_seconds = Some(expiration_seconds);
    self
  }

  pub fn expiration_seconds(&self) -> Option<&i64> {
    self.expiration_seconds.as_ref()
  }

  pub fn reset_expiration_seconds(&mut self) {
    self.expiration_seconds = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> V1ServiceAccountTokenProjection {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


}


