# GetLkeClusterNode200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Node's ID. | [optional]
**instance_id** | Option<**i32**> | The Linode's ID. When no Linode is currently provisioned for this Node, this will be `null`. | [optional]
**status** | Option<**String**> | The creation status of this Node. This status is distinct from this Node's readiness as a Kubernetes Node Object as determined by the command `kubectl get nodes`.  `not_ready` indicates that the Linode is still being created.  `ready` indicates that the Linode has successfully been created and is running Kubernetes software. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


