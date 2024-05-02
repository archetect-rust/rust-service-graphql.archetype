use std::env;
use std::path::PathBuf;

{%- for application_key in applications %}
{%- set application = applications[application_key] %}
const {{ application['PROJECT_NAME'] }}_PROTO: &str = "../../specs/clients/{{ application['project_name'] }}.proto";
{%- endfor %}
const CLIENTS_DIR: &str = "../../specs/clients";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}

    println!("cargo:rerun-if-changed={}", {{ application['PROJECT_NAME'] }}_PROTO);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("{{ application['project_name'] }}.bin"))
        .build_server(false)
        .build_client(true)
        .compile(&[{{ application['PROJECT_NAME'] }}_PROTO],
                 &[CLIENTS_DIR]
        )
        .unwrap();
    {%- endfor %}

    Ok(())
}