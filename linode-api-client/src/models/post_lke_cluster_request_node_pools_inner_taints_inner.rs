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
pub struct PostLkeClusterRequestNodePoolsInnerTaintsInner {
    /// The Kubernetes taint effect. For `NoSchedule`, `PreferNoSchedule` and `NoExecute` descriptions, see [Kubernetes Taints and Tolerations](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/).
    #[serde(rename = "effect")]
    pub effect: EffectEnum,
    /// The Kubernetes taint key.  - A key can contain alphanumeric characters, dashes (`-`), underscores (`_`), or dots (`.`). Start and end it with an alphanumeric character.  - If the key begins with a DNS subdomain prefix followed by a single slash, for example `example.com/my-app`, the prefix part needs to adhere to [RFC 1123](https://datatracker.ietf.org/doc/html/rfc1123) DNS subdomain restrictions and be a maximum of 253 characters.
    #[serde(rename = "key")]
    pub key: String,
    /// The Kubernetes taint value.  - A key can contain alphanumeric characters, dashes (`-`), underscores (`_`), or dots (`.`). Start and end it with an alphanumeric character.  - Can be specified as an empty string value with `\"\"`.
    #[serde(rename = "value")]
    pub value: String,
}

impl PostLkeClusterRequestNodePoolsInnerTaintsInner {
    pub fn new(effect: EffectEnum, key: String, value: String) -> PostLkeClusterRequestNodePoolsInnerTaintsInner {
        PostLkeClusterRequestNodePoolsInnerTaintsInner {
            effect,
            key,
            value,
        }
    }
}
/// The Kubernetes taint effect. For `NoSchedule`, `PreferNoSchedule` and `NoExecute` descriptions, see [Kubernetes Taints and Tolerations](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EffectEnum {
    #[serde(rename = "NoSchedule")]
    NoSchedule,
    #[serde(rename = "PreferNoSchedule")]
    PreferNoSchedule,
    #[serde(rename = "NoExecute")]
    NoExecute,
}

impl Default for EffectEnum {
    fn default() -> EffectEnum {
        Self::NoSchedule
    }
}

