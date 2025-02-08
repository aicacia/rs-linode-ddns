# PostLkeClusterRequestNodePoolsInnerTaintsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**effect** | **String** | The Kubernetes taint effect. For `NoSchedule`, `PreferNoSchedule` and `NoExecute` descriptions, see [Kubernetes Taints and Tolerations](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/). | 
**key** | **String** | The Kubernetes taint key.  - A key can contain alphanumeric characters, dashes (`-`), underscores (`_`), or dots (`.`). Start and end it with an alphanumeric character.  - If the key begins with a DNS subdomain prefix followed by a single slash, for example `example.com/my-app`, the prefix part needs to adhere to [RFC 1123](https://datatracker.ietf.org/doc/html/rfc1123) DNS subdomain restrictions and be a maximum of 253 characters. | 
**value** | **String** | The Kubernetes taint value.  - A key can contain alphanumeric characters, dashes (`-`), underscores (`_`), or dots (`.`). Start and end it with an alphanumeric character.  - Can be specified as an empty string value with `\"\"`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


