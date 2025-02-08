# PutDatabasesMysqlInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_list** | Option<**Vec<String>**> | Controls access to the Managed Database.  - Individually included IP addresses or CIDR ranges can access the Managed Database while all other sources are blocked.  - A standalone value of `0.0.0.0/0` allows all IP addresses access to the Managed Database.  - An empty array (`[]`) blocks all public and private connections to the Managed Database. | [optional]
**label** | Option<**String**> | __Filterable__ A unique, user-defined string referring to the Managed Database. This string needs to be unique per Managed Database engine type. | [optional]
**r#type** | Option<**String**> | Request re-sizing of your cluster to a Linode Type with more disk space. For example, you could request a Linode Type that uses a higher plan.  - Needs to be a Linode Type with more disk space than your current Linode.  - Resizing to a larger Linode Type can accrue additional cost. Review the `price` output in the [List types](https://techdocs.akamai.com/linode-api/reference/get-linode-types) operation for more information.  - You can't update the `allow_list` and set a new `type` in the same request.  - Any active updates to your cluster need to complete before you can request a resize. The reverse is also true: An active resizing needs to complete before you can perform any other update. | [optional]
**updates** | Option<[**models::GetDatabasesInstances200ResponseAllOfDataInnerUpdates**](get_databases_instances_200_response_allOf_data_inner_updates.md)> |  | [optional]
**version** | Option<**String**> | __Filterable__ The Managed Database engine version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


