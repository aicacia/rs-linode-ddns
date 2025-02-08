# GetLkeClusters200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**control_plane** | Option<[**models::GetLkeClusters200ResponseDataInnerControlPlane**](get_lke_clusters_200_response_data_inner_control_plane.md)> |  | [optional]
**created** | Option<**String**> | __Read-only__ When this Kubernetes cluster was created. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This Kubernetes cluster's unique ID. | [optional][readonly]
**k8s_version** | Option<**String**> | __Filterable__ The desired Kubernetes version for this Kubernetes cluster in the format of &lt;major&gt;.&lt;minor&gt;, and the latest supported patch version will be deployed. | [optional]
**label** | Option<**String**> | __Filterable__ This Kubernetes cluster's unique label for display purposes only. Labels have the following constraints:    - UTF-8 characters will be returned by the API using escape sequences of their Unicode code points. For example, the Japanese character _か_ is 3 bytes in UTF-8 (`0xE382AB`). Its Unicode code point is 2 bytes (`0x30AB`). APIv4 supports this character and the API will return it as the escape sequence using six 1 byte characters which represent 2 bytes of Unicode code point (`\"\\u30ab\"`).    - 4 byte UTF-8 characters are not supported.    - If the label is entirely composed of UTF-8 characters, the API response will return the code points using up to 193 1 byte characters. | [optional]
**region** | Option<**String**> | __Filterable__ This Kubernetes cluster's location. | [optional]
**tags** | Option<**Vec<String>**> | __Filterable__ An array of tags applied to the Kubernetes cluster. Tags are for organizational purposes only. | [optional]
**tier** | Option<**String**> | __Beta__, __Filterable__ The desired Kubernetes tier.  > 🚧 > > This field is in beta and only works when using the beta API. Call the URL with the `apiVersion` path parameter set to `v4beta`. | [optional]
**updated** | Option<**String**> | __Read-only__ When this Kubernetes cluster was updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


