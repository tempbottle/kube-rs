/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodCondition : PodCondition contains details for the current condition of this pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PodCondition {
  /// Last time we probed the condition.
  #[serde(rename = "lastProbeTime")]
  last_probe_time: Option<String>,
  /// Last time the condition transitioned from one status to another.
  #[serde(rename = "lastTransitionTime")]
  last_transition_time: Option<String>,
  /// Human-readable message indicating details about last transition.
  #[serde(rename = "message")]
  message: Option<String>,
  /// Unique, one-word, CamelCase reason for the condition's last transition.
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
  #[serde(rename = "status")]
  status: String,
  /// Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
  #[serde(rename = "type")]
  _type: String
}

impl V1PodCondition {
  /// PodCondition contains details for the current condition of this pod.
  pub fn new(status: String, _type: String) -> V1PodCondition {
    V1PodCondition {
      last_probe_time: None,
      last_transition_time: None,
      message: None,
      reason: None,
      status: status,
      _type: _type
    }
  }

  pub fn set_last_probe_time(&mut self, last_probe_time: String) {
    self.last_probe_time = Some(last_probe_time);
  }

  pub fn with_last_probe_time(mut self, last_probe_time: String) -> V1PodCondition {
    self.last_probe_time = Some(last_probe_time);
    self
  }

  pub fn last_probe_time(&self) -> Option<&String> {
    self.last_probe_time.as_ref()
  }

  pub fn reset_last_probe_time(&mut self) {
    self.last_probe_time = None;
  }

  pub fn set_last_transition_time(&mut self, last_transition_time: String) {
    self.last_transition_time = Some(last_transition_time);
  }

  pub fn with_last_transition_time(mut self, last_transition_time: String) -> V1PodCondition {
    self.last_transition_time = Some(last_transition_time);
    self
  }

  pub fn last_transition_time(&self) -> Option<&String> {
    self.last_transition_time.as_ref()
  }

  pub fn reset_last_transition_time(&mut self) {
    self.last_transition_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> V1PodCondition {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> V1PodCondition {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> V1PodCondition {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> V1PodCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}


