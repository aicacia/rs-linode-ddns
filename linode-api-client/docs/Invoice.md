# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**billing_source** | Option<**String**> | __Filterable__, __Read-only__ `akamai`: This Invoice was generated according to the terms of an agreement between the customer and Akamai.  `linode`: This Invoice was generated according to the default terms, prices, and discounts. | [optional][readonly]
**date** | Option<**String**> | __Filterable__, __Read-only__ When this Invoice was generated. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The Invoice's unique ID. | [optional][readonly]
**label** | Option<**String**> | __Filterable__, __Read-only__ The Invoice's display label. | [optional][readonly]
**subtotal** | Option<**f64**> | __Read-only__ The amount of the Invoice before taxes in US Dollars. | [optional][readonly]
**tax** | Option<**f64**> | __Read-only__ The amount of tax levied on the Invoice in US Dollars. | [optional][readonly]
**tax_summary** | Option<[**Vec<models::GetInvoices200ResponseDataInnerTaxSummaryInner>**](get_invoices_200_response_data_inner_tax_summary_inner.md)> | __Read-only__ The amount of tax broken down into subtotals by source. | [optional][readonly]
**total** | Option<**f64**> | __Filterable__, __Read-only__ The amount of the Invoice after taxes in US Dollars. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


