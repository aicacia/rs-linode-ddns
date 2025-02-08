# Tcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | Option<**String**> | The algorithm this TCP NodeBalancer uses to route traffic to backends. | [optional][default to Roundrobin]
**check** | Option<**String**> | The type of check to perform against backends to ensure they are serving requests. This is used to determine if backends are up or down.  - If `none`, no check is performed. - `connection` requires only a connection to the backend to succeed. - `http` and `http_body` rely on the backend serving HTTP, and that the response returned matches what is expected. | [optional][default to None]
**check_attempts** | Option<**i32**> | How many times to attempt a check before considering a backend to be down. | [optional][default to 3]
**check_body** | Option<**String**> | This value must be present in the response body of the check in order for it to pass. If this value is not present in the response body of a check request, the backend is considered to be down. | [optional]
**check_interval** | Option<**i32**> | How often, in seconds, to check that backends are up and serving requests.  Must be greater than `check_timeout`. | [optional][default to 5]
**check_passive** | Option<**bool**> | If `true`, any response from this backend with a `5xx` status code will be enough for it to be considered unhealthy and taken out of rotation. | [optional][default to true]
**check_path** | Option<**String**> | The URL path to check on each backend. If the backend does not respond to this request it is considered to be down. | [optional]
**check_timeout** | Option<**i32**> | How long, in seconds, to wait for a check attempt before considering it failed.  Must be less than `check_interval`. | [optional][default to 30]
**cipher_suite** | Option<**String**> | __Read-only__ Not applicable for HTTP configs. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This config's unique ID. | [optional][readonly]
**nodebalancer_id** | Option<**i32**> | __Read-only__ The ID for the NodeBalancer this config belongs to. | [optional][readonly]
**nodes** | [**Vec<models::TcpHttpOrHttpsConfig>**](TCP__HTTP__or_HTTPS_config.md) | The NodeBalancer nodes that serve this configuration. | 
**nodes_status** | Option<[**models::TcpNodesStatus**](TCP_nodes_status.md)> |  | [optional]
**port** | Option<**i32**> | This is the port the NodeBalancer listens on for this configuration. Port numbers must be unique across TCP, HTTP, and HTTPS configurations on a single NodeBalancer. However, ports assigned to TCP, HTTP, or HTTPS configurations can also be reused for UDP configurations. For example, Port 80 can simultaneously serve a TCP and a UDP configuration on the same NodeBalancer, but it can't be shared by both a TCP and an HTTP configuration. Although certain ports are traditionally associated with specific protocols, this isn't strictly enforced, and you may configure your NodeBalancer however you find useful. | [optional][default to 80]
**protocol** | Option<**String**> | The protocol the port is configured to serve, `tcp` in this case.  Review our guide on [Available protocols](https://techdocs.akamai.com/cloud-computing/docs/available-protocols) for information on protocol features. | [optional][default to http]
**proxy_protocol** | Option<**String**> | Proxy protocol is a TCP extension that sends initial TCP connection information such as source or destination IPs and ports to backend devices. Proxy protocol preserves initial TCP information that would be lost otherwise. Backend devices must be configured to work with `proxy_protocol` if enabled.  - If omitted, or set to `none`, the NodeBalancer doesn't send any auxiliary data over TCP connections. This is the default. - If set to `v1`, the human-readable header format (Version 1) is used. Requires `tcp` protocol. - If set to `v2`, the binary header format (Version 2) is used. Requires `tcp` protocol. | [optional][default to None]
**ssl_cert** | Option<**String**> | __Read-only__ Not applicable for TCP configs. | [optional][readonly]
**ssl_commonname** | Option<**String**> | __Read-only__ Not applicable for TCP configs. | [optional][readonly]
**ssl_fingerprint** | Option<**String**> | __Read-only__ Not applicable for TCP configs. | [optional][readonly]
**ssl_key** | Option<**String**> | __Read-only__ Not applicable for HTTP configs. | [optional][readonly]
**stickiness** | Option<**String**> | __Read-only__ Controls how session stickiness is handled on this port.  Not applicable to TCP configurations. | [optional][readonly][default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


