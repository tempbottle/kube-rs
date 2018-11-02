/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V2beta1HorizontalPodAutoscalerSpec : HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2beta1HorizontalPodAutoscalerSpec {
  /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
  #[serde(rename = "maxReplicas")]
  max_replicas: i32,
  /// metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond.
  #[serde(rename = "metrics")]
  metrics: Option<Vec<::models::V2beta1MetricSpec>>,
  /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down. It defaults to 1 pod.
  #[serde(rename = "minReplicas")]
  min_replicas: Option<i32>,
  /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
  #[serde(rename = "scaleTargetRef")]
  scale_target_ref: ::models::V2beta1CrossVersionObjectReference
}

impl V2beta1HorizontalPodAutoscalerSpec {
  /// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
  pub fn new(max_replicas: i32, scale_target_ref: ::models::V2beta1CrossVersionObjectReference) -> V2beta1HorizontalPodAutoscalerSpec {
    V2beta1HorizontalPodAutoscalerSpec {
      max_replicas: max_replicas,
      metrics: None,
      min_replicas: None,
      scale_target_ref: scale_target_ref
    }
  }

  pub fn set_max_replicas(&mut self, max_replicas: i32) {
    self.max_replicas = max_replicas;
  }

  pub fn with_max_replicas(mut self, max_replicas: i32) -> V2beta1HorizontalPodAutoscalerSpec {
    self.max_replicas = max_replicas;
    self
  }

  pub fn max_replicas(&self) -> &i32 {
    &self.max_replicas
  }


  pub fn set_metrics(&mut self, metrics: Vec<::models::V2beta1MetricSpec>) {
    self.metrics = Some(metrics);
  }

  pub fn with_metrics(mut self, metrics: Vec<::models::V2beta1MetricSpec>) -> V2beta1HorizontalPodAutoscalerSpec {
    self.metrics = Some(metrics);
    self
  }

  pub fn metrics(&self) -> Option<&Vec<::models::V2beta1MetricSpec>> {
    self.metrics.as_ref()
  }

  pub fn reset_metrics(&mut self) {
    self.metrics = None;
  }

  pub fn set_min_replicas(&mut self, min_replicas: i32) {
    self.min_replicas = Some(min_replicas);
  }

  pub fn with_min_replicas(mut self, min_replicas: i32) -> V2beta1HorizontalPodAutoscalerSpec {
    self.min_replicas = Some(min_replicas);
    self
  }

  pub fn min_replicas(&self) -> Option<&i32> {
    self.min_replicas.as_ref()
  }

  pub fn reset_min_replicas(&mut self) {
    self.min_replicas = None;
  }

  pub fn set_scale_target_ref(&mut self, scale_target_ref: ::models::V2beta1CrossVersionObjectReference) {
    self.scale_target_ref = scale_target_ref;
  }

  pub fn with_scale_target_ref(mut self, scale_target_ref: ::models::V2beta1CrossVersionObjectReference) -> V2beta1HorizontalPodAutoscalerSpec {
    self.scale_target_ref = scale_target_ref;
    self
  }

  pub fn scale_target_ref(&self) -> &::models::V2beta1CrossVersionObjectReference {
    &self.scale_target_ref
  }


}


