# PutLkeNodePoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaler** | Option<[**models::PostLkeClusterRequestNodePoolsInnerAutoscaler**](post_lke_cluster_request_node_pools_inner_autoscaler.md)> |  | [optional]
**count** | Option<**i32**> | The number of nodes in the Node Pool. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Key-value pairs added as labels to nodes in the node pool. Labels help classify your nodes and easily select subsets of objects. To learn more, review [Add Labels and Taints to your LKE node pools](https://www.linode.com/docs/products/compute/kubernetes/guides/deploy-and-manage-cluster-with-the-linode-api/#add-labels-and-taints-to-your-lke-node-pools).  **Label key:**  - A key can contain alphanumeric characters, dashes (`-`), underscores (`_`), or dots (`.`). Start and end it with an alphanumeric character.  - If the key begins with a DNS subdomain prefix followed by a single slash, for example `example.com/my-app`, the maximum total length of the label key is 128 characters. Domain labels can be up to 62 characters after the '/'. The prefix needs to adhere to [RFC 1123](https://datatracker.ietf.org/doc/html/rfc1123) DNS subdomain restrictions.  - If the key doesn't begin with a DNS subdomain prefix, the maximum key length is 63 characters.  Specifying an empty object removes all previously set labels.  **Label value:**  - The label's value can contain alphanumeric characters, dashes (`-`), underscores (`_`), or dots (`.`). Start and end it with an alphanumeric character.  - Can be specified as an empty string value with `\"\"`. | [optional]
**taints** | Option<[**Vec<models::PostLkeClusterRequestNodePoolsInnerTaintsInner>**](post_lke_cluster_request_node_pools_inner_taints_inner.md)> | Kubernetes taints to add to node pool nodes. Taints help control how pods are scheduled onto nodes, specifically allowing them to repel certain pods. To learn more, review [Add labels and taints to your LKE node pools](https://www.linode.com/docs/products/compute/kubernetes/guides/deploy-and-manage-cluster-with-the-linode-api/#add-labels-and-taints-to-your-lke-node-pools).  Specifying an empty array (`[]`) removes all previously set taints. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


