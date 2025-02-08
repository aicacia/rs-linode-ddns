# \FirewallsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_firewall**](FirewallsApi.md#delete_firewall) | **DELETE** /{apiVersion}/networking/firewalls/{firewallId} | Delete a firewall
[**get_firewall**](FirewallsApi.md#get_firewall) | **GET** /{apiVersion}/networking/firewalls/{firewallId} | Get a firewall
[**get_firewall_rule_version**](FirewallsApi.md#get_firewall_rule_version) | **GET** /{apiVersion}/networking/firewalls/{firewallId}/history/rules/{version} | Get a firewall rule version
[**get_firewall_rule_versions**](FirewallsApi.md#get_firewall_rule_versions) | **GET** /{apiVersion}/networking/firewalls/{firewallId}/history | List firewall rule versions
[**get_firewall_rules**](FirewallsApi.md#get_firewall_rules) | **GET** /{apiVersion}/networking/firewalls/{firewallId}/rules | List firewall rules
[**get_firewalls**](FirewallsApi.md#get_firewalls) | **GET** /{apiVersion}/networking/firewalls | List firewalls
[**get_linode_firewalls**](FirewallsApi.md#get_linode_firewalls) | **GET** /{apiVersion}/linode/instances/{linodeId}/firewalls | List a Linode's firewalls
[**get_node_balancer_firewalls**](FirewallsApi.md#get_node_balancer_firewalls) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId}/firewalls | List NodeBalancer firewalls
[**post_apply_firewalls**](FirewallsApi.md#post_apply_firewalls) | **POST** /{apiVersion}/linode/instances/{linodeId}/firewalls/apply | Apply a Linode's firewalls
[**post_firewalls**](FirewallsApi.md#post_firewalls) | **POST** /{apiVersion}/networking/firewalls | Create a firewall
[**put_firewall**](FirewallsApi.md#put_firewall) | **PUT** /{apiVersion}/networking/firewalls/{firewallId} | Update a firewall
[**put_firewall_rules**](FirewallsApi.md#put_firewall_rules) | **PUT** /{apiVersion}/networking/firewalls/{firewallId}/rules | Update firewall rules



## delete_firewall

> serde_json::Value delete_firewall(api_version, firewall_id)
Delete a firewall

Delete a Firewall resource by its ID. This removes all of the Firewall's Rules from any services that the Firewall was assigned to.  - Assigned Linodes must not have any ongoing live migrations.  - A `firewall_delete` Event is generated when this operation returns successfully.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewall

> models::GetFirewall200Response get_firewall(api_version, firewall_id)
Get a firewall

Get a specific Firewall resource by its ID. The Firewall's Devices will not be returned in the response. Instead, run the [List firewall devices](https://techdocs.akamai.com/linode-api/reference/get-firewall-devices) operation to review them.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |

### Return type

[**models::GetFirewall200Response**](get_firewall_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewall_rule_version

> models::GetFirewallRuleVersion200Response get_firewall_rule_version(api_version, firewall_id, version)
Get a firewall rule version

Get a specific firewall rule version for an `enabled` or `disabled` firewall.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls version-view \\   123 2     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**version** | **i32** | The firewall rule version to view. | [required] |

### Return type

[**models::GetFirewallRuleVersion200Response**](get_firewall_rule_version_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewall_rule_versions

> models::GetFirewallRuleVersions200Response get_firewall_rule_versions(api_version, firewall_id)
List firewall rule versions

Lists the current and historical rules of the firewall (that is not deleted), using `version`. Whenever rules update, the `version` increments from `1`.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls versions-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |

### Return type

[**models::GetFirewallRuleVersions200Response**](get_firewall_rule_versions_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewall_rules

> models::GetFirewallRules200Response get_firewall_rules(api_version, firewall_id)
List firewall rules

Returns the inbound and outbound Rules for a Firewall.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls rules-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |

### Return type

[**models::GetFirewallRules200Response**](get_firewall_rules_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewalls

> models::GetFirewalls200Response get_firewalls(api_version, page, page_size)
List firewalls

Returns a paginated list of accessible Firewalls.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetFirewalls200Response**](get_firewalls_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_firewalls

> models::GetLinodeFirewalls200Response get_linode_firewalls(api_version, linode_id, page, page_size)
List a Linode's firewalls

View Firewall information for Firewalls assigned to this Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes firewalls-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to access. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLinodeFirewalls200Response**](get_linode_firewalls_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer_firewalls

> models::GetNodeBalancerFirewalls200Response get_node_balancer_firewalls(api_version, node_balancer_id)
List NodeBalancer firewalls

View information for Firewalls assigned to this NodeBalancer.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers firewalls $nodeBalancerId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |

### Return type

[**models::GetNodeBalancerFirewalls200Response**](get_node_balancer_firewalls_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_apply_firewalls

> serde_json::Value post_apply_firewalls(api_version, linode_id)
Apply a Linode's firewalls

Reapply assigned firewalls to a Linode in case they were not applied successfully.  The `firewall_apply` event indicates if the firewalls were applied.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes apply-firewalls 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_firewalls

> models::PostFirewalls200Response post_firewalls(api_version, post_firewalls_request)
Create a firewall

Creates a Firewall to filter network traffic.  - Use `rules` to create inbound and outbound access rules. Rule versions increment from `1` whenever the firewall's `rules` change.  - Use `devices` to assign the firewall to a service and apply its rules to the device. Requires `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) to the device. Currently, firewalls can be assigned to Linode compute instances and NodeBalancers.  - A Firewall can be assigned to multiple services at a time.  - Use `firewall_id` to assign a firewall when [creating a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance).  - A service can have one assigned Firewall at a time.  - Firewalls apply to all of a Linode's non-`vlan` purpose Configuration Profile Interfaces.  - Assigned Linodes must not have any ongoing live migrations.  - A `firewall_create` Event is generated when this operation succeeds.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls create \\   --label example-firewall \\   --rules.outbound_policy ACCEPT \\   --rules.inbound_policy DROP \\   --rules.inbound '[{\"protocol\": \"TCP\", \"ports\": \"22, 80, 8080, 443\", \"addresses\": {\"ipv4\": [\"192.0.2.0/24\", \"198.51.100.2/32\"], \"ipv6\": [\"2001:DB8::/128\"]}, \"action\": \"ACCEPT\"}]' \\   --rules.outbound '[{\"protocol\": \"TCP\", \"ports\": \"49152-65535\", \"addresses\": {\"ipv4\": [\"192.0.2.0/24\", \"198.51.100.2/32\"],\"ipv6\": [\"2001:DB8::/128\"]}, \"action\": \"DROP\", \"label\": \"outbound-rule123\", \"description\": \"An example outbound rule description.\"}]'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_firewalls_request** | Option<[**PostFirewallsRequest**](PostFirewallsRequest.md)> | Creates a Firewall object that can be applied to a service to filter the service's network traffic. |  |

### Return type

[**models::PostFirewalls200Response**](post_firewalls_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_firewall

> models::PutFirewall200Response put_firewall(api_version, firewall_id, put_firewall_request)
Update a firewall

Updates information for a Firewall.  - Assigned Linodes must not have any ongoing live migrations.  - If a Firewall's status is changed with this operation, a corresponding `firewall_enable` or `firewall_disable` Event will be generated.  Some parts of a Firewall's configuration cannot be manipulated by this operation:  - A Firewall's Devices cannot be set with this operation. Instead, run the [Create a firewall device](https://techdocs.akamai.com/linode-api/reference/post-firewall-device) and [Delete a firewall device](https://techdocs.akamai.com/linode-api/reference/delete-firewall-device) operations to assign and remove this Firewall from services.  - A Firewall's Rules cannot be changed with this operation. Instead, run the [Update firewall rules](https://techdocs.akamai.com/linode-api/reference/put-firewall-rules) operation to update your Rules.  - A Firewall's status can be set to `enabled` or `disabled` by this operation, but it cannot be set to `deleted`. Instead, run the [Delete a firewall](https://techdocs.akamai.com/linode-api/reference/delete-firewall) operation to delete a Firewall.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls update 123 \\   --status disabled     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**put_firewall_request** | Option<[**PutFirewallRequest**](PutFirewallRequest.md)> | The Firewall information to update. |  |

### Return type

[**models::PutFirewall200Response**](put_firewall_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_firewall_rules

> models::PutFirewallRules200Response put_firewall_rules(api_version, firewall_id, put_firewall_rules_request)
Update firewall rules

Updates the inbound and outbound Rules for a Firewall.  - Assigned Linodes must not have any ongoing live migrations.  - __Note__. This operation replaces all of a Firewall's `inbound` and `outbound` rulesets with the values specified in your request.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls rules-update 123 \\   --inbound '[{\"action\":\"ACCEPT\", \"protocol\": \"TCP\", \"ports\": \"22, 80, 8080, 443\", \"addresses\": {\"ipv4\": [\"192.0.2.0/24\", \"198.51.100.2/32\"], \"ipv6\": [\"2001:DB8::/128\"]}}]' \\   --outbound '[{\"action\":\"DROP\",\"protocol\": \"TCP\", \"ports\": \"49152-65535\", \"addresses\": {\"ipv4\": [\"192.0.2.0/24\", \"198.51.100.2/32\"], \"ipv6\": [\"2001:DB8::/128`\"]}}]'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**put_firewall_rules_request** | Option<[**PutFirewallRulesRequest**](PutFirewallRulesRequest.md)> | The Firewall Rules information to update. |  |

### Return type

[**models::PutFirewallRules200Response**](put_firewall_rules_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

