# V1beta1CustomResourceDefinitionSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_printer_columns** | [**Vec<::models::V1beta1CustomResourceColumnDefinition>**](v1beta1.CustomResourceColumnDefinition.md) | AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name. Defaults to a created-at column. | [optional] [default to null]
**group** | **String** | Group is the group this resource belongs in | [default to null]
**names** | [***::models::V1beta1CustomResourceDefinitionNames**](v1beta1.CustomResourceDefinitionNames.md) | Names are the names used to describe this custom resource | [default to null]
**scope** | **String** | Scope indicates whether this resource is cluster or namespace scoped.  Default is namespaced | [default to null]
**subresources** | [***::models::V1beta1CustomResourceSubresources**](v1beta1.CustomResourceSubresources.md) | Subresources describes the subresources for CustomResources | [optional] [default to null]
**validation** | [***::models::V1beta1CustomResourceValidation**](v1beta1.CustomResourceValidation.md) | Validation describes the validation methods for CustomResources | [optional] [default to null]
**version** | **String** | Version is the version this resource belongs in Should be always first item in Versions field if provided. Optional, but at least one of Version or Versions must be set. Deprecated: Please use &#x60;Versions&#x60;. | [optional] [default to null]
**versions** | [**Vec<::models::V1beta1CustomResourceDefinitionVersion>**](v1beta1.CustomResourceDefinitionVersion.md) | Versions is the list of all supported versions for this resource. If Version field is provided, this field is optional. Validation: All versions must use the same validation schema for now. i.e., top level Validation field is applied to all of these versions. Order: The version name will be used to compute the order. If the version string is \&quot;kube-like\&quot;, it will sort above non \&quot;kube-like\&quot; version strings, which are ordered lexicographically. \&quot;Kube-like\&quot; versions start with a \&quot;v\&quot;, then are followed by a number (the major version), then optionally the string \&quot;alpha\&quot; or \&quot;beta\&quot; and another number (the minor version). These are sorted first by GA &gt; beta &gt; alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

