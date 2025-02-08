/*
 * Akamai: Linode API
 *
 * Add a Cloud Computing instance so you can build, release, and scale applications faster with virtual machines. 
 *
 * The version of the OpenAPI document: 4.193.0
 * Contact: jperez@linode.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PostAddLinodeConfigRequestAllOfInterfacesInner : The network interface to apply to this Linode's configuration profile.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostAddLinodeConfigRequestAllOfInterfacesInner {
    /// __Read-only__ Returns `true` if the interface is in use, meaning that the Linode has been booted using the configuration profile to which the interface belongs.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// __Read-only__ The unique ID representing this interface.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// IPv4 CIDR VPC subnet ranges that are routed to this interface.  When included in a request:  - A range can't include any addresses that are assigned to an active Linode or another VPC subnet.  - When updating, you need to include any existing ranges to maintain them. If a range is left out, it will be removed.  - Submit this as an empty array removes all existing values.  - Omit this object to leave existing values as is.  <<LB>>  > 📘 > > This is only supported for interfaces with a `purpose` of `vpc`.
    #[serde(rename = "ip_ranges", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Option<Vec<String>>>,
    /// This interface's private IP address in classless inter-domain routing (CIDR) notation.  For interfaces with a `purpose` of `public`:  - If you include this in a request, set it to an empty string (`\"\"`) or `null`.  - Returned as `null` in a response.  For interfaces with a `purpose` of `vlan`:  - To avoid conflicting addresses, make sure this value is unique for each `vlan` interface.  - This should be unique among devices attached to the VLAN to avoid conflict.  - If Network Helper is enabled, the Linode's interface will be automatically configured to use this address after the Linode is rebooted. If Network Helper is disabled, enable the address using [manual static IP configuration](https://www.linode.com/docs/guides/manual-network-configuration/).  For interfaces with a `purpose` of `vpc`:  - If you include this in a request, set it to an empty string (`\"\"`) or `null`.  - Returned as `null` in a response.
    #[serde(rename = "ipam_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ipam_address: Option<Option<String>>,
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<models::PostAddLinodeConfigRequestAllOfInterfacesInnerIpv4>,
    /// __Filterable__ The name of this interface.  For interfaces with a `purpose` of `vlan`:  - Required.  - This needs to be unique among a Linode's interfaces. A Linode can't be attached to the same VLAN multiple times.  - This can only contain ASCII letters, numbers, and dashes (`-`). You can't use two consecutive dashes (`--`).  - If the VLAN label is new, a VLAN is created. Up to 10 VLANs can be created in each data center `region`. To view your active VLANs, run the [List VLANs](https://techdocs.akamai.com/linode-api/reference/get-vlans) operation.  For interfaces with a `purpose` of `public`:  - If you include this in a request, set it to an empty string (`\"\"`) or `null`.  - Returned as `null` in a response.  For interfaces with a `purpose` of `vpc`:  - If you include this in a request, set it to an empty string (`\"\"`) or `null`.  - Returned as `null` in a response.
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<String>>,
    /// The default route to the Linode. Each Linode can have one interface set as its `primary`. If you haven't specifically set a `primary`, the first non-`vlan` type interface is automatically treated as the primary.  > 📘 > > This needs to be set to `false` for any interface that uses `vlan` as its `purpose`.
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// The type of interface. This can be `public`, `vlan`, or `vpc`.  For interfaces with a `purpose` of `public`:  - You can only define one `public` interface per Linode.  - The Linode's default public IPv4 address is assigned to the `public` interface.  - A Linode needs a `public` interface in the first or `eth0` position to be reachable via the public internet, after it boots. If no `public` interface is configured, you can only access the Linode through [LISH](https://www.linode.com/docs/products/compute/compute-instances/guides/lish/), or through another Linode that's connected to the same VLAN or VPC.  For interfaces with a `purpose` of `vlan`:  - Configuring this `purpose` of interface attaches a Linode to the VLAN with the specified `label`.  - If an `ipam_address` is configured, the Linode uses this address.  For interfaces with a `purpose` of `vpc`:  - Configuring this `purpose` of interface attaches a Linode to an existing VPC subnet with the specified `subnet_id`.  - When the interface is activated, the Linode is configured to use an IP address from the range in the assigned VPC subnet. See `ipv4.vpc` for more information.
    #[serde(rename = "purpose")]
    pub purpose: PurposeEnum,
    /// The `id` of the VPC subnet for this interface. Use this value in a request to assign a Linode to a VPC subnet.  - Required for `vpc` type interfaces.  - Returned as `null` for non-`vpc` type interfaces.  - Once you've assigned a VPC subnet to an interface, you can't update it.  - You need to reboot a Linode using the interface's configuration profile to assign the Linode to a VPC subnet.
    #[serde(rename = "subnet_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<Option<i32>>,
    /// __Read-only__ The `id` of the VPC configured for this interface. Returned as `null` for non-`vpc` type interfaces.
    #[serde(rename = "vpc_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<Option<i32>>,
}

impl PostAddLinodeConfigRequestAllOfInterfacesInner {
    /// The network interface to apply to this Linode's configuration profile.
    pub fn new(purpose: PurposeEnum) -> PostAddLinodeConfigRequestAllOfInterfacesInner {
        PostAddLinodeConfigRequestAllOfInterfacesInner {
            active: None,
            id: None,
            ip_ranges: None,
            ipam_address: None,
            ipv4: None,
            label: None,
            primary: None,
            purpose,
            subnet_id: None,
            vpc_id: None,
        }
    }
}
/// The type of interface. This can be `public`, `vlan`, or `vpc`.  For interfaces with a `purpose` of `public`:  - You can only define one `public` interface per Linode.  - The Linode's default public IPv4 address is assigned to the `public` interface.  - A Linode needs a `public` interface in the first or `eth0` position to be reachable via the public internet, after it boots. If no `public` interface is configured, you can only access the Linode through [LISH](https://www.linode.com/docs/products/compute/compute-instances/guides/lish/), or through another Linode that's connected to the same VLAN or VPC.  For interfaces with a `purpose` of `vlan`:  - Configuring this `purpose` of interface attaches a Linode to the VLAN with the specified `label`.  - If an `ipam_address` is configured, the Linode uses this address.  For interfaces with a `purpose` of `vpc`:  - Configuring this `purpose` of interface attaches a Linode to an existing VPC subnet with the specified `subnet_id`.  - When the interface is activated, the Linode is configured to use an IP address from the range in the assigned VPC subnet. See `ipv4.vpc` for more information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PurposeEnum {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "vlan")]
    Vlan,
    #[serde(rename = "vpc")]
    Vpc,
}

impl Default for PurposeEnum {
    fn default() -> PurposeEnum {
        Self::Public
    }
}

