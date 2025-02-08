# PutLkeClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**control_plane** | Option<[**models::PutLkeClusterRequestControlPlane**](put_lke_cluster_request_control_plane.md)> |  | [optional]
**k8s_version** | Option<**String**> | The desired Kubernetes version for this Kubernetes cluster in the format of &lt;major&gt;.&lt;minor&gt;. New and recycled Nodes in this cluster will be installed with the latest available patch for the Cluster's Kubernetes version.  When upgrading the Kubernetes version, only the next latest minor version following the current version can be deployed. For example, a cluster with Kubernetes version 1.29 can be upgraded to version 1.30, but not directly to 1.31.  The Kubernetes version of a cluster can not be downgraded. | [optional]
**label** | Option<**String**> | __Filterable__ This Kubernetes cluster's unique label for display purposes only. Labels have the following constraints:    - UTF-8 characters will be returned by the API using escape sequences of their Unicode code points. For example, the Japanese character _か_ is 3 bytes in UTF-8 (`0xE382AB`). Its Unicode code point is 2 bytes (`0x30AB`). APIv4 supports this character and the API will return it as the escape sequence using six 1 byte characters which represent 2 bytes of Unicode code point (`\"\\u30ab\"`).    - 4 byte UTF-8 characters are not supported.    - If the label is entirely composed of UTF-8 characters, the API response will return the code points using up to 193 1 byte characters. | [optional]
**tags** | Option<**Vec<String>**> | An array of tags applied to the Kubernetes cluster. Tags are for organizational purposes only. To delete a tag, exclude it from your `tags` array. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


