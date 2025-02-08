# GetNodeBalancerConfigNodes200ResponseDataInner

## Enum Variants

| Name | Description |
|---- | -----|
| TcpHttpOrHttpsConfig | A NodeBalancer node represents a single backend serving requests for a single port of a NodeBalancer.  Nodes are specific to NodeBalancer configs, and serve traffic over their private IP.  If the same Linode is serving traffic for more than one port on the same NodeBalancer, one NodeBalancer node is required for each config (port) it should serve requests on.  For example, if you have four backends, and each should respond to both HTTP and HTTPS requests, you will need two NodeBalancer configs (port 80 and port 443) and four backends each, one for each of the Linodes serving requests for that port. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


