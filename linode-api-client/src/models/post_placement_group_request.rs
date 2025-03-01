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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostPlacementGroupRequest {
    /// A unique name for the placement group. A placement group `label` has the following constraints:  - It needs to begin and end with an alphanumeric character. - It can only consist of alphanumeric characters, hyphens (`-`), underscores (`_`), or periods (`.`).
    #[serde(rename = "label")]
    pub label: String,
    /// How requests to add future compute instances to your placement group are handled:  - `strict`. Don't add a compute instance if it breaks the grouped-together or spread-apart model set by your `placement_group_type`. For example, with `anti-affinity:local` as your `placement_group_type`, assume it already has three qualifying compute instances on separate hosts, to support the spread-apart model. If a fourth compute instance is assigned that's on the same host as one of the existing three, an error is thrown and the system won't add it. Ensures the placement group stays compliant with your selected model. - `flexible`. Add a new compute instance, even if it breaks the grouped-together or spread-apart model set by your `placement_group_type`. Breaking the model makes the placement group non-compliant. You need to wait for Akamai to move the offending compute instances to make the group compliant again, once the necessary capacity is available in the region. Offers flexibility to add future compute instances if compliance isn't an immediate concern.  > 📘 > > Once you create a placement group, you can't change its `placement_group_policy` setting.
    #[serde(rename = "placement_group_policy")]
    pub placement_group_policy: PlacementGroupPolicyEnum,
    /// How compute instances are distributed in your placement group. A `placement_group_type` using anti-affinity (`anti-affinity:local`) places compute instances in separate hosts, but still in the same region. This best supports the spread-apart model for high availability. A `placement_group_type` using affinity places compute instances physically close together, possibly on the same host. This supports the grouped-together model for low-latency.  > 📘 > > Currently, only `anti_affinity:local` is available for `placement_group_type`.
    #[serde(rename = "placement_group_type")]
    pub placement_group_type: PlacementGroupTypeEnum,
    /// The data center that houses the compute instances you want to add to your placement group. Run the [List Linodes](https://techdocs.akamai.com/linode-api/reference/get-linode-instances) operation to view your compute instances and store the `region` identifier.
    #[serde(rename = "region")]
    pub region: String,
}

impl PostPlacementGroupRequest {
    pub fn new(label: String, placement_group_policy: PlacementGroupPolicyEnum, placement_group_type: PlacementGroupTypeEnum, region: String) -> PostPlacementGroupRequest {
        PostPlacementGroupRequest {
            label,
            placement_group_policy,
            placement_group_type,
            region,
        }
    }
}
/// How requests to add future compute instances to your placement group are handled:  - `strict`. Don't add a compute instance if it breaks the grouped-together or spread-apart model set by your `placement_group_type`. For example, with `anti-affinity:local` as your `placement_group_type`, assume it already has three qualifying compute instances on separate hosts, to support the spread-apart model. If a fourth compute instance is assigned that's on the same host as one of the existing three, an error is thrown and the system won't add it. Ensures the placement group stays compliant with your selected model. - `flexible`. Add a new compute instance, even if it breaks the grouped-together or spread-apart model set by your `placement_group_type`. Breaking the model makes the placement group non-compliant. You need to wait for Akamai to move the offending compute instances to make the group compliant again, once the necessary capacity is available in the region. Offers flexibility to add future compute instances if compliance isn't an immediate concern.  > 📘 > > Once you create a placement group, you can't change its `placement_group_policy` setting.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlacementGroupPolicyEnum {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "flexible")]
    Flexible,
}

impl Default for PlacementGroupPolicyEnum {
    fn default() -> PlacementGroupPolicyEnum {
        Self::Strict
    }
}
/// How compute instances are distributed in your placement group. A `placement_group_type` using anti-affinity (`anti-affinity:local`) places compute instances in separate hosts, but still in the same region. This best supports the spread-apart model for high availability. A `placement_group_type` using affinity places compute instances physically close together, possibly on the same host. This supports the grouped-together model for low-latency.  > 📘 > > Currently, only `anti_affinity:local` is available for `placement_group_type`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlacementGroupTypeEnum {
    #[serde(rename = "anti_affinity:local")]
    AntiAffinityColonLocal,
}

impl Default for PlacementGroupTypeEnum {
    fn default() -> PlacementGroupTypeEnum {
        Self::AntiAffinityColonLocal
    }
}

