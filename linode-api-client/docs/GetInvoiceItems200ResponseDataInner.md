# GetInvoiceItems200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**f64**> | __Read-only__ The price, in US dollars, of the Invoice Item. Equal to the unit price multiplied by quantity. | [optional][readonly]
**from** | Option<**String**> | __Read-only__ The date the Invoice Item started, based on month. | [optional][readonly]
**label** | Option<**String**> | __Read-only__ The Invoice Item's display label. | [optional][readonly]
**quantity** | Option<**i32**> | __Read-only__ The quantity of this Item for the specified Invoice. | [optional][readonly]
**region** | Option<**String**> | __Read-only__ The ID of the applicable Region associated with this Invoice Item.  `null` if there is no applicable Region. | [optional][readonly]
**tax** | Option<**f64**> | __Read-only__ The amount of tax levied on this Item in US Dollars. | [optional][readonly]
**to** | Option<**String**> | __Read-only__ The date the Invoice Item ended, based on month. | [optional][readonly]
**total** | Option<**f64**> | __Read-only__ The price of this Item after taxes in US Dollars. | [optional][readonly]
**r#type** | Option<**String**> | __Read-only__ The type of service, ether `hourly` or `misc`. | [optional][readonly]
**unit_price** | Option<**String**> | __Read-only__ The monthly service fee in US Dollars for this Item. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


