# GetDatabasesInstances200ResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_list** | Option<**Vec<String>**> | Controls access to the Managed Database.  - Individually included IP addresses or CIDR ranges can access the Managed Database while all other sources are blocked.  - A standalone value of `0.0.0.0/0` allows all IP addresses access to the Managed Database.  - An empty array (`[]`) blocks all public and private connections to the Managed Database. | [optional]
**cluster_size** | Option<**i32**> | The number of Linode instance nodes deployed to the Managed Database.   - Choose `3` nodes to create a high availability cluster that consists of one primary node and two replica nodes.  - A `2` node cluster is only available with a dedicated plan. It consists of one primary node and one replica node. | [optional][default to Variant1]
**created** | Option<**String**> | __Read-only__ When this Managed Database was created. | [optional][readonly]
**encrypted** | Option<**bool**> | __Read-only__ Whether the Managed Databases is encrypted. Currently required to be `true`. | [optional][readonly][default to true]
**engine** | Option<**String**> | __Filterable__, __Read-only__ The Managed Database engine type. | [optional][readonly]
**fork** | Option<[**models::GetDatabasesInstances200ResponseAllOfDataInnerFork**](get_databases_instances_200_response_allOf_data_inner_fork.md)> |  | [optional]
**hosts** | Option<[**models::GetDatabasesInstances200ResponseAllOfDataInnerHosts**](get_databases_instances_200_response_allOf_data_inner_hosts.md)> |  | [optional]
**id** | Option<**i32**> | __Read-only__ A unique ID that can be used to identify and reference the Managed Database. | [optional][readonly]
**instance_uri** | Option<**String**> | __Read-only__ Append this to `https://api.linode.com` to run commands for the Managed Database. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ A unique, user-defined string referring to the Managed Database. This string needs to be unique per Managed Database engine type. | [optional]
**members** | Option<[**serde_json::Value**](.md)> | __Read-only__ A mapping between IP addresses and strings designating them as `primary` or `failover`. | [optional][readonly]
**oldest_restore_time** | Option<**String**> | __Read-only__ The oldest time to which a database can be restored. | [optional][readonly]
**platform** | Option<**String**> | __Filterable__, __Read-only__ The back-end platform for relational databases used by the service. | [optional][readonly]
**port** | Option<**i32**> | __Read-only__ The access port for this Managed Database. | [optional][readonly]
**region** | Option<**String**> | __Filterable__ The [Region](https://techdocs.akamai.com/linode-api/reference/get-regions) ID for the Managed Database. | [optional]
**status** | Option<**String**> | __Filterable__, __Read-only__ The operating status of the Managed Database. | [optional][readonly]
**total_disk_size_gb** | Option<**i32**> | __Read-only__ The total disk size of the database, in GB. | [optional][readonly]
**r#type** | Option<**String**> | __Filterable__ The Linode Instance type used by the Managed Database for its nodes. | [optional]
**updated** | Option<**String**> | __Read-only__ When this Managed Database was last updated. | [optional][readonly]
**updates** | Option<[**models::GetDatabasesInstances200ResponseAllOfDataInnerUpdates**](get_databases_instances_200_response_allOf_data_inner_updates.md)> |  | [optional]
**used_disk_size_gb** | Option<**i32**> | __Read-only__ The amount of space currently in use in the database, in GB. | [optional][readonly]
**version** | Option<**String**> | __Filterable__ The Managed Database engine version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


