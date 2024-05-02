{%- for application_key in applications %}
{%- set application = applications[application_key] %}
tonic::include_proto!("{{ application['project_name'] }}");
{%- endfor %}