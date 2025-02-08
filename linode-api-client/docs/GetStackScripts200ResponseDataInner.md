# GetStackScripts200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Read-only__ The date this StackScript was created. | [optional][readonly]
**deployments_active** | Option<**i32**> | __Read-only__ Count of currently active, deployed Linodes created from this StackScript. | [optional][readonly]
**deployments_total** | Option<**i32**> | __Filterable__, __Read-only__ The total number of times this StackScript has been deployed. | [optional][readonly]
**description** | Option<**String**> | __Filterable__ A description for the StackScript. | [optional]
**id** | Option<**i32**> | __Read-only__ The unique ID of this StackScript. | [optional][readonly]
**images** | Option<**Vec<String>**> | An array of Image IDs. These are the Images that can be deployed with this StackScript.  `any/all` indicates that all available Images, including private Images, are accepted. | [optional]
**is_public** | Option<**bool**> | __Filterable__ This determines whether other users can use your StackScript. __Once a StackScript is made public, it cannot be made private.__ | [optional]
**label** | Option<**String**> | __Filterable__ The StackScript's label is for display purposes only. | [optional]
**mine** | Option<**bool**> | __Filterable__, __Read-only__ Returns `true` if this StackScript is owned by the account of the user making the request, and the user making the request is unrestricted or has access to this StackScript. | [optional][readonly]
**rev_note** | Option<**String**> | __Filterable__ This field allows you to add notes for the set of revisions made to this StackScript. | [optional]
**script** | Option<**String**> | The script to execute when provisioning a new Linode with this StackScript. | [optional]
**updated** | Option<**String**> | __Read-only__ The date this StackScript was last updated. | [optional][readonly]
**user_defined_fields** | Option<[**Vec<models::GetStackScripts200ResponseDataInnerUserDefinedFieldsInner>**](get_stack_scripts_200_response_data_inner_user_defined_fields_inner.md)> | __Read-only__ This is a list of fields defined with a special syntax inside this StackScript that allow for supplying customized parameters during deployment. See [Declare User-Defined Fields (UDFs)](https://www.linode.com/docs/products/tools/stackscripts/guides/write-a-custom-script/#declare-user-defined-fields-udfs) for more information. | [optional][readonly]
**user_gravatar_id** | Option<**String**> | __Read-only__ The Gravatar ID for the User who created the StackScript. | [optional][readonly]
**username** | Option<**String**> | __Read-only__ The User who created the StackScript. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


