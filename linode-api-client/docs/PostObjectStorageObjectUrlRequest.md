# PostObjectStorageObjectUrlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_type** | Option<**String**> | The expected `Content-type` header of the request this signed URL will be valid for.  If provided, the `Content-type` header _must_ be sent with the request when this URL is used, and _must_ be the same as it was when the signed URL was created. Required for all methods _except_ `GET` or `DELETE`. | [optional]
**expires_in** | Option<**i32**> | How long this signed URL will be valid for, in seconds.  If omitted, the URL will be valid for 3600 seconds (1 hour). | [optional][default to 3600]
**method** | **String** | The HTTP method allowed to be used with the pre-signed URL. | [default to GET]
**name** | **String** | The name of the object that will be accessed with the pre-signed URL. This object need not exist, and no error will be returned if it doesn't. This behavior is useful for generating pre-signed URLs to upload new objects to by setting the `method` to `PUT`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


