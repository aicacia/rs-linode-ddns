# PostLinodeConfigInterfacesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ids** | **Vec<i32>** | An ordered array of existing Configuration Profile Interface `id`s.  - All current Interface `id`s must be present in the array. - If the Configuration Profile contains Interfaces and is active on the Linode, the Linode must first be shut down prior to running this operation. - Reordering takes effect after rebooting the Linode with this Configuration Profile.  The position in the array determines which of the Linode's network Interfaces is configured:  - First [0]:  eth0 - Second [1]: eth1 - Third [2]:  eth2 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


