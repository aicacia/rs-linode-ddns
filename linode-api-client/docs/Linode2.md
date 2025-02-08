# Linode2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alerts** | Option<[**models::LinodeAlerts**](Linode_alerts.md)> |  | [optional]
**backups** | Option<[**models::LinodeBackups**](Linode_backups.md)> |  | [optional]
**capabilities** | Option<**Vec<String>**> | __Limited availability__, __Read-only__ A list of capabilities this compute instance supports. | [optional][readonly]
**created** | Option<**String**> | __Read-only__ When this Linode was created. | [optional][readonly]
**disk_encryption** | Option<**String**> | __Limited availability__, __Read-only__ Indicates the local disk encryption setting for this Linode. If the Linode is part of an LKE cluster, the value is `null`. | [optional][readonly][default to Enabled]
**group** | Option<**String**> | __Deprecated__, __Filterable__ The group label for this Linode. | [optional]
**has_user_data** | Option<**bool**> | __Read-only__ Whether this compute instance was provisioned with `user_data` provided via the Metadata service. See the [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) description for more information on Metadata. | [optional][readonly]
**host_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | __Read-only__ The Linode's host machine, as a UUID. | [optional][readonly]
**hypervisor** | Option<**String**> | __Read-only__ The virtualization software powering this Linode. | [optional][readonly]
**id** | Option<**i32**> | __Filterable__, __Read-only__ This Linode's ID which must be provided for all operations impacting this Linode. | [optional][readonly]
**image** | Option<**String**> | An Image ID to deploy the Linode Disk from.  Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation with authentication to view all available Images. Official Linode Images start with `linode/`, while your Account's Images start with `private/`. Creating a disk from a Private Image requires `read_only` or `read_write` permissions for that Image. Run the [Update a user's grants](https://techdocs.akamai.com/linode-api/reference/put-user-grants) operation to adjust permissions for an Account Image. | [optional]
**ipv4** | Option<**Vec<String>**> | __Filterable__, __Read-only__ This Linode's IPv4 Addresses. Each Linode is assigned a single public IPv4 address upon creation, and may get a single private IPv4 address if needed. You may need to [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to get additional IPv4 addresses.  IPv4 addresses may be reassigned between your Linodes, or shared with other Linodes. See the [networking](https://techdocs.akamai.com/linode-api/reference/post-firewalls) operations for details. | [optional][readonly]
**ipv6** | Option<**String**> | __Read-only__ This Linode's IPv6 SLAAC address. This address is specific to a Linode, and may not be shared. If the Linode has not been assigned an IPv6 address, the return value will be `null`. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ Provides a name for the Linode. If not provided, the API generates one for it.  Linode labels have the following constraints:  - It needs to begin and end with an alphanumeric character. - It can only consist of alphanumeric characters, hyphens (`-`), underscores (`_`) or periods (`.`). - Cannot have two hyphens (`--`), underscores (`__`) or periods (`..`) in a row. | [optional]
**lke_cluster_id** | Option<**i32**> | __Read-only__ The ID of the Kubernetes cluster if the Linode is part of cluster. | [optional][readonly]
**placement_group** | Option<[**models::LinodePlacementGroup**](Linode_placement_group.md)> |  | [optional]
**region** | Option<**String**> | __Filterable__, __Read-only__ The [region](https://techdocs.akamai.com/linode-api/reference/get-regions) where the Linode deployed. A Linode's region can only be changed by initiating a [cross data center migration](https://techdocs.akamai.com/linode-api/reference/post-migrate-linode-instance). | [optional][readonly]
**specs** | Option<[**models::LinodeSpecs**](Linode_specs.md)> |  | [optional]
**status** | Option<**String**> | __Read-only__ A brief description of the compute instance's current state. This value can change without direct action from you. For example, when a compute instance goes into maintenance mode, its status is `stopped`. Status is generally self-explanatory, based on its name.  - `busy` indicates you've assigned the compute instance to a [placement group](https://techdocs.akamai.com/cloud-computing/docs/work-with-placement-groups), but the compute instance is currently booting. Once the boot completes, the API completes the assignment and updates the compute instance's `status` accordingly. - `provisioning` indicates that the API is applying operating system or Marketplace applications on the compute instance. - `billing_suspension` indicates that payment is past due on the compute instance, so we've suspended its use. | [optional][readonly]
**tags** | Option<**Vec<String>**> | __Filterable__ Tags to help you organize your content. | [optional]
**r#type** | Option<**String**> | __Read-only__ This is the [Linode type](https://techdocs.akamai.com/linode-api/reference/get-linode-types) that this Linode was deployed with. To change a Linode's type, use [Resize a Linode](https://techdocs.akamai.com/linode-api/reference/post-resize-linode-instance). | [optional][readonly]
**updated** | Option<**String**> | __Read-only__ When this Linode was last updated. | [optional][readonly]
**watchdog_enabled** | Option<**bool**> | The watchdog, named Lassie, is a Shutdown Watchdog that monitors your Linode and reboots it if it powers off unexpectedly. It works by issuing a boot job when your Linode powers off without a shutdown job being responsible. To prevent a loop, Lassie gives up if there have been more than 5 boot jobs issued within 15 minutes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


