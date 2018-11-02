/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionsV1beta1HostPortRange : HostPortRange defines a range of host ports that will be enabled by a policy for pods to use.  It requires both the start and end to be defined. Deprecated: use HostPortRange from policy API Group instead.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsV1beta1HostPortRange {
  /// max is the end of the range, inclusive.
  #[serde(rename = "max")]
  max: i32,
  /// min is the start of the range, inclusive.
  #[serde(rename = "min")]
  min: i32
}

impl ExtensionsV1beta1HostPortRange {
  /// HostPortRange defines a range of host ports that will be enabled by a policy for pods to use.  It requires both the start and end to be defined. Deprecated: use HostPortRange from policy API Group instead.
  pub fn new(max: i32, min: i32) -> ExtensionsV1beta1HostPortRange {
    ExtensionsV1beta1HostPortRange {
      max: max,
      min: min
    }
  }

  pub fn set_max(&mut self, max: i32) {
    self.max = max;
  }

  pub fn with_max(mut self, max: i32) -> ExtensionsV1beta1HostPortRange {
    self.max = max;
    self
  }

  pub fn max(&self) -> &i32 {
    &self.max
  }


  pub fn set_min(&mut self, min: i32) {
    self.min = min;
  }

  pub fn with_min(mut self, min: i32) -> ExtensionsV1beta1HostPortRange {
    self.min = min;
    self
  }

  pub fn min(&self) -> &i32 {
    &self.min
  }


}


