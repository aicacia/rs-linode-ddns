# \LinodeInstancesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_linode_instance**](LinodeInstancesApi.md#delete_linode_instance) | **DELETE** /{apiVersion}/linode/instances/{linodeId} | Delete a Linode
[**get_linode_instance**](LinodeInstancesApi.md#get_linode_instance) | **GET** /{apiVersion}/linode/instances/{linodeId} | Get a Linode
[**get_linode_instances**](LinodeInstancesApi.md#get_linode_instances) | **GET** /{apiVersion}/linode/instances | List Linodes
[**post_boot_linode_instance**](LinodeInstancesApi.md#post_boot_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/boot | Boot a Linode
[**post_clone_linode_instance**](LinodeInstancesApi.md#post_clone_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/clone | Clone a Linode
[**post_linode_instance**](LinodeInstancesApi.md#post_linode_instance) | **POST** /{apiVersion}/linode/instances | Create a Linode
[**post_migrate_linode_instance**](LinodeInstancesApi.md#post_migrate_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/migrate | Initiate a DC migration/pending host migration
[**post_mutate_linode_instance**](LinodeInstancesApi.md#post_mutate_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/mutate | Upgrade a Linode
[**post_reboot_linode_instance**](LinodeInstancesApi.md#post_reboot_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/reboot | Reboot a Linode
[**post_rebuild_linode_instance**](LinodeInstancesApi.md#post_rebuild_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/rebuild | Rebuild a Linode
[**post_rescue_linode_instance**](LinodeInstancesApi.md#post_rescue_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/rescue | Boot a Linode into rescue mode
[**post_reset_linode_password**](LinodeInstancesApi.md#post_reset_linode_password) | **POST** /{apiVersion}/linode/instances/{linodeId}/password | Reset a Linode's root password
[**post_resize_linode_instance**](LinodeInstancesApi.md#post_resize_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/resize | Resize a Linode
[**post_shutdown_linode_instance**](LinodeInstancesApi.md#post_shutdown_linode_instance) | **POST** /{apiVersion}/linode/instances/{linodeId}/shutdown | Shut down a Linode
[**put_linode_instance**](LinodeInstancesApi.md#put_linode_instance) | **PUT** /{apiVersion}/linode/instances/{linodeId} | Update a Linode



## delete_linode_instance

> serde_json::Value delete_linode_instance(api_version, linode_id)
Delete a Linode

Deletes a Linode you have permission to `read_write`.  __Deleting a Linode is a destructive action and cannot be undone.__  Additionally, deleting a Linode:    - Gives up any IP addresses the Linode was assigned.   - Deletes all Disks, Backups, Configs, etc.   - Detaches any Volumes associated with the Linode.   - Stops billing for the Linode and its associated services. You will be billed for time used within the billing period the Linode was active.  Linodes that are in the process of [cloning](https://techdocs.akamai.com/linode-api/reference/post-clone-linode-instance) or [backup restoration](https://techdocs.akamai.com/linode-api/reference/post-restore-backup) cannot be deleted.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_instance

> models::Linode2 get_linode_instance(api_version, linode_id)
Get a Linode

Get a specific Linode by ID.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**models::Linode2**](Linode_2.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_instances

> models::GetLinodeInstances200Response get_linode_instances(api_version, x_filter, page, page_size)
List Linodes

Returns a paginated list of Linodes you have permission to view.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**x_filter** | Option<[**GetLinodeInstancesXFilterParameter**](.md)> | Specifies a JSON object to filter down the results. See [Filtering and sorting](filtering-and-sorting) for details. |  |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLinodeInstances200Response**](get_linode_instances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_boot_linode_instance

> serde_json::Value post_boot_linode_instance(api_version, linode_id, post_boot_linode_instance_request)
Boot a Linode

Boots a Linode you have permission to modify. If no parameters are given, a Config profile will be chosen for this boot based on the following criteria:  - If there is only one Config profile for this Linode, it will be used. - If there is more than one Config profile, the last booted config will be used. - If there is more than one Config profile and none were the last to be booted (because the Linode was never booted or the last booted config was deleted) an error will be returned.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes boot 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode to boot. | [required] |
**post_boot_linode_instance_request** | Option<[**PostBootLinodeInstanceRequest**](PostBootLinodeInstanceRequest.md)> | Optional configuration to boot into (see above). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clone_linode_instance

> models::Linode5 post_clone_linode_instance(api_version, linode_id, post_clone_linode_instance_request)
Clone a Linode

You can clone your Linode's existing Disks or Configuration profiles to another Linode on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Cloning to a new Linode will incur a charge on your Account.  If cloning to an existing Linode, any actions currently running or queued must be completed first before you can clone to it.  Up to five clone operations from any given source Linode can be run concurrently. If more concurrent clones are attempted, an HTTP 400 error will be returned by this operation.  Any [tags](https://techdocs.akamai.com/linode-api/reference/get-tags) existing on the source Linode will be cloned to the target Linode.  Linodes utilizing Metadata (`\"has_user_data\": true`) must be cloned to a new Linode with `metadata.user_data` included with the clone request.  `vpc` details  - If the Linode you are cloning has a `vpc` purpose Interface on its active Configuration Profile that includes a 1:1 NAT, the resulting clone is configured with an `any` 1:1 NAT. - See the [VPC documentation](https://www.linode.com/docs/products/networking/vpc/#technical-specifications) guide for its specifications and limitations.  `vlan` details  - Only Next Generation Network (NGN) data centers support VLANs. If a VLAN is attached to your Linode and you attempt clone it to a non-NGN data center, the cloning will not initiate. If a Linode cannot be cloned because of an incompatibility, you will be prompted to select a different data center or contact support. - See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes clone 123 \\   --linode_id 124 \\   --region us-east \\   --type g6-standard-2 \\   --label cloned-linode \\   --backups_enabled true \\   --placement_group.id 528 \\   --disks 25674 \\   --configs 23456 \\   --private_ip true \\   --metadata.user_data I2Nsb3VkLWNvbmZpZw==     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to clone. | [required] |
**post_clone_linode_instance_request** | [**PostCloneLinodeInstanceRequest**](PostCloneLinodeInstanceRequest.md) | The requested state your Linode will be cloned into. | [required] |

### Return type

[**models::Linode5**](Linode_5.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_linode_instance

> models::Linode1 post_linode_instance(api_version, post_linode_instance_request)
Create a Linode

Creates a Linode Instance on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Creating a new Linode will incur a charge on your Account.  Linodes can be created using one of the available Types. Run [List Linode types](https://techdocs.akamai.com/linode-api/reference/get-linode-types) to get more information about each Type's specs and cost.  Linodes can be created in any one of our available Regions, which are accessible from the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation.  In an effort to fight spam, Linode restricts outbound connections on ports 25, 465, and 587 on all Linodes for new accounts created after November 5th, 2019. For more information, see our guide on [Running a Mail Server](https://www.linode.com/docs/guides/running-a-mail-server/).  __Important__. You must be an unrestricted User in order to add or modify tags on Linodes.  Linodes can be created in a number of ways:  - Using a Linode Public Image distribution or a Private Image you created based on another Linode.    - Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation with authentication to view all available Images.    - The Linode will be `running` after it completes `provisioning`.   - A default config with two Disks, one being a 512 swap disk, is created.     - `swap_size` can be used to customize the swap disk size.   - Requires a `root_pass` be supplied to use for the root User's Account.   - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.   - You may also supply a list of usernames via the `authorized_users` field.     - These users must have an SSH Key associated with your Profile first. See the [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key)) operation for more information.  - Using cloud-init with [Metadata](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata/).   - Automate system configuration and software installation by providing a base-64 encoded [cloud-config](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata-cloud-config/) file.   - Requires a compatible Image. You can determine compatible Images by checking for `cloud-init` under `capabilities` when running [List images](https://techdocs.akamai.com/linode-api/reference/get-images).   - Requires a compatible Region.  You can determine compatible Regions by checking for `Metadata` under `capabilities` when running [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions).  - Using a StackScript.    - Run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts) for a list of available StackScripts.   - The Linode will be `running` after it completes `provisioning`.   - Requires a compatible Image to be supplied.     - Run [Get a StackScript](https://techdocs.akamai.com/linode-api/reference/get-stack-script) for compatible Images.   - Requires a `root_pass` be supplied to use for the root User's Account.   - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.   - You may also supply a list of usernames via the `authorized_users` field.     - These users must have an SSH Key associated with your Profile first. See [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) for more information.  - Using one of your other Linode's backups.   - You must create a Linode large enough to accommodate the Backup's size.   - The Disks and Config will match that of the Linode that was backed up.   - The `root_pass` will match that of the Linode that was backed up.  - Attached to a private VLAN.   - Review the `interfaces` property of the [Request Body Schema](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) for details.   - For more information, see our guide on [Getting Started with VLANs](https://www.linode.com/docs/products/networking/vlans/get-started/).  - Create an empty Linode.   - The Linode will remain `offline` and must be manually started.     - Run [Boot a Linode](https://techdocs.akamai.com/linode-api/reference/post-boot-linode-instance).   - Disks and Configs must be created manually.   - This is only recommended for advanced use cases.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes create \\   --label linode123 \\   --root_pass aComplex@Password \\   --booted true \\   --stackscript_id 10079 \\   --stackscript_data '{\"gh_username\": \"linode\"}' \\   --region us-east \\   --disk_encryption enabled\\   --placement_group.id 528 \\   --type g6-standard-2 \\   --authorized_keys \"ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer\" \\   --authorized_users \"myUser\" \\   --authorized_users \"secondaryUser\" \\   --metadata.user_data \"I2Nsb3VkLWNvbmZpZw==\" \\   --firewall_id 9000     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_linode_instance_request** | [**PostLinodeInstanceRequest**](PostLinodeInstanceRequest.md) | The requested initial state of a new Linode. | [required] |

### Return type

[**models::Linode1**](Linode_1.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_migrate_linode_instance

> serde_json::Value post_migrate_linode_instance(api_version, linode_id, post_migrate_linode_instance_request)
Initiate a DC migration/pending host migration

Initiate a pending host migration that has been scheduled by Linode or initiate a cross data center (DC) migration.  A list of pending migrations, if any, can be accessed from [List notifications](https://techdocs.akamai.com/linode-api/reference/get-notifications). When the migration begins, your Linode will be shutdown if not already off. If the migration initiated the shutdown, it will reboot the Linode when completed.  To initiate a cross DC migration, you must pass a `region` parameter to the request body specifying the target data center region. You can view a list of all available regions and their feature capabilities from [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions). See our [Pricing Page](https://www.linode.com/pricing/) for Region-specific pricing, which applies after migration is complete. If your Linode has a DC migration already queued or you have initiated a previously scheduled migration, you will not be able to initiate a DC migration until it has completed.  `vpc` details  - Cross DC migrations are not allowed for Linodes that have a `vpc` purpose Configuration Profile Interface. Host migrations within the same DC are permitted. - See the [VPC documentation](https://www.linode.com/docs/products/networking/vpc/#technical-specifications) guide for its specifications and limitations.  `vlan` details  - Only Next Generation Network (NGN) data centers support VLANs. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated or cloned because of an incompatibility, you will be prompted to select a different data center or contact support. - Next Generation Network (NGN) data centers do not support IPv6 `/116` pools or IP Failover. If you have these features enabled on your Linode and attempt to migrate to an NGN data center, the migration will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support. - See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes migrate 123 \\   --region us-central \\   --placement_group.id 528     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to migrate. | [required] |
**post_migrate_linode_instance_request** | Option<[**PostMigrateLinodeInstanceRequest**](PostMigrateLinodeInstanceRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_mutate_linode_instance

> serde_json::Value post_mutate_linode_instance(api_version, linode_id, post_mutate_linode_instance_request)
Upgrade a Linode

Linodes created with now-deprecated Types are entitled to a free upgrade to the next generation. A mutating Linode will be allocated any new resources the upgraded Type provides, and will be subsequently restarted if it was currently running. If any actions are currently running or queued, those actions must be completed first before you can initiate a mutate.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes upgrade 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to mutate. | [required] |
**post_mutate_linode_instance_request** | Option<[**PostMutateLinodeInstanceRequest**](PostMutateLinodeInstanceRequest.md)> | Whether to automatically resize disks or not. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_reboot_linode_instance

> serde_json::Value post_reboot_linode_instance(api_version, linode_id, post_reboot_linode_instance_request)
Reboot a Linode

Reboots a Linode you have permission to modify. If any actions are currently running or queued, those actions must be completed first before you can initiate a reboot.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes reboot 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the linode to reboot. | [required] |
**post_reboot_linode_instance_request** | Option<[**PostRebootLinodeInstanceRequest**](PostRebootLinodeInstanceRequest.md)> | Optional reboot parameters. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_rebuild_linode_instance

> models::Linode6 post_rebuild_linode_instance(api_version, linode_id, post_rebuild_linode_instance_request)
Rebuild a Linode

Rebuilds a Linode you have the `read_write` permission to modify.  A rebuild will first shut down the Linode, delete all disks and configs on the Linode, and then deploy a new `image` to the Linode with the given attributes. Additionally:    - Requires an `image` be supplied.   - Requires a `root_pass` be supplied to use for the root User's Account.   - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.   - Linodes utilizing Metadata (`\"has_user_data\": true`) should include `metadata.user_data` in the rebuild request to continue using the service.  During a rebuild, you can `enable` or `disable` local disk encryption. If disk encryption is not included in the request, the previous `disk_encryption` value is used. Disk encryption cannot be disabled if the compute instance is attached to a LKE nodepool.  You also have the option to resize the Linode to a different plan by including the `type` parameter with your request. Note that resizing involves migrating the Linode to a new hardware host, while rebuilding without resizing maintains the same hardware host. Resizing also requires significantly more time for completion of this operation. The following additional conditions apply:    - The Linode must not have a pending migration.   - Your Account cannot have an outstanding balance.   - The Linode must not have more disk allocation than the new Type allows.     - In that situation, you must first delete or resize the disk to be smaller.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes rebuild 123 \\   --image \"linode/debian9\" \\   --root_pass aComplex@Password \\   --disk_encryption disabled \\   --authorized_keys \"ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer\" \\   --authorized_users \"myUsername\" \\   --authorized_users \"secondaryUsername\" \\   --booted true \\   --stackscript_id 10079 \\   --stackscript_data '{\"gh_username\": \"linode\"}' \\   --type \"g6-standard-2\" \\   --metadata.userdata \"I2Nsb3VkLWNvbmZpZw==\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to rebuild. | [required] |
**post_rebuild_linode_instance_request** | [**PostRebuildLinodeInstanceRequest**](PostRebuildLinodeInstanceRequest.md) | The requested state your Linode will be rebuilt into. | [required] |

### Return type

[**models::Linode6**](Linode_6.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_rescue_linode_instance

> serde_json::Value post_rescue_linode_instance(api_version, linode_id, post_rescue_linode_instance_request)
Boot a Linode into rescue mode

Rescue Mode is a safe environment for performing many system recovery and disk management tasks. Rescue Mode is based on the Finnix recovery distribution, a self-contained and bootable Linux distribution. You can also use Rescue Mode for tasks other than disaster recovery, such as formatting disks to use different filesystems, copying data between disks, and downloading files from a disk via SSH and SFTP.  - Note that `sdh` is reserved and unavailable during rescue.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes rescue 123 \\   --devices.sda.disk_id 124458     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to rescue. | [required] |
**post_rescue_linode_instance_request** | Option<[**PostRescueLinodeInstanceRequest**](PostRescueLinodeInstanceRequest.md)> | Optional object of devices to be mounted. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_reset_linode_password

> serde_json::Value post_reset_linode_password(api_version, linode_id, post_reset_linode_password_request)
Reset a Linode's root password

Resets the root password for this Linode.  - Your Linode must be [shut down](https://techdocs.akamai.com/linode-api/reference/post-shutdown-linode-instance) for a password reset to complete. - If your Linode has more than one disk (not counting its swap disk), run the [Reset a disk root password](https://techdocs.akamai.com/linode-api/reference/post-reset-disk-password) operation to update a specific disk's root password. - A `password_reset` event is generated when a root password reset is successful.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes linode-reset-password 123 a$eCure4assw0rd!43v51     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode for which to reset its root password. | [required] |
**post_reset_linode_password_request** | Option<[**PostResetLinodePasswordRequest**](PostResetLinodePasswordRequest.md)> | This Linode's new root password. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_resize_linode_instance

> serde_json::Value post_resize_linode_instance(api_version, linode_id, post_resize_linode_instance_request)
Resize a Linode

Resizes a Linode you have the `read_write` permission to a different Type. If any actions are currently running or queued, those actions must be completed first before you can initiate a resize. Additionally, the following criteria must be met in order to resize a Linode:    - The Linode must not have a pending migration.   - Your Account cannot have an outstanding balance.   - The Linode must not have more disk allocation than the new Type allows.     - In that situation, you must first delete or resize the disk to be smaller.  You can also resize a Linode when using the [Rebuild a Linode](https://techdocs.akamai.com/linode-api/reference/post-rebuild-linode-instance) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes resize 123 \\   --type g6-standard-2     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to resize. | [required] |
**post_resize_linode_instance_request** | [**PostResizeLinodeInstanceRequest**](PostResizeLinodeInstanceRequest.md) | The Type your current Linode will resize to, and whether to attempt to automatically resize the Linode's disks. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shutdown_linode_instance

> serde_json::Value post_shutdown_linode_instance(api_version, linode_id)
Shut down a Linode

Shuts down a Linode you have permission to modify. If any actions are currently running or queued, those actions must be completed first before you can initiate a shutdown.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes shutdown 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to shutdown. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_linode_instance

> models::Linode4 put_linode_instance(api_version, linode_id, linode3)
Update a Linode

Updates a Linode that you have permission to `read_write`. Only unrestricted users can add or modify tags on Linodes. <<LB>>  > 🚧 > > All tags for the instance are overwritten if `tags` are included in the request.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes update 7833080 \\   --label linode123 \\   --backups.schedule.day \"Saturday\" \\   --backups.schedule.window \"W22\" \\   --alerts.cpu 180 \\   --alerts.network_in 10 \\   --alerts.network_out 10 \\   --alerts.transfer_quota 80 \\   --alerts.io 10000     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**linode3** | [**Linode3**](Linode3.md) | Any field that is not marked as `readOnly` may be updated. Fields that are marked `readOnly` will be ignored. If any updated field fails to pass validation, the Linode will not be updated. | [required] |

### Return type

[**models::Linode4**](Linode_4.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

