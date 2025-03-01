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

/// PostCloneLinodeInstanceRequestPlacementGroup : Include this to assign this Linode to an existing [placement group](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/). Consider these points when cloning:  - If the Linode you're cloning exists in a placement group, the API won't automatically add the cloned instance to the same placement group. You need to specify a placement group to add the clone to. - The target placement group needs to be in the same `region` set for this Linode. - The placement group needs to have capacity. Run the [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region) operation and store the `maximum_linodes_per_pg` value to know the Linode limit per placement group. You can then run the [Get a placement group](https://techdocs.akamai.com/linode-api/reference/get-placement-group) operation to review the Linodes in that group.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCloneLinodeInstanceRequestPlacementGroup {
    /// The placement group's ID. You need to provide it for all operations that affect it.
    #[serde(rename = "id")]
    pub id: i32,
}

impl PostCloneLinodeInstanceRequestPlacementGroup {
    /// Include this to assign this Linode to an existing [placement group](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/). Consider these points when cloning:  - If the Linode you're cloning exists in a placement group, the API won't automatically add the cloned instance to the same placement group. You need to specify a placement group to add the clone to. - The target placement group needs to be in the same `region` set for this Linode. - The placement group needs to have capacity. Run the [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region) operation and store the `maximum_linodes_per_pg` value to know the Linode limit per placement group. You can then run the [Get a placement group](https://techdocs.akamai.com/linode-api/reference/get-placement-group) operation to review the Linodes in that group.
    pub fn new(id: i32) -> PostCloneLinodeInstanceRequestPlacementGroup {
        PostCloneLinodeInstanceRequestPlacementGroup {
            id,
        }
    }
}

