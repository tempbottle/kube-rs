/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1VolumeMount : VolumeMount describes a mounting of a Volume within a container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1VolumeMount {
  /// Path within the container at which the volume should be mounted.  Must not contain ':'.
  #[serde(rename = "mountPath")]
  mount_path: String,
  /// This must match the Name of a Volume.
  #[serde(rename = "name")]
  name: String,
  /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root).
  #[serde(rename = "subPath")]
  sub_path: Option<String>
}

impl IoK8sKubernetesPkgApiV1VolumeMount {
  /// VolumeMount describes a mounting of a Volume within a container.
  pub fn new(mount_path: String, name: String) -> IoK8sKubernetesPkgApiV1VolumeMount {
    IoK8sKubernetesPkgApiV1VolumeMount {
      mount_path: mount_path,
      name: name,
      read_only: None,
      sub_path: None
    }
  }

  pub fn set_mount_path(&mut self, mount_path: String) {
    self.mount_path = mount_path;
  }

  pub fn with_mount_path(mut self, mount_path: String) -> IoK8sKubernetesPkgApiV1VolumeMount {
    self.mount_path = mount_path;
    self
  }

  pub fn mount_path(&self) -> &String {
    &self.mount_path
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sKubernetesPkgApiV1VolumeMount {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> IoK8sKubernetesPkgApiV1VolumeMount {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_sub_path(&mut self, sub_path: String) {
    self.sub_path = Some(sub_path);
  }

  pub fn with_sub_path(mut self, sub_path: String) -> IoK8sKubernetesPkgApiV1VolumeMount {
    self.sub_path = Some(sub_path);
    self
  }

  pub fn sub_path(&self) -> Option<&String> {
    self.sub_path.as_ref()
  }

  pub fn reset_sub_path(&mut self) {
    self.sub_path = None;
  }

}


