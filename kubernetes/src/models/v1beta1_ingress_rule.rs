/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1IngressRule : IngressRule represents the rules mapping the paths under a specified host to the related backend services. Incoming requests are first evaluated for a host match, then routed to the backend associated with the matching IngressRuleValue.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1IngressRule {
  /// Host is the fully qualified domain name of a network host, as defined by RFC 3986. Note the following deviations from the \"host\" part of the URI as defined in the RFC: 1. IPs are not allowed. Currently an IngressRuleValue can only apply to the    IP in the Spec of the parent Ingress. 2. The `:` delimiter is not respected because ports are not allowed.    Currently the port of an Ingress is implicitly :80 for http and    :443 for https. Both these may change in the future. Incoming requests are matched against the host before the IngressRuleValue. If the host is unspecified, the Ingress routes all traffic based on the specified IngressRuleValue.
  #[serde(rename = "host")]
  host: Option<String>,
  #[serde(rename = "http")]
  http: Option<::models::V1beta1HttpIngressRuleValue>
}

impl V1beta1IngressRule {
  /// IngressRule represents the rules mapping the paths under a specified host to the related backend services. Incoming requests are first evaluated for a host match, then routed to the backend associated with the matching IngressRuleValue.
  pub fn new() -> V1beta1IngressRule {
    V1beta1IngressRule {
      host: None,
      http: None
    }
  }

  pub fn set_host(&mut self, host: String) {
    self.host = Some(host);
  }

  pub fn with_host(mut self, host: String) -> V1beta1IngressRule {
    self.host = Some(host);
    self
  }

  pub fn host(&self) -> Option<&String> {
    self.host.as_ref()
  }

  pub fn reset_host(&mut self) {
    self.host = None;
  }

  pub fn set_http(&mut self, http: ::models::V1beta1HttpIngressRuleValue) {
    self.http = Some(http);
  }

  pub fn with_http(mut self, http: ::models::V1beta1HttpIngressRuleValue) -> V1beta1IngressRule {
    self.http = Some(http);
    self
  }

  pub fn http(&self) -> Option<&::models::V1beta1HttpIngressRuleValue> {
    self.http.as_ref()
  }

  pub fn reset_http(&mut self) {
    self.http = None;
  }

}


