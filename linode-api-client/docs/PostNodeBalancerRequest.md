# PostNodeBalancerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_conn_throttle** | Option<**i32**> | Throttle TCP connections per second for TCP, HTTP, and HTTPS configurations.  Set to `0` (zero) to disable throttling. | [optional]
**configs** | Option<[**Vec<models::PostNodeBalancerRequestConfigsInner>**](post_node_balancer_request_configs_inner.md)> | The port configs to create for this NodeBalancer. Each config needs a unique port and at least one node. | [optional]
**firewall_id** | Option<**i32**> | The ID of the Firewall to assign to the NodeBalancer.  - A NodeBalancer can have only one Firewall assigned to it. - Firewalls control inbound network traffic to NodeBalancers. | [optional]
**label** | Option<**String**> | __Filterable__ This NodeBalancer's label. These must be unique on your Account. | [optional]
**region** | **String** | The ID of the Region to create this NodeBalancer in. | 
**tags** | Option<**Vec<String>**> | An array of Tags applied to this object. Tags are for organizational purposes only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


