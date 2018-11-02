# V1PodCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | **String** | Last time we probed the condition. | [optional] [default to null]
**last_transition_time** | **String** | Last time the condition transitioned from one status to another. | [optional] [default to null]
**message** | **String** | Human-readable message indicating details about last transition. | [optional] [default to null]
**reason** | **String** | Unique, one-word, CamelCase reason for the condition&#39;s last transition. | [optional] [default to null]
**status** | **String** | Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions | [default to null]
**_type** | **String** | Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

