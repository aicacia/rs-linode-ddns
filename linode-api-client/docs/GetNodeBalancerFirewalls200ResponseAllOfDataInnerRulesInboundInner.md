# GetNodeBalancerFirewalls200ResponseAllOfDataInnerRulesInboundInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | Controls whether traffic is accepted or dropped by this rule. Overrides the Firewall's `inbound_policy` if this is an inbound rule, or the `outbound_policy` if this is an outbound rule. | [optional]
**addresses** | Option<[**models::GetNodeBalancerFirewalls200ResponseAllOfDataInnerRulesInboundInnerAddresses**](get_node_balancer_firewalls_200_response_allOf_data_inner_rules_inbound_inner_addresses.md)> |  | [optional]
**description** | Option<**String**> | Used to describe this rule. For display purposes only. | [optional]
**label** | Option<**String**> | Used to identify this rule. For display purposes only. | [optional]
**ports** | Option<**String**> | A string representing the port or ports affected by this rule:  - The string may be a single port, a range of ports, or a comma-separated list of single ports and port ranges. A space is permitted following each comma. - A range of ports is inclusive of the start and end values for the range. The end value of the range must be greater than the start value. - Ports must be within 1 and 65535, and may not contain any leading zeroes. For example, port `080` is not allowed. - The ports string can have up to 15 _pieces_, where a single port is treated as one piece, and a port range is treated as two pieces. For example, the string \"22-24, 80, 443\" has four pieces. - If no ports are configured, all ports are affected. - Only allowed for the TCP and UDP protocols. Ports are not allowed for the ICMP and IPENCAP protocols. | [optional]
**protocol** | Option<**String**> | The type of network traffic affected by this rule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


