/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1VolumeMount : VolumeMount describes a mounting of a Volume within a container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1VolumeMount {
  /// Path within the container at which the volume should be mounted.  Must not contain ':'.
  #[serde(rename = "mountPath")]
  mount_path: String,
  /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
  #[serde(rename = "mountPropagation")]
  mount_propagation: Option<String>,
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

impl V1VolumeMount {
  /// VolumeMount describes a mounting of a Volume within a container.
  pub fn new(mount_path: String, name: String) -> V1VolumeMount {
    V1VolumeMount {
      mount_path: mount_path,
      mount_propagation: None,
      name: name,
      read_only: None,
      sub_path: None
    }
  }

  pub fn set_mount_path(&mut self, mount_path: String) {
    self.mount_path = mount_path;
  }

  pub fn with_mount_path(mut self, mount_path: String) -> V1VolumeMount {
    self.mount_path = mount_path;
    self
  }

  pub fn mount_path(&self) -> &String {
    &self.mount_path
  }


  pub fn set_mount_propagation(&mut self, mount_propagation: String) {
    self.mount_propagation = Some(mount_propagation);
  }

  pub fn with_mount_propagation(mut self, mount_propagation: String) -> V1VolumeMount {
    self.mount_propagation = Some(mount_propagation);
    self
  }

  pub fn mount_propagation(&self) -> Option<&String> {
    self.mount_propagation.as_ref()
  }

  pub fn reset_mount_propagation(&mut self) {
    self.mount_propagation = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1VolumeMount {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1VolumeMount {
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

  pub fn with_sub_path(mut self, sub_path: String) -> V1VolumeMount {
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


