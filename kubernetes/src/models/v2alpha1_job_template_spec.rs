/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V2alpha1JobTemplateSpec : JobTemplateSpec describes the data a Job should have when created from a template

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2alpha1JobTemplateSpec {
  /// Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::V1ObjectMeta>,
  /// Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
  #[serde(rename = "spec")]
  spec: Option<::models::V1JobSpec>
}

impl V2alpha1JobTemplateSpec {
  /// JobTemplateSpec describes the data a Job should have when created from a template
  pub fn new() -> V2alpha1JobTemplateSpec {
    V2alpha1JobTemplateSpec {
      metadata: None,
      spec: None
    }
  }

  pub fn set_metadata(&mut self, metadata: ::models::V1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::V1ObjectMeta) -> V2alpha1JobTemplateSpec {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::V1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::V1JobSpec) {
    self.spec = Some(spec);
  }

  pub fn with_spec(mut self, spec: ::models::V1JobSpec) -> V2alpha1JobTemplateSpec {
    self.spec = Some(spec);
    self
  }

  pub fn spec(&self) -> Option<&::models::V1JobSpec> {
    self.spec.as_ref()
  }

  pub fn reset_spec(&mut self) {
    self.spec = None;
  }

}


