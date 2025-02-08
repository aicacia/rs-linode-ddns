# PostLkeClusterRequestControlPlaneAcl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | Option<[**models::PostLkeClusterRequestControlPlaneAclAddresses**](post_lke_cluster_request_control_plane_acl_addresses.md)> |  | [optional]
**enabled** | Option<**bool**> | Defines a default policy. A value of `true` results in a default policy of `DENY`. A value of `false` results in a default policy of `ALLOW`, such as for disabled access controls. It defaults to `true`. Creating a cluster with ACL, or upgrading a cluster to use ACL for LKE, is an irreversible change. Once upgraded, you can only toggle access controls with this field. | [optional]
**revision_id** | Option<**String**> | Enables clients to track events related to ACL update requests and enforcements. Optional field. If omitted, defaults to a randomly generated string. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


