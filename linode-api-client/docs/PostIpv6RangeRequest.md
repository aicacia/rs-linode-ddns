# PostIpv6RangeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**linode_id** | Option<**i32**> | The ID of the Linode to assign this range to. The SLAAC address for the provided Linode is used as the range's `route_target`.  - __Required__ if `route_target` is omitted from the request.  - Mutually exclusive with `route_target`. Submitting values for both properties in a request results in an error. | [optional]
**prefix_length** | **i32** | The prefix length of the IPv6 range. | 
**route_target** | Option<**String**> | The IPv6 SLAAC address to assign this range to.  - __Required__ if `linode_id` is omitted from the request.  - Mutually exclusive with `linode_id`. Submitting values for both properties in a request results in an error.  - __Note__. Omit the `/128` prefix length of the SLAAC address when using this property. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


