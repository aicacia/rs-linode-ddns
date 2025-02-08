use std::{io, str::FromStr, sync::Arc, time::Duration};

use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{core::config::Config, service::ddns::ddns};

#[cfg(feature = "completions")]
use super::completions;
use super::opt::{Command, Opt};

pub async fn run(opt: Opt) -> io::Result<()> {
  let config = match Config::new(&opt.config).await {
    Ok(c) => Arc::new(c),
    Err(e) => {
      log::error!("Error parsing Config: {}", e);
      return Err(io::Error::new(io::ErrorKind::Other, e));
    }
  };

  let level = tracing::Level::from_str(&config.log_level).unwrap_or(tracing::Level::DEBUG);
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
          "{}={level}",
          env!("CARGO_PKG_NAME").replace("-", "_"),
          level = level.as_str().to_lowercase()
        )
        .into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  match opt.command {
    Command::Run { domains } => {
      run_command(config, domains).await?;
    }
    Command::Watch {
      domains,
      interval_in_seconds,
    } => {
      let cancellation_token = CancellationToken::new();

      let serve_handle = tokio::spawn(run_forever_command(
        config,
        domains,
        interval_in_seconds,
        cancellation_token.clone(),
      ));

      shutdown_signal(cancellation_token).await;

      match serve_handle.await {
        Ok(_) => {}
        Err(e) => {
          log::error!("Error: {}", e);
        }
      }
    }
    #[cfg(feature = "completions")]
    Command::Completions { shell } => completions::run(shell),
  }
  Ok(())
}

pub async fn run_command(config: Arc<Config>, domains: Option<Vec<String>>) -> io::Result<()> {
  ddns(config.as_ref(), domains.as_ref().map(Vec::as_slice)).await
}

pub async fn run_forever_command(
  config: Arc<Config>,
  domains: Option<Vec<String>>,
  interval_in_seconds: u64,
  cancellation_token: CancellationToken,
) -> io::Result<()> {
  ddns(config.as_ref(), domains.as_ref().map(Vec::as_slice)).await?;
  loop {
    tokio::select! {
      _ = cancellation_token.cancelled() => {
        log::info!("Shutting down");
        return Ok(());
      }
      _ = tokio::time::sleep(Duration::from_secs(interval_in_seconds)) => {
        ddns(config.as_ref(), domains.as_ref().map(Vec::as_slice)).await?;
      }
    }
  }
}

async fn shutdown_signal(cancellation_token: CancellationToken) {
  let ctrl_c = async { tokio::signal::ctrl_c().await };

  #[cfg(unix)]
  let terminate = async {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?
      .recv()
      .await
    {
      Some(_) => Ok(()),
      None => Ok(()),
    }
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  let result = tokio::select! {
    result = ctrl_c => result,
    result = terminate => result,
  };

  match result {
    Ok(_) => log::info!("Shutdown signal received, shutting down"),
    Err(e) => log::error!("Error receiving shutdown signal: {}", e),
  }

  cancellation_token.cancel();
}
