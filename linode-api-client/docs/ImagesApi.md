# \ImagesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /{apiVersion}/images/{imageId} | Delete an image
[**get_image**](ImagesApi.md#get_image) | **GET** /{apiVersion}/images/{imageId} | Get an image
[**get_images**](ImagesApi.md#get_images) | **GET** /{apiVersion}/images | List images
[**post_image**](ImagesApi.md#post_image) | **POST** /{apiVersion}/images | Create an image
[**post_replicate_image**](ImagesApi.md#post_replicate_image) | **POST** /{apiVersion}/images/{imageId}/regions | Replicate an image
[**post_upload_image**](ImagesApi.md#post_upload_image) | **POST** /{apiVersion}/images/upload | Upload an image
[**put_image**](ImagesApi.md#put_image) | **PUT** /{apiVersion}/images/{imageId} | Update an image



## delete_image

> serde_json::Value delete_image(api_version, image_id)
Delete an image

Deletes a private image you have permission to `read_write`.  > 🚧 > > - You can't undo this delete action. > > - When you delete an image, all [replicated instances](https://techdocs.akamai.com/linode-api/reference/post-replicate-image) of that image are also deleted.   <<LB>>  ---   - __CLI__.      ```     linode-cli images delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**image_id** | **String** | The unique identifier assigned to the image after creation. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> models::GetImages200ResponseDataInner get_image(api_version, image_id)
Get an image

Get information about a single image. An image can be one of two types:  - **Public image**. The `id` for these images begins with `linode/`. These images are generally available to all users. To limit the response to public images, don't include [authentication](https://techdocs.akamai.com/linode-api/reference/get-started#authentication) when calling this operation.  - **Private image**. The `id` for these images begins with `private/`. These images are account-specific and only accessible to users with appropriate [grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants). To view private images, you need to include authentication when calling this operation. The response will also include public images.   <<LB>>  ---   - __CLI__.      ```     linode-cli images view linode/debian9     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**image_id** | **String** | The unique identifier assigned to the image after creation. | [required] |

### Return type

[**models::GetImages200ResponseDataInner**](get_images_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_images

> models::GetImages200Response get_images(api_version, page, page_size)
List images

Returns a paginated list of images. An image can be one of two types:  - **Public image**. The `id` for these images begins with `linode/`. These images are generally available to all users. To limit the response to public images, don't include [authentication](https://techdocs.akamai.com/linode-api/reference/get-started#authentication) when calling this operation.  - **Private image**. The `id` for these images begins with `private/`. These images are account-specific and only accessible to users with appropriate [grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants). To view private images, you need to include authentication when calling this operation. The response includes both private and public images.   <<LB>>  ---   - __CLI__.      ```     linode-cli images list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetImages200Response**](get_images_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_image

> models::GetImages200ResponseDataInner post_image(api_version, post_image_request)
Create an image

Captures a private, gold-master image from a Linode disk.  > 📘 > > - Captured images are stored using our Object Storage service. The `region` where the target image exists determines where the [resulting image is stored](https://techdocs.akamai.com/cloud-computing/docs/images#regions-and-captured-custom-images). > > - If you create an image from an encrypted disk, the API doesn't encrypt the image. When you rebuild a compute instance from that image, the resulting disk will be automatically encrypted.   <<LB>>  ---   - __CLI__.      ```     linode-cli images create \\   --label this_is_a_label \\   --description \"A longer description \\     of the image\" \\   --disk_id 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_write linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_image_request** | Option<[**PostImageRequest**](PostImageRequest.md)> |  |  |

### Return type

[**models::GetImages200ResponseDataInner**](get_images_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_replicate_image

> models::GetImages200ResponseDataInner post_replicate_image(api_version, image_id, post_replicate_image_request)
Replicate an image

Target an existing image and replicate it to another compute region.  - This operation is in Limited Availability. Talk to your account team about access to it.  - This is only available in a `region` that supports Object Storage, which stores the replicated image. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to review a region's `capabilities`.  - To replicate an image, it needs to have a `status` of `available`. Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation to view an image's `status`.  - To also keep the target image in its original compute region, you need to include that `region` in the request's data. If you leave it out, the API removes the image from that region. Run the [Get an image](https://techdocs.akamai.com/linode-api/reference/get-image) operation to see the `regions` where an image currently exists.  - You can't include an empty array to delete all images. You need to provide at least one compute `region` where the image is `available`. Use the [Delete an image](https://techdocs.akamai.com/linode-api/reference/delete-image) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli images replicate private/12345 \\   --regions \"us-mia\" \\   --regions \"us-east\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**image_id** | **String** | The unique identifier assigned to the image after creation. | [required] |
**post_replicate_image_request** | [**PostReplicateImageRequest**](PostReplicateImageRequest.md) |  | [required] |

### Return type

[**models::GetImages200ResponseDataInner**](get_images_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_upload_image

> models::PostUploadImage200Response post_upload_image(api_version, post_upload_image_request)
Upload an image

Creates a new private image container and returns a URL as the `upload_to` object in the response. Use this URL to upload your own disk image to the container.  1. Ensure the disk image is raw disk image (`.img`) format.  2. Compress the disk image using gzip (`.gz`) format. Compressed, the file can be up to 5 GB and decompressed it can be up to 6 GB.  3. Upload the file in a separate PUT request that includes the `Content-type: application/octet-stream` header:    ```   curl -v \\     -H \"Content-Type: application/octet-stream\" \\     --upload-file example.img.gz \\     $UPLOAD_URL \\     --progress-bar \\     --output /dev/null   ```  > 📘 > > - You need to upload image data within 24 hours of creation or the API cancels the upload and deletes the image container. > > - Only core regions that support our [Object Storage](https://techdocs.akamai.com/cloud-computing/reference/how-to-choose-a-data-center#product-availability) service can store an uploaded image. > > - If you create an image from an encrypted disk, the API doesn't encrypt the image. When you rebuild a compute instance from that image, the resulting disk will be automatically encrypted. > > - You can create a new image and upload image data using a single process through [Cloud Manager](https://www.linode.com/docs/products/tools/images/guides/upload-an-image/#uploading-an-image-file-through-the-cloud-manager) or the [Linode CLI](https://www.linode.com/docs/products/tools/images/guides/upload-an-image/#uploading-an-image-file-through-the-linode-cli).   <<LB>>  ---   - __CLI__.      ```     # Run the operation to just get the upload_to URL linode-cli images upload \\   --description \"Optional details about the Image\" \\   --label \"Example Image\" \\   --region us-east  # Upload the image file in a single step linode-cli image-upload \\   --description \"Optional details about the Image\" \\   --label \"Example Image\" \\   --region us-east \\   /path/to/image-file.img.gz     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_upload_image_request** | Option<[**PostUploadImageRequest**](PostUploadImageRequest.md)> | The uploaded image details. |  |

### Return type

[**models::PostUploadImage200Response**](post_upload_image_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_image

> models::GetImages200ResponseDataInner put_image(api_version, image_id, put_image_request)
Update an image

Updates a private image that you have permission to `read_write`.  > 📘 > > You can't update the `regions` with this operation. Use the [Replicate an image](https://techdocs.akamai.com/linode-api/reference/post-replicate-image) operation to modify the existing regions for your image.   <<LB>>  ---   - __CLI__.      ```     linode-cli images update private/12345 \\   --label \"My gold-master image\" \\   --description \"The detailed description \\     of my image.\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     images:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**image_id** | **String** | The unique identifier assigned to the image after creation. | [required] |
**put_image_request** | [**PutImageRequest**](PutImageRequest.md) |  | [required] |

### Return type

[**models::GetImages200ResponseDataInner**](get_images_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

