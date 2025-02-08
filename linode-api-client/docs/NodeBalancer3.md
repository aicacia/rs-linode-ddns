# NodeBalancer3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_conn_throttle** | Option<**i32**> | Throttle TCP connections per second for TCP, HTTP, and HTTPS configurations.  Set to `0` (zero) to disable throttling. | [optional]
**created** | Option<**String**> | __Read-only__ When this NodeBalancer was created. | [optional][readonly]
**hostname** | Option<**String**> | __Read-only__ This NodeBalancer's hostname, beginning with its IP address and ending with _.ip.linodeusercontent.com_. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This NodeBalancer's unique ID. | [optional][readonly]
**ipv4** | Option<**String**> | __Filterable__, __Read-only__ This NodeBalancer's public IPv4 address. | [optional][readonly]
**ipv6** | Option<**String**> | __Read-only__ This NodeBalancer's public IPv6 address. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ This NodeBalancer's label. These must be unique on your Account. | [optional]
**region** | Option<**String**> | __Filterable__, __Read-only__ The Region where this NodeBalancer is located. NodeBalancers only support backends in the same Region. | [optional][readonly]
**tags** | Option<**Vec<String>**> | __Filterable__ An array of Tags applied to this object.  Tags are for organizational purposes only. | [optional]
**transfer** | Option<[**models::NodeBalancerTransfer**](NodeBalancer_transfer.md)> |  | [optional]
**updated** | Option<**String**> | __Read-only__ When this NodeBalancer was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


