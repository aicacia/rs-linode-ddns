use std::{
  io,
  net::{Ipv4Addr, Ipv6Addr},
};

use linode_api_client::{
  apis::{configuration::Configuration, domains_api, records_api},
  models::{
    get_domain_records_200_response_data_inner, post_domain_record_request, Domain,
    GetDomainRecords200ResponseDataInner, PostDomainRecordRequest, PutDomainRecordRequest,
  },
};

use crate::core::config::Config;

const LINODE_API_VERSION: &str = "v4";

pub async fn ddns(config: &Config, cli_domains: Option<&[String]>) -> io::Result<()> {
  let mut domains = config.domains.clone();

  if let Some(cli_domains) = cli_domains {
    for cli_domain in cli_domains {
      if !domains.contains(cli_domain) {
        domains.push(cli_domain.clone());
      }
    }
  }

  let addr_v4 = public_ip::addr_v4().await;
  let addr_v6 = public_ip::addr_v6().await;
  if addr_v4.is_none() && addr_v6.is_none() {
    return Err(io::Error::new(
      io::ErrorKind::Other,
      "No public IP address found",
    ));
  }

  let configuration = linode_configuration(config);
  let linode_domains = get_domains(&configuration).await?;

  for domain in domains {
    let parts = domain.split('.').collect::<Vec<&str>>();
    let subdomain_name = if parts.len() > 2 {
      Some(parts[0])
    } else {
      None
    };
    let domain_name = if parts.len() > 2 {
      parts[1..].join(".")
    } else {
      domain.to_string()
    };

    if let Some(linode_domain) = linode_domains
      .iter()
      .find(|d| d.domain.as_ref() == Some(&domain_name))
    {
      if let Some(id) = linode_domain.id {
        update_domain_dns(
          &configuration,
          &domain,
          subdomain_name,
          id,
          addr_v4,
          addr_v6,
        )
        .await?;
      }
    }
  }

  Ok(())
}

async fn update_domain_dns(
  configuration: &Configuration,
  full_domain_name: &str,
  subdomain_name: Option<&str>,
  domain_id: i32,
  ipv4: Option<Ipv4Addr>,
  ipv6: Option<Ipv6Addr>,
) -> io::Result<()> {
  log::info!("Checking records for {full_domain_name}");

  let mut ipv4_record = None;
  let mut ipv6_record = None;
  for domain_record in get_domain_records(configuration, domain_id).await? {
    if let Some(subdomain_name) = subdomain_name {
      if Some(subdomain_name) != domain_record.name.as_ref().map(String::as_ref) {
        continue;
      }
    }
    match domain_record.r#type {
      Some(get_domain_records_200_response_data_inner::TypeEnum::A) => {
        ipv4_record = Some(domain_record)
      }
      Some(get_domain_records_200_response_data_inner::TypeEnum::Aaaa) => {
        ipv6_record = Some(domain_record)
      }
      _ => {}
    }
  }
  if let Some(ipv4) = ipv4 {
    if let Some(ipv4_record) = ipv4_record {
      if let Some(record_id) = ipv4_record.id {
        if ipv4_record.target != Some(ipv4.to_string()) {
          update_ip_record(configuration, domain_id, record_id, ipv4.to_string()).await?;
          log::info!("{full_domain_name} A record updated to {ipv4}");
        } else {
          log::info!("{full_domain_name} A record already set to {ipv4}");
        }
      }
    } else {
      create_ipv4_record(configuration, domain_id, ipv4, subdomain_name).await?;
      log::info!("{full_domain_name} A record created for {ipv4}");
    }
  }
  if let Some(ipv6) = ipv6 {
    if let Some(ipv6_record) = ipv6_record {
      if let Some(record_id) = ipv6_record.id {
        if ipv6_record.target != Some(ipv6.to_string()) {
          update_ip_record(configuration, domain_id, record_id, ipv6.to_string()).await?;
          log::info!("{full_domain_name} AAAA record updated to {ipv6}");
        } else {
          log::info!("{full_domain_name} AAAA record already set to {ipv6}");
        }
      }
    } else {
      create_ipv6_record(configuration, domain_id, ipv6, subdomain_name).await?;
      log::info!("{full_domain_name} AAAA record created for {ipv6}");
    }
  }
  Ok(())
}

