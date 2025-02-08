# PostManagedServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | The URL at which this Service is monitored. URL parameters such as `?no-cache=1` are preserved. URL fragments/anchors such as `#monitor` are __not__ preserved. | 
**body** | Option<**String**> | What to expect to find in the response body for the Service to be considered up. | [optional]
**consultation_group** | Option<**String**> | The group of ManagedContacts who should be notified or consulted with when an Issue is detected. | [optional]
**created** | Option<**String**> | __Read-only__ When this Managed Service was created. | [optional][readonly]
**credentials** | Option<**Vec<i32>**> | An array of ManagedCredential IDs that should be used when attempting to resolve issues with this Service. | [optional]
**id** | Option<**i32**> | __Read-only__ This Service's unique ID. | [optional][readonly]
**label** | **String** | The label for this Service. This is for display purposes only. | 
**notes** | Option<**String**> | Any information relevant to the Service that Linode special forces should know when attempting to resolve Issues. | [optional]
**region** | Option<**String**> | The Region in which this Service is located. This is required if address is a private IP, and may not be set otherwise. | [optional]
**service_type** | **String** | How this Service is monitored. | 
**status** | Option<**String**> | __Read-only__ The current status of this Service. | [optional][readonly]
**timeout** | **i32** | How long to wait, in seconds, for a response before considering the Service to be down. | 
**updated** | Option<**String**> | __Read-only__ When this Managed Service was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


