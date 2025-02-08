# ChildAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_since** | Option<**String**> | __Read-only__ The activation date and time for the child account. | [optional][readonly]
**address_1** | Option<**String**> | __Filterable__ First line of this child account's billing address. | [optional]
**address_2** | Option<**String**> | __Filterable__ Second line of this child account's billing address, if applicable. | [optional]
**balance** | Option<**f64**> | __Read-only__ This child account's balance, in US dollars. | [optional][readonly]
**balance_uninvoiced** | Option<**f64**> | __Read-only__ This child account's current estimated invoice in US dollars. This is not your final invoice balance. Transfer charges are not included in the estimate. | [optional][readonly]
**billing_source** | Option<**String**> | __Read-only__ The source of service charges for this account, as determined by its relationship with Akamai. The API returns a value of `external` to describe a child account in a parent-child account environment. | [optional][readonly]
**capabilities** | Option<**Vec<String>**> | __Read-only__ A list of the capabilities the child account supports. | [optional][readonly]
**city** | Option<**String**> | __Filterable__ The city for this child account's billing address. | [optional]
**company** | Option<**String**> | __Filterable__ The company name for the owner of this child account. It can't include any of these characters: `<` `>` `(` `)` `\"` `=`. You can't change this value yourself. We use it to create the proxy users that a parent account uses to access a child account. Talk to your account team if you need to change this value. | [optional]
**country** | Option<**String**> | __Filterable__ The two-letter ISO 3166 country code for this child account's billing address. | [optional]
**credit_card** | Option<[**models::GetChildAccounts200ResponseDataInnerCreditCard**](get_child_accounts_200_response_data_inner_credit_card.md)> |  | [optional]
**email** | Option<**String**> | __Filterable__ The email address of the owner of this child account. | [optional]
**euuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | __Read-only__ An external, unique identifier that Akamai assigned to the child account. | [optional][readonly]
**first_name** | Option<**String**> | __Filterable__ The first name of the owner of this child account. It can't include any of these characters: `<` `>` `(` `)` `\"` `=`. | [optional]
**last_name** | Option<**String**> | __Filterable__ The last name of the owner of this child account. It can't include any of these characters: `<` `>` `(` `)` `\"` `=`. | [optional]
**phone** | Option<**String**> | __Filterable__ The phone number for the owner of this child account. | [optional]
**state** | Option<**String**> | __Filterable__ The state or province for the billing address (`address_1` and `address_2, if applicable`). If in the United States (US) or Canada (CA), this is the two-letter ISO 3166 State or Province code.  __Note__. If this is a US military address, use state abbreviations (AA, AE, AP). | [optional]
**tax_id** | Option<**String**> | The tax identification number for this child account. Use this for tax calculations in some countries. If you live in a country that doesn't collect taxes, ensure this is an empty string (`\"\"`). | [optional]
**zip** | Option<**String**> | __Filterable__ The zip code of this Account's billing address. The following restrictions apply:  - Can only contain ASCII letters, numbers, and hyphens (`-`). - Can't contain more than 9 letter or number characters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


