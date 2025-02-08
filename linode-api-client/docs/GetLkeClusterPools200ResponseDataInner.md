# GetLkeClusterPools200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaler** | Option<[**models::GetLkeClusterPools200ResponseDataInnerAutoscaler**](get_lke_cluster_pools_200_response_data_inner_autoscaler.md)> |  | [optional]
**count** | Option<**i32**> | The number of nodes in the Node Pool. | [optional]
**disk_encryption** | Option<**String**> | __Limited availability__ For new LKE node pools, `disk_encryption` is automatically `enabled` where disk encryption is supported. It can't be `disabled`. For existing LKE node pools, this derives from the Linode's `disk_encryption` setting. If a Linode's node pool is not encrypted and you want an encrypted node pool, delete the node pool and create a new node pool. | [optional]
**disks** | Option<[**Vec<models::PostLkeClusterRequestNodePoolsInnerDisksInner>**](post_lke_cluster_request_node_pools_inner_disks_inner.md)> | This Node Pool's custom disk layout. | [optional]
**id** | Option<**i32**> | __Filterable__ This Node Pool's unique ID. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Key-value pairs added as labels to nodes in the node pool. Labels help classify your nodes and easily select subsets of objects. To learn more, review [Add Labels and Taints to your LKE node pools](https://www.linode.com/docs/products/compute/kubernetes/guides/deploy-and-manage-cluster-with-the-linode-api/#add-labels-and-taints-to-your-lke-node-pools). | [optional]
**nodes** | Option<[**Vec<models::GetLkeClusterNode200Response>**](get_lke_cluster_node_200_response.md)> | Status information for the Nodes which are members of this Node Pool. If a Linode has not been provisioned for a given Node slot, the `instance_id` will be returned as `null`. | [optional]
**tags** | Option<**Vec<String>**> | __Filterable__ An array of tags applied to this object. Tags are for organizational purposes only. | [optional]
**taints** | Option<[**Vec<models::GetLkeClusterPools200ResponseDataInnerTaintsInner>**](get_lke_cluster_pools_200_response_data_inner_taints_inner.md)> | Kubernetes taints added to nodes in the node pool. Taints help control how pods are scheduled onto nodes, specifically allowing them to repel certain pods. | [optional]
**r#type** | Option<**String**> | The Linode Type for all of the nodes in the Node Pool. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


