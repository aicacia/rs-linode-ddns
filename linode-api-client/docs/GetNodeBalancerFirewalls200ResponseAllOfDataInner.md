# GetNodeBalancerFirewalls200ResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Filterable__, __Read-only__ When this Firewall was created. | [optional][readonly]
**id** | Option<**i32**> | __Filterable__, __Read-only__ The Firewall's unique ID. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ The Firewall's label, for display purposes only.  Firewall labels have the following constraints:    - Must begin and end with an alphanumeric character.   - May only consist of alphanumeric characters, hyphens (`-`), underscores (`_`) or periods (`.`).   - Cannot have two hyphens (`--`), underscores (`__`) or periods (`..`) in a row.   - Must be between 3 and 32 characters.   - Must be unique. | [optional]
**rules** | Option<[**models::GetNodeBalancerFirewalls200ResponseAllOfDataInnerRules**](get_node_balancer_firewalls_200_response_allOf_data_inner_rules.md)> |  | [optional]
**status** | Option<**String**> | __Read-only__ The status of this Firewall.    - When a Firewall is first created its status is `enabled`.   - Run the [Update a firewall](https://techdocs.akamai.com/linode-api/reference/put-firewall) operation to set a Firewall's status to `enabled` or `disabled`.   - Run the [Delete a firewall](https://techdocs.akamai.com/linode-api/reference/delete-firewall) operation to delete a Firewall. | [optional][readonly]
**tags** | Option<**Vec<String>**> | __Filterable__ An array of tags applied to this object. Tags are for organizational purposes only. | [optional]
**updated** | Option<**String**> | __Filterable__, __Read-only__ When this Firewall was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


