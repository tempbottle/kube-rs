/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1HostPathVolumeSource : Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1HostPathVolumeSource {
  /// Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
  #[serde(rename = "path")]
  path: String,
  /// Type for HostPath Volume Defaults to \"\" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
  #[serde(rename = "type")]
  _type: Option<String>
}

impl V1HostPathVolumeSource {
  /// Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.
  pub fn new(path: String) -> V1HostPathVolumeSource {
    V1HostPathVolumeSource {
      path: path,
      _type: None
    }
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> V1HostPathVolumeSource {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1HostPathVolumeSource {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}


