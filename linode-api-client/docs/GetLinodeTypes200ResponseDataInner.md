# GetLinodeTypes200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addons** | Option<[**models::GetLinodeTypes200ResponseDataInnerAddons**](get_linode_types_200_response_data_inner_addons.md)> |  | [optional]
**class** | Option<**String**> | __Filterable__, __Read-only__ The class of the Linode type.  - `nanode`. These instances are good for low-duty workloads, where performance isn't critical.  - `standard`. These instances are good for medium-duty workloads, and offer a good mix of performance, resources, and price.  - `dedicated`. These instances are good for full-duty workloads where consistent performance is important.  - `premium` (limited regions). This includes the features of a `dedicated` instance as well as the latest AMD EPYC&trade; CPUs. This ensures your applications are running on the latest hardware with consistently high performance. Only available in [regions](https://techdocs.akamai.com/linode-api/reference/get-regions) with \"Premium Plans\" in their `capabilities`.  - `gpu` (limited regions). Linodes with dedicated NVIDIA Quadro&reg; RTX 6000 GPUs accelerate highly specialized applications such as machine learning, AI, and video transcoding. Only available in [regions](https://techdocs.akamai.com/linode-api/reference/get-regions) with `GPU Linodes` in their `capabilities`.  - `accelerated` (limited regions - **Beta**). These leverage the power of dedicated, application-specific integrated circuits (ASIC), starting with NETINT Video Processing Units (VPUs). They're ideal for video transcoding, media processing, and other compute-heavy workloads. Designed to offload specialized tasks, these instances deliver faster processing times and greater efficiency than traditional CPU-based solutions.  - `highmem`. High Memory instances favor RAM over other resources, and can be good for memory hungry use cases like caching and in-memory databases. All High Memory plans contain dedicated CPU cores.  > 📘 > > - A `nanode` class is listed as a 1 GB Linode in Cloud Manager. The API, the CLI, and billing continue to refer to these as a Nanode. > > - A `standard` class is listed as a Shared Linode in Cloud Manager. The API, the CLI, and billing still refer to these as Standard. > > - The `accelerated` Linode type is in **Beta**. Talk to your account team or [open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket). | [optional][readonly]
**disk** | Option<**i32**> | __Filterable__, __Read-only__ The Linode type's disk size in MB. | [optional][readonly]
**gpus** | Option<**i32**> | __Filterable__, __Read-only__ The number of GPUs this Linode type offers. | [optional][readonly]
**id** | Option<**String**> | __Read-only__ The ID representing the Linode type. | [optional][readonly]
**label** | Option<**String**> | __Filterable__, __Read-only__ The display name for the Linode type. | [optional][readonly]
**memory** | Option<**i32**> | __Filterable__, __Read-only__ Amount of RAM included in this Linode type. | [optional][readonly]
**network_out** | Option<**i32**> | __Filterable__, __Read-only__ The Mbits outbound bandwidth allocation. | [optional][readonly]
**price** | Option<[**models::GetLinodeTypes200ResponseDataInnerPrice**](get_linode_types_200_response_data_inner_price.md)> |  | [optional]
**region_prices** | Option<[**Vec<models::GetLinodeTypes200ResponseDataInnerRegionPricesInner>**](get_linode_types_200_response_data_inner_region_prices_inner.md)> |  | [optional]
**successor** | Option<**String**> | __Read-only__ After a [mutate](https://techdocs.akamai.com/linode-api/reference/post-mutate-linode-instance), the Linode is upgraded to this Linode type. If `null`, this Linode type can't be mutated. | [optional][readonly]
**transfer** | Option<**i32**> | __Filterable__, __Read-only__ The monthly outbound transfer amount in MB. | [optional][readonly]
**vcpus** | Option<**i32**> | __Filterable__, __Read-only__ The number of VCPU cores this Linode type offers. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