async fn create_ipv4_record(
  configuration: &Configuration,
  domain_id: i32,
  addr_v4: Ipv4Addr,
  subdomain_name: Option<&str>,
) -> io::Result<()> {
  match records_api::post_domain_record(
    configuration,
    LINODE_API_VERSION,
    domain_id,
    PostDomainRecordRequest {
      name: subdomain_name.map(str::to_owned),
      target: Some(addr_v4.to_string()),
      ttl_sec: Some(30),
      r#type: post_domain_record_request::TypeEnum::A,
      ..Default::default()
    },
  )
  .await
  {
    Ok(_) => Ok(()),
    Err(e) => {
      log::error!("Error: {:?}", e);
      Err(io::Error::new(io::ErrorKind::Other, e))
    }
  }
}

async fn create_ipv6_record(
  configuration: &Configuration,
  domain_id: i32,
  addr_v6: Ipv6Addr,
  subdomain_name: Option<&str>,
) -> io::Result<()> {
  match records_api::post_domain_record(
    configuration,
    LINODE_API_VERSION,
    domain_id,
    PostDomainRecordRequest {
      name: subdomain_name.map(str::to_owned),
      target: Some(addr_v6.to_string()),
      ttl_sec: Some(30),
      r#type: post_domain_record_request::TypeEnum::Aaaa,
      ..Default::default()
    },
  )
  .await
  {
    Ok(_) => Ok(()),
    Err(e) => {
      log::error!("Error: {:?}", e);
      Err(io::Error::new(io::ErrorKind::Other, e))
    }
  }
}

async fn update_ip_record(
  configuration: &Configuration,
  domain_id: i32,
  record_id: i32,
  addr: String,
) -> io::Result<()> {
  match records_api::put_domain_record(
    configuration,
    LINODE_API_VERSION,
    domain_id,
    record_id,
    PutDomainRecordRequest {
      target: Some(addr),
      ttl_sec: Some(30),
      ..Default::default()
    },
  )
  .await
  {
    Ok(_) => Ok(()),
    Err(e) => {
      log::error!("Error: {:?}", e);
      Err(io::Error::new(io::ErrorKind::Other, e))
    }
  }
}

async fn get_domain_records(
  configuration: &Configuration,
  domain_id: i32,
) -> io::Result<Vec<GetDomainRecords200ResponseDataInner>> {
  let mut page = 0;
  let mut results = Vec::new();
  loop {
    match records_api::get_domain_records(
      configuration,
      LINODE_API_VERSION,
      domain_id,
      Some(page),
      Some(25),
    )
    .await
    {
      Ok(response) => {
        if let Some(current_page) = response.page {
          page = current_page + 1;

          if let Some(data) = response.data {
            if data.is_empty() {
              break;
            }
            results.extend(data);
          } else {
            break;
          }
        } else {
          break;
        }
      }
      Err(e) => {
        log::error!("Error: {:?}", e);
        return Err(io::Error::new(io::ErrorKind::Other, e));
      }
    };
  }
  Ok(results)
}

async fn get_domains(configuration: &Configuration) -> io::Result<Vec<Domain>> {
  let mut page = 0;
  let mut results = Vec::new();
  loop {
    match domains_api::get_domains(configuration, LINODE_API_VERSION, Some(page), Some(25)).await {
      Ok(response) => {
        if let Some(current_page) = response.page {
          page = current_page + 1;

          if let Some(data) = response.data {
            if data.is_empty() {
              break;
            }
            results.extend(data);
          } else {
            break;
          }
        } else {
          break;
        }
      }
      Err(e) => {
        log::error!("Error: {:?}", e);
        return Err(io::Error::new(io::ErrorKind::Other, e));
      }
    };
  }
  Ok(results)
}

fn linode_configuration(config: &Config) -> Configuration {
  let mut configuration = Configuration::default();
  configuration.bearer_access_token = Some(config.linode_api_key.to_owned());
  configuration
}
