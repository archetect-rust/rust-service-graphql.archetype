#![allow(non_camel_case_types)]

use std::env;

use anyhow::Result;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::prelude::*;

use crate::settings::TraceSettings;

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum TraceFormat {
    standard,
    json,
    pretty,
}

impl Default for TraceFormat {
    fn default() -> Self {
        TraceFormat::standard
    }
}

pub fn init(settings: &TraceSettings) -> Result<()> {
    let mut filter = EnvFilter::new(settings.filter());
    if let Ok(rust_log) = env::var(EnvFilter::DEFAULT_ENV) {
        filter = filter.add_directive(rust_log.parse()?);
    }

    match settings.format() {
        TraceFormat::standard => {
            tracing_subscriber::registry()
                .with(fmt::layer().with_ansi(atty::is(atty::Stream::Stdout)))
                .with(filter)
                .init();
        }
        TraceFormat::json => {
            tracing_subscriber::registry()
                .with(fmt::layer().json().flatten_event(true))
                .with(filter)
                .init();
        }
        TraceFormat::pretty => {
            tracing_subscriber::registry()
                .with(fmt::layer().pretty().with_ansi(atty::is(atty::Stream::Stdout)))
                .with(filter)
                .init();
        }
    };

    Ok(())
}
