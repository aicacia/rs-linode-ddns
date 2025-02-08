# LinodePlacementGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The placement group's ID. You need to provide it for all operations that affect it. | [optional]
**label** | Option<**String**> | __Filterable__ The unique name set for the placement group. A label has these constraints:  - It needs to begin and end with an alphanumeric character. - It can only consist of alphanumeric characters, hyphens (`-`), underscores (`_`) or periods (`.`). | [optional]
**migrating_to** | Option<**String**> | __Read-only__ The unique identifier for the [placement group](https://techdocs.akamai.com/cloud-computing/docs/work-with-placement-groups) to which this Linode is being migrated. Displayed as `null` if the Linode is not being migrated to a new placement group. | [optional][readonly]
**placement_group_policy** | Option<**String**> | How requests to add future compute instances to your placement group are handled, and whether it remains compliant:  - `strict`. Don't assign a new compute instance if it breaks the grouped-together or spread-apart model set by the `placement_group_type`. Use this to ensure the placement group stays compliant (`is_compliant: true`). - `flexible`. Assign a new compute instance, even if it breaks the grouped-together or spread-apart model set by the `placement_group_type`. This makes the group non-compliant (`is_compliant: false`). You need to wait for Akamai to move the offending compute instance to make it compliant again, once the necessary capacity is available in the region. Offers flexibility to add future compute instances if compliance isn't an immediate concern.  <<LB>>  > 📘 > > In rare cases, non-compliance can occur with a `strict` placement group if Akamai needs to failover or migrate your compute instances for maintenance. Fixing non-compliance for a `strict` placement group is prioritized over a `flexible` group. | [optional]
**placement_group_type** | Option<**String**> | __Filterable__, __Read-only__ How compute instances are distributed in your placement group. A `placement_group_type` using anti-affinity (`anti-affinity:local`) places compute instances in separate hosts, but still in the same region. This best supports the spread-apart model for high availability. A `placement_group_type` using affinity places compute instances physically close together, possibly on the same host. This supports the grouped-together model for low-latency.  > 📘 > > Currently, only `anti_affinity:local` is available for `placement_group_type`. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


