# \SslCertificatesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_databases_mysql_instance_ssl**](SslCertificatesApi.md#get_databases_mysql_instance_ssl) | **GET** /{apiVersion}/databases/mysql/instances/{instanceId}/ssl | Get a MySQL Managed Database SSL certificate
[**get_databases_postgresql_instance_ssl**](SslCertificatesApi.md#get_databases_postgresql_instance_ssl) | **GET** /{apiVersion}/databases/postgresql/instances/{instanceId}/ssl | Get a PostgreSQL Managed Database SSL certificate



## get_databases_mysql_instance_ssl

> models::GetDatabasesMysqlInstanceSsl200Response get_databases_mysql_instance_ssl(api_version, instance_id)
Get a MySQL Managed Database SSL certificate

Display the SSL CA certificate for an accessible MySQL Managed Database. The database's status needs to be `active`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-ssl-cert 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed MySQL Database. | [required] |

### Return type

[**models::GetDatabasesMysqlInstanceSsl200Response**](get_databases_mysql_instance_ssl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_postgresql_instance_ssl

> models::GetDatabasesMysqlInstanceSsl200Response get_databases_postgresql_instance_ssl(api_version, instance_id)
Get a PostgreSQL Managed Database SSL certificate

Display the SSL CA certificate for an accessible PostgreSQL Managed Database. The database's status needs to be `active`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-ssl-cert 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**models::GetDatabasesMysqlInstanceSsl200Response**](get_databases_mysql_instance_ssl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

