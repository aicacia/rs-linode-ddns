# \ConfigurationsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_linode_config**](ConfigurationsApi.md#delete_linode_config) | **DELETE** /{apiVersion}/linode/instances/{linodeId}/configs/{configId} | Delete a configuration profile
[**delete_node_balancer_config**](ConfigurationsApi.md#delete_node_balancer_config) | **DELETE** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId} | Delete a config
[**get_linode_config**](ConfigurationsApi.md#get_linode_config) | **GET** /{apiVersion}/linode/instances/{linodeId}/configs/{configId} | Get a configuration profile
[**get_linode_configs**](ConfigurationsApi.md#get_linode_configs) | **GET** /{apiVersion}/linode/instances/{linodeId}/configs | List configuration profiles
[**get_node_balancer_config**](ConfigurationsApi.md#get_node_balancer_config) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId} | Get a config
[**get_node_balancer_configs**](ConfigurationsApi.md#get_node_balancer_configs) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs | List configs
[**post_add_linode_config**](ConfigurationsApi.md#post_add_linode_config) | **POST** /{apiVersion}/linode/instances/{linodeId}/configs | Create a configuration profile
[**post_node_balancer_config**](ConfigurationsApi.md#post_node_balancer_config) | **POST** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs | Create a config
[**post_rebuild_node_balancer_config**](ConfigurationsApi.md#post_rebuild_node_balancer_config) | **POST** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/rebuild | Rebuild a config
[**put_linode_config**](ConfigurationsApi.md#put_linode_config) | **PUT** /{apiVersion}/linode/instances/{linodeId}/configs/{configId} | Update a configuration profile
[**put_node_balancer_config**](ConfigurationsApi.md#put_node_balancer_config) | **PUT** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId} | Update a config



## delete_linode_config

> serde_json::Value delete_linode_config(api_version, linode_id, config_id)
Delete a configuration profile

Deletes the specified configuration profile from the specified Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-delete 123 23456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node_balancer_config

> serde_json::Value delete_node_balancer_config(api_version, node_balancer_id, config_id)
Delete a config

Deletes the Config for a port of this NodeBalancer.  __This cannot be undone.__  Once completed, this NodeBalancer will no longer respond to requests on the given port. This also deletes all associated NodeBalancerNodes, but the Linodes they were routing traffic to will be unchanged and will not be removed.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers config-delete \\   12345 4567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_config

> models::GetLinodeConfig200Response get_linode_config(api_version, linode_id, config_id)
Get a configuration profile

Returns information about a specific configuration profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-view 123 23456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |

### Return type

[**models::GetLinodeConfig200Response**](get_linode_config_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_configs

> models::GetLinodeConfigs200Response get_linode_configs(api_version, linode_id, page, page_size)
List configuration profiles

Lists configuration profiles associated with a Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes configs-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up Configuration profiles for. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLinodeConfigs200Response**](get_linode_configs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer_config

> models::GetNodeBalancerConfigs200ResponseDataInner get_node_balancer_config(api_version, node_balancer_id, config_id)
Get a config

Returns configuration information for a single port of this NodeBalancer.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers config-view \\   12345 4567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |

### Return type

[**models::GetNodeBalancerConfigs200ResponseDataInner**](get_node_balancer_configs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer_configs

> models::GetNodeBalancerConfigs200Response get_node_balancer_configs(api_version, node_balancer_id, page, page_size)
List configs

Returns a paginated list of NodeBalancer Configs associated with this NodeBalancer. NodeBalancer Configs represent individual ports that this NodeBalancer will accept traffic on, one Config per port.  For example, if you wanted to accept standard HTTP traffic, you would need a Config listening on port 80.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers configs-list 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetNodeBalancerConfigs200Response**](get_node_balancer_configs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_add_linode_config

> models::PostAddLinodeConfig200Response post_add_linode_config(api_version, linode_id, post_add_linode_config_request)
Create a configuration profile

Adds a new configuration profile to a Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-create 7590910 \\   --label \"My Config\" \\   --devices.sda.disk_id 123456 \\   --devices.sdb.disk_id 123457     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up Configuration profiles for. | [required] |
**post_add_linode_config_request** | [**PostAddLinodeConfigRequest**](PostAddLinodeConfigRequest.md) | The parameters to set when creating the configuration profile. This determines things like the kernel, devices, and how much memory a Linode boots with. | [required] |

### Return type

[**models::PostAddLinodeConfig200Response**](post_add_linode_config_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_node_balancer_config

> models::GetNodeBalancerConfigs200ResponseDataInner post_node_balancer_config(api_version, node_balancer_id, get_node_balancer_configs200_response_data_inner)
Create a config

Creates a NodeBalancer configuration, which allows the NodeBalancer to accept traffic on a new port. You will need to add NodeBalancer nodes to the new configuration before it can actually serve requests.   <<LB>>  ---   - __CLI: HTTPS__.      ```     linode-cli nodebalancers config-create 12345 \\   --port 443 \\   --protocol https \\   --algorithm roundrobin \\   --stickiness http_cookie \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\   --check_passive true \\   --proxy_protocol \"none\" \\   --ssl_cert \"-----BEGIN CERTIFICATE-----               CERTIFICATE_INFORMATION               -----END CERTIFICATE-----\" \\   --ssl_key \"-----BEGIN PRIVATE KEY-----              PRIVATE_KEY_INFORMATION              -----END PRIVATE KEY-----\" \\   --cipher_suite recommended \\     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __CLI: TCP__.      ```     linode-cli nodebalancers config-create 12345 \\   --port 80 \\   --protocol tcp \\   --algorithm roundrobin \\   --stickiness none \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\   --proxy_protocol \"v2\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __CLI: HTTP__.      ```     linode-cli nodebalancers config-create 12345 \\   --port 440 \\   --protocol http \\   --algorithm roundrobin \\   --stickiness http_cookie \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**get_node_balancer_configs200_response_data_inner** | Option<[**GetNodeBalancerConfigs200ResponseDataInner**](GetNodeBalancerConfigs200ResponseDataInner.md)> | NodeBalancer configuration details for the port based on the routing protocol. |  |

### Return type

[**models::GetNodeBalancerConfigs200ResponseDataInner**](get_node_balancer_configs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_rebuild_node_balancer_config

> models::GetNodeBalancerConfigs200ResponseDataInner post_rebuild_node_balancer_config(api_version, node_balancer_id, config_id, post_rebuild_node_balancer_config_request)
Rebuild a config

Rebuilds a NodeBalancer Config and its Nodes that you have permission to modify.  Use this operation to update a NodeBalancer's Config and Nodes with a single request.   <<LB>>  ---   - __CLI: HTTPS__.      ```     linode-cli nodebalancers config-rebuild \\   12345 4567 \\   --port 443 \\   --protocol https \\   --algorithm roundrobin \\   --stickiness http_cookie \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\   --check_passive true \\   --proxy_protocol \"none\" \\   --ssl_cert \"-----BEGIN CERTIFICATE-----               CERTIFICATE_INFORMATION               -----END CERTIFICATE-----\" \\   --ssl_key \"-----BEGIN PRIVATE KEY-----              PRIVATE_KEY_INFORMATION              -----END PRIVATE KEY-----\" \\   --cipher_suite recommended \\   --nodes.label \"node1\" --nodes.address \"192.168.210.120:80\" --nodes.mode \"accept\" --nodes.weight 50 \\   --nodes '[{\"address\":\"192.168.210.122:80\",\"label\":\"node2\",\"weight\":50,\"mode\":\"accept\"}]'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __CLI: TCP__.      ```     linode-cli nodebalancers config-rebuild \\   12345 4567 \\   --port 80 \\   --protocol tcp \\   --algorithm roundrobin \\   --stickiness none \\   --proxy_protocol \"v2\"   --nodes.label \"node1\" --nodes.address \"192.168.210.120:80\" --nodes.mode \"accept\" --nodes.weight 50 \\   --nodes '[{\"address\":\"192.168.210.122:80\",\"label\":\"node2\",\"weight\":50,\"mode\":\"accept\"}]'     ```      [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)  - __CLI: HTTP__.      ```     linode-cli nodebalancers config-rebuild \\   12345 4567 \\   --port 440 \\   --protocol http \\   --algorithm roundrobin \\   --stickiness none \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\   --nodes.label \"node1\" --nodes.address \"192.168.210.120:80\" --nodes.mode \"accept\" --nodes.weight 50 \\   --nodes '[{\"address\":\"192.168.210.122:80\",\"label\":\"node2\",\"weight\":50,\"mode\":\"accept\"}]'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |
**post_rebuild_node_balancer_config_request** | [**PostRebuildNodeBalancerConfigRequest**](PostRebuildNodeBalancerConfigRequest.md) | Information about the NodeBalancer Config to rebuild. | [required] |

### Return type

[**models::GetNodeBalancerConfigs200ResponseDataInner**](get_node_balancer_configs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_linode_config

> models::PutLinodeConfig200Response put_linode_config(api_version, linode_id, config_id, put_linode_config_request)
Update a configuration profile

Updates a configuration profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-update 123 23456 \\   --kernel \"linode/latest-64bit\" \\   --comments \"This is my main Config\" \\   --memory_limit 2048 \\   --run_level default \\   --virt_mode paravirt \\   --helpers.updatedb_disabled true \\   --helpers.distro true \\   --helpers.modules_dep true \\   --helpers.network true \\   --helpers.devtmpfs_automount false \\   --label \"My Config\" \\   --devices.sda.disk_id 123456 \\   --devices.sdb.disk_id 123457     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |
**put_linode_config_request** | [**PutLinodeConfigRequest**](PutLinodeConfigRequest.md) |  | [required] |

### Return type

[**models::PutLinodeConfig200Response**](put_linode_config_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_node_balancer_config

> models::GetNodeBalancerConfigs200ResponseDataInner put_node_balancer_config(api_version, node_balancer_id, config_id, get_node_balancer_configs200_response_data_inner)
Update a config

Updates the configuration for a single port on a NodeBalancer.   <<LB>>  ---   - __CLI: HTTPS__.      ```     linode-cli nodebalancers config-update \\   12345 4567 \\   --port 443 \\   --protocol https \\   --algorithm roundrobin \\   --stickiness http_cookie \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\   --check_passive true \\   --proxy_protocol \"none\" \\   --ssl_cert \"-----BEGIN CERTIFICATE-----               CERTIFICATE_INFORMATION               -----END CERTIFICATE-----\" \\   --ssl_key \"-----BEGIN PRIVATE KEY-----              PRIVATE_KEY_INFORMATION              -----END PRIVATE KEY-----\" \\   --cipher_suite recommended     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __CLI: TCP__.      ```     linode-cli nodebalancers config-update \\   12345 4567 \\   --port 80 \\   --protocol tcp \\   --algorithm roundrobin \\   --stickiness none \\   --stickiness http_cookie \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\   --proxy_protocol \"v2\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __CLI: HTTP__.      ```     linode-cli nodebalancers config-update \\   12345 4567 \\   --port 440 \\   --protocol http \\   --algorithm roundrobin \\   --stickiness http_cookie \\   --check http_body \\   --check_interval 90 \\   --check_timeout 10 \\   --check_attempts 3 \\   --check_path \"/test\" \\   --check_body \"it works\" \\     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |
**get_node_balancer_configs200_response_data_inner** | [**GetNodeBalancerConfigs200ResponseDataInner**](GetNodeBalancerConfigs200ResponseDataInner.md) | The fields to update. | [required] |

### Return type

[**models::GetNodeBalancerConfigs200ResponseDataInner**](get_node_balancer_configs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

