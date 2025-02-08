# PutAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_promotions** | Option<[**Vec<models::GetAccount200ResponseActivePromotionsInner>**](get_account_200_response_active_promotions_inner.md)> |  | [optional][readonly]
**active_since** | Option<**String**> | __Read-only__ The date and time the account was activated. | [optional][readonly]
**address_1** | Option<**String**> | The first line of this account's billing address. | [optional]
**address_2** | Option<**String**> | The second line of this account's billing address. | [optional]
**balance** | Option<**f64**> | __Read-only__ This account's balance, in US dollars. | [optional][readonly]
**balance_uninvoiced** | Option<**f64**> | __Read-only__ This account's current estimated invoice in US dollars. This is not your final invoice balance. Transfer charges are not included in the estimate. | [optional][readonly]
**billing_source** | Option<**String**> | __Read-only__ The source of service charges for this account. Accounts that are associated with Akamai-specific customers return a value of `akamai`. All other accounts return a value of `linode`. | [optional][readonly]
**capabilities** | Option<**Vec<String>**> | __Read-only__ The Akamai Cloud Computing services your account supports. | [optional][readonly]
**city** | Option<**String**> | The city for this account's `address`. | [optional]
**company** | Option<**String**> | The company name assigned to this account. This value can't include the characters, `<` `>` `(` `)` `\"` `=`. | [optional]
**country** | Option<**String**> | The two-letter ISO 3166 country code for this account's `address`. | [optional]
**credit_card** | Option<[**models::GetAccount200ResponseCreditCard**](get_account_200_response_credit_card.md)> |  | [optional]
**email** | Option<**String**> | The email address of the person assigned to this account. | [optional]
**euuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | __Read-only__ An external unique identifier for this account. | [optional][readonly]
**first_name** | Option<**String**> | The first name of the person assigned to this account. This value can't include the characters, `<` `>` `(` `)` `\"` `=`. | [optional]
**last_name** | Option<**String**> | The last name of the person assigned to this account. This value can't include the characters, `<` `>` `(` `)` `\"` `=`. | [optional]
**phone** | Option<**String**> | The phone number assigned to this account. | [optional]
**state** | Option<**String**> | The state or province for the `address` set for your account, if applicable.  - If the `address` is in the United States (US) or Canada (CA), this is the two-letter ISO 3166 code for the state or province.  - If it's a US military `address`, this is the abbreviation for that territory. This includes `AA` for Armed Forces Americas (excluding Canada), `AE` for Armed Forces Africa, Europe, Middle East, and Canada, or `AP` for Armed Forces Pacific.  - If outside the US or CA, this is the province associated with the account's `address`. | [optional]
**tax_id** | Option<**String**> | The tax identification number assigned to this account, used for tax calculations in a `country` that collects tax. Set to an empty string (`\"\"`) for countries that don't collect tax.  > 📘 > > This value is externally validated. If the validation is successful, a `tax_id_valid` [event](https://techdocs.akamai.com/linode-api/reference/get-events) is triggered. If unsuccessful, a `tax_id_invalid` event is triggered and an error response is issued for an operation that included it. | [optional]
**zip** | Option<**String**> | The zip code for this account's `address`.  - It can only contain ASCII letters, numbers, and dashes (`-`).  - It can't contain more than nine letter or number characters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


