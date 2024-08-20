use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use clap::ArgMatches;
use config::{Config, ConfigError, File, Source, Value};
use serde::{Deserialize, Serialize};

use {{ project_name }}_core::settings::CoreSettings;{% if persistence != "None" %}
use {{ project_name }}_persistence::settings::PersistenceSettings;{% endif %}
use {{ project_name }}_server::settings::ServerSettings;

use crate::traces::TraceFormat;

const DEFAULT_CONFIG_FILE: &str = "{{ project-name }}";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    server: ServerSettings,
    core: CoreSettings,{% if persistence != "None" %}
    persistence: PersistenceSettings,{% endif %}
    tracing: TraceSettings,
}

impl Settings {
    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    pub fn core(&self) -> &CoreSettings {
        &self.core
    }{% if persistence != "None" %}

    pub fn persistence(&self) -> &PersistenceSettings {
        &self.persistence
    }

    pub fn persistence_mut(&mut self) -> &mut PersistenceSettings {
        &mut self.persistence
    }{% endif %}

    pub fn tracing(&self) -> &TraceSettings {
        &self.tracing
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    pub fn generate(&self) -> Result<(), anyhow::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true) // <--------- this
            .create(true)
            .open(format!("{DEFAULT_CONFIG_FILE}.yaml"))?;
        file.write_all(self.to_yaml()?.as_bytes())?;
        Ok(())
    }

    pub fn print(&self) -> Result<(), anyhow::Error> {
        println!("{}", self.to_yaml()?);
        Ok(())
    }
}

impl Settings {
    pub fn new(args: &ArgMatches) -> Result<Self, ConfigError> {
        let config = Config::builder();

        // Load Defaults
        let config = config.add_source(File::from_str(
            Settings::default()
                .to_yaml()
                .map_err(|err| ConfigError::Foreign(Box::new(err)))?
                .as_str(),
            config::FileFormat::Yaml,
        ));

        // Merge Config File from Default Location
        let config = config.add_source(File::with_name(DEFAULT_CONFIG_FILE).required(false));

        // Merge Config File specified from Command Line
        let config = if let Some(config_file) = args.get_one::<PathBuf>("config-file") {
            if let Ok(config_file) = shellexpand::full(config_file.to_str().expect("Valid Config Path")) {
                let config = config.add_source(File::with_name(config_file.as_ref()).required(true));
                config
            } else {
                config
            }
        } else {
            config
        };

        // Merge Environment Variables/Command Line overrides
        let clap_source = ClapSource::builder()
            .map(ArgType::string("database-url"), MapTo::string("persistence.database.url"))
            .map(ArgType::flag("migrate"), MapTo::string("persistence.migrate"))
            .map(ArgType::string("host"), MapTo::string("server.host"))
            .map(ArgType::flag("log-sql"), MapTo::string("persistence.database.log_sql"))

            .map(ArgType::u64("service-port"), MapTo::string("server.service.port"))
            .map(ArgType::flag("temp-db"), MapTo::string("persistence.temporary"))
            .map(ArgType::string("tracing-format"), MapTo::string("tracing.format"))
            .map(ArgType::string("tracing-filter"), MapTo::string("tracing.filter"))
{%- for application_key in applications %}
{%- set application = applications[application_key] %}
            .map(ArgType::string("{{ application['project-name'] }}"), MapTo::string("core.{{ application['project_name'] }}.url"))
{%- endfor %}
            .build(args.clone());
        let config = config.add_source(clap_source);

        let conf = config.build()?;

        conf.try_deserialize()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceSettings {
    format: TraceFormat,
    filter: String,
}

impl TraceSettings {
    pub fn format(&self) -> &TraceFormat {
        &self.format
    }

    pub fn filter(&self) -> &str {
        self.filter.as_str()
    }
}

impl Default for TraceSettings {
    fn default() -> Self {
        TraceSettings {
            format: Default::default(),
            filter: "info".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct ClapSource {
    mappings: HashMap<ArgType, MapTo>,
    matches: ArgMatches,
}

struct ClapBuilder {
    mappings: HashMap<ArgType, MapTo>,
}

impl ClapBuilder {
    fn build(self, matches: ArgMatches) -> ClapSource {
        ClapSource {
            mappings: self.mappings,
            matches,
        }
    }

    fn map(mut self, arg: ArgType, property: MapTo) -> ClapBuilder {
        self.mappings.insert(arg, property);
        self
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ArgType {
    String(String),
    Flag(String),
    U64(String),
}

impl ArgType {
    fn string<T: Into<String>>(arg: T) -> ArgType {
        ArgType::String(arg.into())
    }

    fn flag<T: Into<String>>(arg: T) -> ArgType {
        ArgType::Flag(arg.into())
    }

    fn u64<T: Into<String>>(arg: T) -> ArgType {
        ArgType::U64(arg.into())
    }
}

#[derive(Clone, Debug)]
enum MapTo {
    String(String),
}

impl MapTo {
    fn string<T: Into<String>>(property: T) -> MapTo {
        MapTo::String(property.into())
    }
}

impl ClapSource {
    pub fn builder() -> ClapBuilder {
        ClapBuilder {
            mappings: Default::default(),
        }
    }
}

impl Source for ClapSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut results: HashMap<String, Value> = HashMap::new();
        for (arg_type, map_to) in &self.mappings {
            match map_to {
                MapTo::String(property) => {
                    match arg_type {
                        ArgType::String(arg) => {
                            if let Some(value) = self.matches.get_one::<String>(arg) {
                                results.insert(property.into(), value.clone().into());
                            }
                        }
                        ArgType::Flag(arg) => {
                            if self.matches.get_flag(arg) {
                                results.insert(property.into(), "true".into());
                            }
                        }
                        ArgType::U64(arg) => {
                            if let Some(value) = self.matches.get_one::<i64>(arg) {
                                results.insert(property.into(), (*value).into());
                            }
                        }
                    }
                }
            }
        }
        Ok(results)
    }
}
