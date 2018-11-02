# V2beta2MetricStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external** | [***::models::V2beta2ExternalMetricStatus**](v2beta2.ExternalMetricStatus.md) | external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). | [optional] [default to null]
**object** | [***::models::V2beta2ObjectMetricStatus**](v2beta2.ObjectMetricStatus.md) | object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object). | [optional] [default to null]
**pods** | [***::models::V2beta2PodsMetricStatus**](v2beta2.PodsMetricStatus.md) | pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value. | [optional] [default to null]
**resource** | [***::models::V2beta2ResourceMetricStatus**](v2beta2.ResourceMetricStatus.md) | resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \&quot;pods\&quot; source. | [optional] [default to null]
**_type** | **String** | type is the type of metric source.  It will be one of \&quot;Object\&quot;, \&quot;Pods\&quot; or \&quot;Resource\&quot;, each corresponds to a matching field in the object. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

