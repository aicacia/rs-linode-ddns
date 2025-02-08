# \DatabasesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_databases_mysql_instance**](DatabasesApi.md#delete_databases_mysql_instance) | **DELETE** /{apiVersion}/databases/mysql/instances/{instanceId} | Delete a MySQL Managed Database
[**delete_databases_postgre_sql_instance**](DatabasesApi.md#delete_databases_postgre_sql_instance) | **DELETE** /{apiVersion}/databases/postgresql/instances/{instanceId} | Delete a PostgreSQL Managed Database
[**get_databases_instances**](DatabasesApi.md#get_databases_instances) | **GET** /{apiVersion}/databases/instances | List Managed Databases
[**get_databases_mysql_instance**](DatabasesApi.md#get_databases_mysql_instance) | **GET** /{apiVersion}/databases/mysql/instances/{instanceId} | Get a MySQL Managed Database
[**get_databases_mysql_instances**](DatabasesApi.md#get_databases_mysql_instances) | **GET** /{apiVersion}/databases/mysql/instances | List MySQL Managed Databases
[**get_databases_postgre_sql_instance**](DatabasesApi.md#get_databases_postgre_sql_instance) | **GET** /{apiVersion}/databases/postgresql/instances/{instanceId} | Get a PostgreSQL Managed Database
[**get_databases_postgre_sql_instances**](DatabasesApi.md#get_databases_postgre_sql_instances) | **GET** /{apiVersion}/databases/postgresql/instances | List PostgreSQL Managed Databases
[**post_databases_mysql_instance_patch**](DatabasesApi.md#post_databases_mysql_instance_patch) | **POST** /{apiVersion}/databases/mysql/instances/{instanceId}/patch | Patch a MySQL Managed Database
[**post_databases_mysql_instances**](DatabasesApi.md#post_databases_mysql_instances) | **POST** /{apiVersion}/databases/mysql/instances | Create or restore a MySQL Managed Database
[**post_databases_postgre_sql_instance_patch**](DatabasesApi.md#post_databases_postgre_sql_instance_patch) | **POST** /{apiVersion}/databases/postgresql/instances/{instanceId}/patch | Patch a PostgreSQL Managed Database
[**post_databases_postgre_sql_instances**](DatabasesApi.md#post_databases_postgre_sql_instances) | **POST** /{apiVersion}/databases/postgresql/instances | Create or restore a PostgreSQL Managed Database
[**put_databases_mysql_instance**](DatabasesApi.md#put_databases_mysql_instance) | **PUT** /{apiVersion}/databases/mysql/instances/{instanceId} | Update a MySQL Managed Database
[**put_databases_postgre_sql_instance**](DatabasesApi.md#put_databases_postgre_sql_instance) | **PUT** /{apiVersion}/databases/postgresql/instances/{instanceId} | Update a PostgreSQL Managed Database
[**resume_databases_mysql_instance**](DatabasesApi.md#resume_databases_mysql_instance) | **POST** /{apiVersion}/databases/mysql/instances/{instanceId}/resume | Resume a MySQL Managed Database
[**resume_databases_postgre_sql_instance**](DatabasesApi.md#resume_databases_postgre_sql_instance) | **POST** /{apiVersion}/databases/postgresql/instances/{instanceId}/resume | Resume a PostgreSQL Managed Database
[**suspend_databases_mysql_instance**](DatabasesApi.md#suspend_databases_mysql_instance) | **POST** /{apiVersion}/databases/mysql/instances/{instanceId}/suspend | Suspend a MySQL Managed Database
[**suspend_databases_postgre_sql_instance**](DatabasesApi.md#suspend_databases_postgre_sql_instance) | **POST** /{apiVersion}/databases/postgresql/instances/{instanceId}/suspend | Suspend a PostgreSQL Managed Database



## delete_databases_mysql_instance

> serde_json::Value delete_databases_mysql_instance(api_version, instance_id)
Delete a MySQL Managed Database

Remove a MySQL Managed Database from your account.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status can be `active`, `failed`, or `degraded`.  - Only unrestricted users can access this operation. They have access regardless of the acting token's OAuth scopes.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_databases_postgre_sql_instance

> serde_json::Value delete_databases_postgre_sql_instance(api_version, instance_id)
Delete a PostgreSQL Managed Database

Remove a PostgreSQL Managed Database from your account.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status can be `active`, `failed`, or `degraded`.  - Only unrestricted users can access this operation. They have access regardless of the acting token's OAuth scopes.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_instances

> models::GetDatabasesInstances200Response get_databases_instances(api_version, page, page_size)
List Managed Databases

Display all Managed Databases accessible to your user, regardless of engine type. For more detailed information on a particular database instance, make a request to its `instance_uri`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesInstances200Response**](get_databases_instances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_mysql_instance

> models::GetDatabasesMysqlInstance200Response get_databases_mysql_instance(api_version, instance_id)
Get a MySQL Managed Database

Display information for a single, accessible MySQL Managed Database.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**models::GetDatabasesMysqlInstance200Response**](get_databases_mysql_instance_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_mysql_instances

> models::GetDatabasesMysqlInstances200Response get_databases_mysql_instances(api_version, page, page_size)
List MySQL Managed Databases

Display all accessible MySQL Managed Databases.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesMysqlInstances200Response**](get_databases_mysql_instances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_postgre_sql_instance

> models::GetDatabasesPostgreSqlInstance200Response get_databases_postgre_sql_instance(api_version, instance_id)
Get a PostgreSQL Managed Database

Display information for a single, accessible PostgreSQL Managed Database.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**models::GetDatabasesPostgreSqlInstance200Response**](get_databases_postgre_sql_instance_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_postgre_sql_instances

> models::GetDatabasesPostgreSqlInstances200Response get_databases_postgre_sql_instances(api_version, page, page_size)
List PostgreSQL Managed Databases

Display all accessible PostgreSQL Managed Databases.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesPostgreSqlInstances200Response**](get_databases_postgre_sql_instances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_databases_mysql_instance_patch

> serde_json::Value post_databases_mysql_instance_patch(api_version, instance_id)
Patch a MySQL Managed Database

Apply security patches and updates to the underlying operating system of the MySQL Managed Database. This function runs during regular maintenance windows, which you can configure with the [Update a managed MySQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-mysql-instance) operation.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status meeds to be `active`.  - If your database cluster is configured with a single node, downtime occurs during maintenance updates. Consider upgrading to a [high availability](https://techdocs.akamai.com/cloud-computing/docs/managed-databases#high-availability) plan to avoid any maintenance downtime.  - Major upgrades are optional until the service reaches end of service, and can be done in place.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-patch 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_databases_mysql_instances

> models::PostDatabasesMysqlInstances200Response post_databases_mysql_instances(api_version, post_databases_mysql_instances_request)
Create or restore a MySQL Managed Database

**Provision a MySQL Managed Database**  Use this operation to create a new MySQL Managed Database.  - Restricted users need the `add_databases` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants).  - New instances can take 10 to 15 minutes to deploy.  - All Managed Databases include automatic, daily backups. Up to seven backups are automatically stored for each Managed Database, providing restore points for each day of the past week.  - All Managed Databases include automatic updates, which apply security patches to the underlying operating system of the MySQL Managed Database. Configure the maintenance window for these updates with the [Update a managed MySQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-mysql-instance) operation.  - If your database cluster is configured with a single node, downtime occurs during maintenance updates. You should adjust the window to match a time that's the least disruptive to your application and users. Also consider upgrading to a [high availability](https://techdocs.akamai.com/cloud-computing/docs/managed-databases#high-availability) plan to avoid any maintenance downtime.  - Major upgrades are optional until the service reaches end of service, and can be done in place.  **Restore a MySQL Managed Database**  Do this by creating a `fork` from a backup. A user needs `read_write` access to the database and its status can be `active`, `degraded`, or `failed`.  > 📘 > > Restoring from a backup creates a second running cluster, which incurs billing. Delete the first cluster after the restore is complete, to avoid this billing.   <<LB>>  ---   - __CLI for create operation__.      ```     linode-cli databases mysql-create \\   --label example-db1 \\   --region us-east \\   --type g6-dedicated-2 \\   --cluster_size 3 \\   --engine mysql/8.0.26 \\   --ssl_connection true \\   --allow_list 203.0.113.1 \\   --allow_list 192.0.1.0/24     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_databases_mysql_instances_request** | [**PostDatabasesMysqlInstancesRequest**](PostDatabasesMysqlInstancesRequest.md) |  | [required] |

### Return type

[**models::PostDatabasesMysqlInstances200Response**](post_databases_mysql_instances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_databases_postgre_sql_instance_patch

> serde_json::Value post_databases_postgre_sql_instance_patch(api_version, instance_id)
Patch a PostgreSQL Managed Database

Apply security patches and updates to the underlying operating system of the PostgreSQL Managed Database. This function runs during regular maintenance windows, which you can configure with the [Update a managed PostgreSQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-postgre-sql-instance) operation.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `active`.  - If your database cluster is configured with a single node, downtime occurs during maintenance updates. Consider upgrading to a [high availability](https://techdocs.akamai.com/cloud-computing/docs/managed-databases#high-availability) plan to avoid any maintenance downtime.  - Major upgrades are optional until the service reaches end of service, and can be done in place.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-patch 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_databases_postgre_sql_instances

> models::PostDatabasesPostgreSqlInstances200Response post_databases_postgre_sql_instances(api_version, post_databases_postgre_sql_instances_request)
Create or restore a PostgreSQL Managed Database

**Provision a PostgreSQL Managed Database**  Use this operation to create a new PostgreSQL Managed Database.  - Restricted users need the `add_databases` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants).  - New instances can take 10 to 15 minutes to deploy.  - All Managed Databases include automatic, daily backups. Up to seven backups are automatically stored for each Managed Database, providing restore points for each day of the past week.  - All Managed Databases include automatic updates, which apply security patches to the underlying operating system of the PostgreSQL Managed Database. Configure the maintenance window for these updates with the [Update a managed PostgreSQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-postgre-sql-instance) operation.  - If your database cluster is configured with a single node, downtime occurs during maintenance updates. Adjust the window to match a time that's the least disruptive to your application and users. Also consider upgrading to a [high availability](https://techdocs.akamai.com/cloud-computing/docs/managed-databases#high-availability) plan to avoid any maintenance downtime.  - Major upgrades are optional until the service reaches end of service, and can be done in place.  **Restore a PostgreSQL Managed Database**  Do this by creating a `fork` from a backup. A user needs `read_write` access to the database, and its status can be `active`, `degraded`, or `failed`.  > 📘 > > Restoring from a backup creates a second running cluster, which incurs billing. Delete the first cluster after the restore is complete, to avoid this billing.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-create \\   --label example-db \\   --region us-east \\   --type g6-dedicated-2 \\   --cluster_size 3 \\   --engine postgresql/13.2 \\   --ssl_connection true \\   --allow_list 203.0.113.1 \\   --allow_list 192.0.1.0/24     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_databases_postgre_sql_instances_request** | [**PostDatabasesPostgreSqlInstancesRequest**](PostDatabasesPostgreSqlInstancesRequest.md) |  | [required] |

### Return type

[**models::PostDatabasesPostgreSqlInstances200Response**](post_databases_postgre_sql_instances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_databases_mysql_instance

> models::PutDatabasesMysqlInstance200Response put_databases_mysql_instance(api_version, instance_id, put_databases_mysql_instance_request)
Update a MySQL Managed Database

Make changes to an existing MySQL Managed Database.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `active`.  - New values set in the `allow_list` overwrite existing values. To keep existing values, run the [List MySQL Managed Databases](https://techdocs.akamai.com/linode-api/reference/get-databases-mysql-instances) operation, store the `allow_list` addresses from the response, and include them with any new addresses in this operation.  - Updates to your `allow_list` may take a short time to complete, making this operation inappropriate for rapid successive updates.  - Also allows resizing the database cluster to a larger one. Clusters can't be resized to smaller plans.  - All Managed Databases include automatic updates, which apply security patches to the underlying operating system of the Managed MySQL Database. Use the `updates` object in this operation to modify the maintenance window for these updates.  - If your database cluster is configured with a single node, downtime occurs during maintenance updates. Use the `updates` object to adjust the window to match a time that's the least disruptive to your application and users. Also consider upgrading to a [high availability](https://techdocs.akamai.com/cloud-computing/docs/managed-databases#high-availability) plan to avoid any maintenance downtime.  - Major upgrades are optional until the service reaches end of service, and can be done in place.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-update 123 \\   --label example-db \\   --allow_list 203.0.113.1 \\   --allow_list 192.0.1.0/24 \\   --type g6-standard-1 \\   --updates.frequency weekly \\   --updates.duration 3 \\   --updates.hour_of_day 12 \\   --updates.day_of_week 4 \\     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |
**put_databases_mysql_instance_request** | [**PutDatabasesMysqlInstanceRequest**](PutDatabasesMysqlInstanceRequest.md) |  | [required] |

### Return type

[**models::PutDatabasesMysqlInstance200Response**](put_databases_mysql_instance_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_databases_postgre_sql_instance

> models::PutDatabasesPostgreSqlInstance200Response put_databases_postgre_sql_instance(api_version, instance_id, put_databases_postgre_sql_instance_request)
Update a PostgreSQL Managed Database

Make changes to an existing PostgreSQL Managed Database.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `active`.  - New values set in the `allow_list` overwrite existing values. To keep existing values, run the [List PostgreSQL Managed Databases](https://techdocs.akamai.com/linode-api/reference/get-databases-postgre-sql-instances) operation, store the `allow_list` addresses from the response, and include them with any new addresses in this operation.  - Updates to your `allow_list` may take a short period of time to complete, making this operation inappropriate for rapid successive updates.  - Also allows resizing the database cluster to a larger one. Clusters can't be resized to smaller plans.  - All Managed Databases include automatic updates, which apply security patches to the underlying operating system of the Managed PostgreSQL Database. Use the `updates` object in this operation to modify the maintenance window for these updates.  - If your database cluster is configured with a single node, downtime occurs during maintenance updates. Use the `updates` object to adjust the window to match a time that's the least disruptive to your application and users. Also consider upgrading to a [high availability](https://techdocs.akamai.com/cloud-computing/docs/managed-databases#high-availability) plan to avoid any maintenance downtime.  - Major upgrades are optional until the service reaches end of service, and can be done in place.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-update 123 \\   --label example-db \\   --allow_list 203.0.113.1 \\   --allow_list 192.0.1.0/24 \\   --type g6-standard-1 \\   --updates.frequency weekly \\   --updates.duration 3 \\   --updates.hour_of_day 12 \\   --updates.day_of_week 4 \\     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |
**put_databases_postgre_sql_instance_request** | [**PutDatabasesPostgreSqlInstanceRequest**](PutDatabasesPostgreSqlInstanceRequest.md) |  | [required] |

### Return type

[**models::PutDatabasesPostgreSqlInstance200Response**](put_databases_postgre_sql_instance_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_databases_mysql_instance

> serde_json::Value resume_databases_mysql_instance(api_version, instance_id)
Resume a MySQL Managed Database

Resume a suspended MySQL Managed Database from your account. This resumes billing for the cluster.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `suspended`.   <<LB>>  ---   - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call the `v4` URL. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_databases_postgre_sql_instance

> serde_json::Value resume_databases_postgre_sql_instance(api_version, instance_id)
Resume a PostgreSQL Managed Database

Resume a suspended PostgreSQL Managed Database from your account. This resumes billing for the cluster.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `suspended`.   <<LB>>  ---   - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call the `v4` URL. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suspend_databases_mysql_instance

> serde_json::Value suspend_databases_mysql_instance(api_version, instance_id)
Suspend a MySQL Managed Database

Suspend a MySQL Managed Database from your account, releasing idle resources and keeping only necessary data. All service data is lost if there are no backups available. This halts billing for the cluster.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `active`.  - Akamai deletes suspended clusters after 180 days.   <<LB>>  ---   - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call the `v4` URL. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suspend_databases_postgre_sql_instance

> serde_json::Value suspend_databases_postgre_sql_instance(api_version, instance_id)
Suspend a PostgreSQL Managed Database

Suspend a PostgreSQL Managed Database from your account, releasing idle resources and keeping only necessary data. All service data is lost if there are no backups available. This halts billing for the cluster.  - The user needs `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) access to the database.  - The database's status needs to be `active`.  - Akamai deletes suspended clusters after 180 days.   <<LB>>  ---   - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call the `v4` URL. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

