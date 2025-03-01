# PostRebuildNodeBalancerConfigRequestAllOfNodesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | The private IP Address where this backend can be reached. This _must_ be a private IP address. | [optional]
**id** | Option<**i32**> | The unique ID of the Node to update. | [optional]
**label** | Option<**String**> | The label for this node.  This is for display purposes only. | [optional]
**mode** | Option<**String**> | The mode this NodeBalancer should use when sending traffic to this backend.  - If set to `accept` this backend is accepting traffic. - If set to `reject` this backend will not receive traffic. - If set to `drain` this backend will not receive _new_ traffic, but connections already pinned to it will continue to be routed to it. - If set to `backup`, this backend will only receive traffic if all `accept` nodes are down. | [optional]
**weight** | Option<**i32**> | Used when picking a backend to serve a request and is not pinned to a single backend yet.  Nodes with a higher weight will receive more traffic. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


