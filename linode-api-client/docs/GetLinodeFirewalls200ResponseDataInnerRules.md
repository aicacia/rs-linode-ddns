# GetLinodeFirewalls200ResponseDataInnerRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fingerprint** | Option<**String**> | __Read-only__ The fingerprint is a 32-bit hash. It represents the firewall rules as an 8 character hex string. You can use `fingerprint` to compare rule versions. | [optional][readonly]
**inbound** | Option<[**Vec<models::GetLinodeFirewalls200ResponseDataInnerRulesInboundInner>**](get_linode_firewalls_200_response_data_inner_rules_inbound_inner.md)> | The inbound rules for the firewall, as a JSON array. | [optional]
**inbound_policy** | Option<**String**> | The default behavior for inbound traffic. This setting can be overridden by [updating](https://techdocs.akamai.com/linode-api/reference/put-firewall-rules) the `inbound.action` property of the Firewall Rule. | [optional]
**outbound** | Option<[**Vec<models::GetLinodeFirewalls200ResponseDataInnerRulesOutboundInner>**](get_linode_firewalls_200_response_data_inner_rules_outbound_inner.md)> | The outbound rules for the firewall, as a JSON array. | [optional]
**outbound_policy** | Option<**String**> | The default behavior for outbound traffic. This setting can be overridden by [updating](https://techdocs.akamai.com/linode-api/reference/put-firewall-rules) the `outbound.action` property of the Firewall Rule. | [optional]
**version** | Option<**i32**> | __Read-only__ The firewall's rule version. The first version is `1`. The version number is incremented when the firewall's rules change. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


