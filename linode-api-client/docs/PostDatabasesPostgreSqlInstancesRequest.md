# PostDatabasesPostgreSqlInstancesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_list** | Option<**Vec<String>**> | Controls access to the Managed Database.  - Individually included IP addresses or CIDR ranges can access the Managed Database while all other sources are blocked.  - A standalone value of `0.0.0.0/0` allows all IP addresses access to the Managed Database.  - An empty array (`[]`) blocks all public and private connections to the Managed Database. | [optional]
**cluster_size** | Option<**i32**> | The number of Linode instance nodes deployed to the Managed Database.   - Choose `3` nodes to create a high availability cluster that consists of one primary node and two replica nodes.  - A `2` node cluster is only available with a dedicated plan. It consists of one primary node and one replica node. | [optional][default to Variant1]
**engine** | **String** | The Managed Database engine in engine/version format. | 
**fork** | Option<[**models::PostDatabasesMysqlInstancesRequestFork**](post_databases_mysql_instances_request_fork.md)> |  | [optional]
**label** | **String** | __Filterable__ A unique, user-defined string referring to the Managed Database. This string needs to be unique per Managed Database engine type. | 
**region** | **String** | __Filterable__ The [Region](https://techdocs.akamai.com/linode-api/reference/get-regions) ID for the Managed Database. | 
**ssl_connection** | Option<**bool**> | Currently required to be `true`. Whether to require SSL credentials to establish a connection to the Managed Database.  Run the [Get managed PostgreSQL database credentials](https://techdocs.akamai.com/linode-api/reference/get-databases-postgre-sql-instance-credentials) operation for access information. | [optional][default to true]
**r#type** | **String** | __Filterable__ The Linode Instance type used by the Managed Database for its nodes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


