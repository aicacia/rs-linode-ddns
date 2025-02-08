# PostFirewallsRequestDevices

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**linodes** | Option<**Vec<i32>**> | An array of Linode IDs. A Firewall Device is created for each ID. | [optional]
**nodebalancers** | Option<**Vec<i32>**> | An array containing a NodeBalancer ID. A Firewall Device is created for the ID.  - A NodeBalancer can have only one Firewall assigned to it. - Firewalls only apply to inbound TCP traffic to NodeBalancers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


