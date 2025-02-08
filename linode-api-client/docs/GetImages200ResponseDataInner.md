# GetImages200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capabilities** | Option<**Vec<String>**> | __Read-only__ A list of the possible capabilities of this image.  - `cloud-init`. The image supports the cloud-init multi-distribution method with our [Metadata service](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata/#troubleshoot-metadata-and-cloud-init). This only applies to public images.  - `distributed-sites`. Whether the image can be used in distributed compute regions. Compared to a core compute region, distributed compute regions offer limited functionality, but they're globally distributed. Your image can be geographically closer to you, potentially letting you deploy it quicker. See [Regions and images](https://techdocs.akamai.com/cloud-computing/docs/images#regions-and-images) for complete details. | [optional][readonly]
**created** | Option<**String**> | __Read-only__ When this image was created. | [optional][readonly]
**created_by** | Option<**String**> | __Read-only__ The name of the user who created this image, or `linode` for public images. | [optional][readonly]
**deprecated** | Option<**bool**> | __Filterable__, __Read-only__ Whether this image is deprecated. Only public images can be deprecated. | [optional][readonly]
**description** | Option<**String**> | A detailed description of this image. | [optional]
**eol** | Option<**String**> | __Read-only__ The date of the public image's planned removal from service. This is `null` for private images. | [optional][readonly]
**expiry** | Option<**String**> | __Read-only__ Only images created automatically from a deleted compute instance (type=automatic) expire. This is `null` for private images. | [optional][readonly]
**id** | Option<**String**> | __Read-only__ The unique identifier for each image. | [optional][readonly]
**is_public** | Option<**bool**> | __Filterable__, __Read-only__ Revealed as `true` if the image is a public distribution image. Private, account-specific images are listed as `false`. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ A short description of the image. | [optional]
**regions** | Option<[**Vec<models::GetImages200ResponseDataInnerRegionsInner>**](get_images_200_response_data_inner_regions_inner.md)> | __Read-only__ Details on the regions where this image is stored. See [Regions and images](https://techdocs.akamai.com/cloud-computing/docs/images#regions-and-images) for full details on support for `regions`. | [optional][readonly]
**size** | Option<**i32**> | __Filterable__, __Read-only__ The minimum size in MB this image needs to deploy. | [optional][readonly]
**status** | Option<**String**> | __Filterable__, __Read-only__ The current status of the image. Possible values are `available`, `creating`, and `pending_upload`.  > 📘 > > The `+order_by` and `+order` operators are not available when [filtering](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) on this key. | [optional][readonly]
**tags** | Option<**Vec<String>**> | __Filterable__ Tags used for organizational purposes. A tag can be from 3 to 100 characters long, and an image can have a maximum of 500 total tags. | [optional]
**total_size** | Option<**i32**> | __Read-only__ The total size in bytes of all instances of this image, in all `regions`.  > 📘 > > This object is empty for existing images. It's intended for use with future functionality. | [optional][readonly]
**r#type** | Option<**String**> | __Filterable__, __Read-only__ How the image was created. Create a `manual` image at any time. An `automatic` image is created automatically from a deleted compute instance. | [optional][readonly]
**updated** | Option<**String**> | __Read-only__ When this image was last updated. | [optional][readonly]
**vendor** | Option<**String**> | __Filterable__, __Read-only__ The upstream distribution vendor. This is `null` for private images. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


