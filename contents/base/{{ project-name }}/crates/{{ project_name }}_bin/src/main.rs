use anyhow::Result;

use {{ project_name }}_core::{{ ProjectName }}Core;{% if persistence != "None" %}
use {{ project_name }}_persistence::{{ ProjectName }}Persistence;{% endif %}
use {{ project_name }}_server::{{ ProjectName }}Server;

mod cli;
mod settings;
mod traces;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = cli::arg_matches();
    let {% if persistence != "None" %}mut {% endif %}settings = settings::Settings::new(&args)?;
    traces::init(settings.tracing())?;

    match args.subcommand() {{'{'}}{% if persistence != "None" %}
        Some(("migrate", args)) => match args.subcommand() {
            Some(("up", _args)) => {
                // Don't migrate automatically
                settings.persistence_mut().set_migrate(Some(false));
                {{ ProjectName }}Persistence::builder()
                    .with_settings(settings.persistence())
                    .build()
                    .await?
                    .migrate_up(None)
                    .await?;
            }
            Some(("down", args)) => {
                let steps = if args.get_flag("all") { None } else { Some(1) };
                // Don't migrate automatically
                settings.persistence_mut().set_migrate(Some(false));
                {{ ProjectName }}Persistence::builder()
                    .with_settings(settings.persistence())
                    .build()
                    .await?
                    .migrate_down(steps)
                    .await?;
            }
            _ => unreachable!(),
        },{% endif %}
        Some(("schema", _args)) => {
            let schema = {{ project_name }}_core::graphql::create_schema()
                .finish()
                .sdl()
                ;
            println!("{schema}");
        },
        Some(("config", args)) => match args.subcommand() {
            Some(("defaults", _)) => settings::Settings::default().print()?,
            Some(("merged", _)) => settings.print()?,
            Some(("generate", _)) => settings.generate()?,
            _ => unreachable!(),
        },
        Some((_command, _args)) => {
            unreachable!()
        }
        None => {
            tracing::info!("Initializing...");{% if persistence != "None" %}
            let persistence = {{ ProjectName }}Persistence::builder()
                .with_settings(settings.persistence())
                .build()
                .await?;{% endif %}
            let core = {{ ProjectName }}Core::builder({% if persistence != "None" %}persistence{% endif %})
                .with_settings(settings.core())
                .build()
                .await?;
            let server = {{ ProjectName }}Server::builder(core)
                .with_settings(settings.server())
                .build()
                .await?;

            tokio::select! {
                result = server.serve() => {
                  return result;
                },
                _ = tokio::signal::ctrl_c() => {
                    return Ok(());
                },
            }
        }
    }

    Ok(())
}
