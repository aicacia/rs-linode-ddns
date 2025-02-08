use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod acl_configurations_api;
pub mod account_api;
pub mod account_agreements_api;
pub mod account_availability_api;
pub mod account_settings_api;
pub mod account_transfer_api;
pub mod apps_api;
pub mod attachments_api;
pub mod backups_api;
pub mod beta_programs_api;
pub mod bucket_access_api;
pub mod bucket_contents_api;
pub mod buckets_api;
pub mod child_accounts_api;
pub mod client_thumbnail_api;
pub mod clients_api;
pub mod cluster_dashboard_api;
pub mod clusters_api;
pub mod configurations_api;
pub mod contacts_api;
pub mod credentials_api;
pub mod databases_api;
pub mod devices_api;
pub mod disks_api;
pub mod domain_zone_file_api;
pub mod domains_api;
pub mod endpoints_api;
pub mod engines_api;
pub mod entity_transfers_api;
pub mod events_api;
pub mod firewalls_api;
pub mod grants_api;
pub mod ip_addresses_api;
pub mod ipv4_addresses_api;
pub mod ipv6_pools_api;
pub mod ipv6_ranges_api;
pub mod images_api;
pub mod interfaces_api;
pub mod invoices_api;
pub mod issues_api;
pub mod kernels_api;
pub mod keys_api;
pub mod kubeconfigs_api;
pub mod kubernetes_types_api;
pub mod lkeapi_endpoints_api;
pub mod lke_control_plane_acl_api;
pub mod lke_versions_api;
pub mod linode_instances_api;
pub mod linode_settings_api;
pub mod linode_types_api;
pub mod logins_api;
pub mod longview_types_api;
pub mod maintenances_api;
pub mod managed_ssh_keys_api;
pub mod managed_settings_api;
pub mod network_transfer_prices_api;
pub mod network_transfer_statistics_api;
pub mod network_transfers_api;
pub mod node_balancer_types_api;
pub mod node_balancers_api;
pub mod node_pools_api;
pub mod nodes_api;
pub mod notifications_api;
pub mod o_auth_clients_api;
pub mod object_storage_api;
pub mod object_storage_types_api;
pub mod payment_methods_api;
pub mod payments_api;
pub mod personal_access_tokens_api;
pub mod phone_number_api;
pub mod placement_groups_api;
pub mod plans_api;
pub mod preferences_api;
pub mod profile_api;
pub mod promo_credits_api;
pub mod records_api;
pub mod regions_api;
pub mod replies_api;
pub mod ssh_keys_api;
pub mod ssl_certificates_api;
pub mod security_questions_api;
pub mod service_tokens_api;
pub mod service_transfers_api;
pub mod services_api;
pub mod settings_api;
pub mod stack_scripts_api;
pub mod statistics_api;
pub mod subscriptions_api;
pub mod support_tickets_api;
pub mod tlsssl_certificates_api;
pub mod tags_api;
pub mod two_factor_authentication_api;
pub mod types_api;
pub mod users_api;
pub mod vlans_api;
pub mod vpc_subnets_api;
pub mod vpcs_api;
pub mod volume_types_api;
pub mod volumes_api;

pub mod configuration;
