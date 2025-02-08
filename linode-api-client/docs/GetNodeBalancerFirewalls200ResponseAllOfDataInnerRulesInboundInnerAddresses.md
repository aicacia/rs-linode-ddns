# GetNodeBalancerFirewalls200ResponseAllOfDataInnerRulesInboundInnerAddresses

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ipv4** | Option<**Vec<String>**> | A list of IPv4 addresses or networks. Addresses must be in IP/mask format. Must not be an empty list.  If `0.0.0.0/0` is included in this list, all IPv4 addresses are affected by this rule. | [optional]
**ipv6** | Option<**Vec<String>**> | A list of IPv6 addresses or networks. Addresses must be in IP/mask format and must not include zone_id notation as described in [RFC 4007](https://www.rfc-editor.org/rfc/rfc4007). Must not be an empty list.  If `::/0` is included in this list, all IPv6 addresses are affected by this rule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


