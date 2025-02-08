# PostTicketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket** | Option<**String**> | The name of an Object Storage bucket entity for this ticket. Run the [List Object Storage buckets](https://techdocs.akamai.com/linode-api/reference/get-object-storage-buckets) operation and store the `label` for the target bucket. You also need to provide the specific `region` where the bucket is located. | [optional]
**database_id** | Option<**i32**> | The ID of the Managed Database entity for the ticket. Run the [List Managed Databases](https://techdocs.akamai.com/linode-api/reference/get-databases-instances) operation and store the `id` for the target database. | [optional]
**description** | **String** | The full details of the issue or question. | 
**domain_id** | Option<**i32**> | The ID of the domain entity for the ticket. Run the [List domains](https://techdocs.akamai.com/linode-api/reference/get-domains) operation and store the `id` for the target domain. | [optional]
**firewall_id** | Option<**i32**> | The ID of the Firewall entity for the ticket. Run the [List a Linode's firewalls](https://techdocs.akamai.com/linode-api/reference/get-linode-firewalls) operation and store the `id` for the target Linode firewall. | [optional]
**linode_id** | Option<**i32**> | The ID of the Linode entity for the ticket. Run the [List Linodes](https://techdocs.akamai.com/linode-api/reference/get-linode-instances) operation and store the `id` for the target Linode. | [optional]
**lkecluster_id** | Option<**i32**> | The ID of the Linode Kubernetes Engine (LKE) cluster entity for the ticket. Run the [List Kubernetes clusters](https://techdocs.akamai.com/linode-api/reference/get-lke-clusters) operation and store the `id` for the target LKE cluster. | [optional]
**longviewclient_id** | Option<**i32**> | The ID of the Longview client entity for the ticket. Run the [List Longview clients](https://techdocs.akamai.com/linode-api/reference/get-longview-clients) operation and store the `id` for the target client. | [optional]
**managed_issue** | Option<**bool**> | Whether this ticket is related to a [managed service](https://www.linode.com/products/managed/). If `true`, the following constraints apply:  - You can't provide an entity, such as a `linode_id` or `bucket` with this request.  - Your account needs a managed service [enabled](https://techdocs.akamai.com/linode-api/reference/post-enable-managed-service). | [optional][default to false]
**nodebalancer_id** | Option<**i32**> | The ID of the NodeBalancer entity for the ticket. Run the [List NodeBalancers](https://techdocs.akamai.com/linode-api/reference/get-node-balancers) operation and store the `id` for the target NodeBalancer. | [optional]
**region** | Option<**String**> | The ID of the [region](https://techdocs.akamai.com/linode-api/reference/get-regions) where this ticket's target entity resides. This only applies to tickets for a `vlan` or an Object Storage  `bucket`.  > 📘 > > Set this to the `clusterId` for a legacy Object Storage `bucket`. | [optional]
**summary** | **String** | The summary or title for this support ticket. | 
**vlan** | Option<**String**> | The label of the VLAN entity for the ticket. Run the [List VLANs](https://techdocs.akamai.com/linode-api/reference/get-vlans) operation and store the `id` for the target VLAN. You also need to provide the specific `region` where the VLAN is located. | [optional]
**volume_id** | Option<**i32**> | The ID of the volume entity for the ticket. Run the [List volumes](https://techdocs.akamai.com/linode-api/reference/get-volumes) operation and store the `id` for the target volume. | [optional]
**vpc_id** | Option<**i32**> | The ID of the VPC entity for the ticket. Run the [List VPCs](https://techdocs.akamai.com/linode-api/reference/get-vpcs) operation and store the `id` for the target VPC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


