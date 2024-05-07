use async_graphql::ID;
use crate::{ConvertFrom, graphql, proto};

impl ConvertFrom<graphql::{{ EntityName }}> for proto::{{ EntityName }} {
    fn convert_from(value: graphql::{{ EntityName }}) -> Self {
        proto::{{ EntityName }} {
            id: value.id.map(|v| v.0),
            {%- for field_key in fields -%}
            {%- set field = fields[field_key] %}
            {{ field["field_name"] }}: value.{{ field["field_name"] }},
            {%- endfor %}
        }
    }
}

impl ConvertFrom<graphql::{{ EntityName }}Input> for proto::{{ EntityName }} {
    fn convert_from(value: graphql::{{ EntityName }}Input) -> Self {
        proto::{{ EntityName }} {
            id: value.id.map(|v| v.0),
            {%- for field_key in fields -%}
            {%- set field = fields[field_key] %}
            {{ field["field_name"] }}: value.{{ field["field_name"] }},
            {%- endfor %}
        }
    }
}

impl ConvertFrom<proto::{{ EntityName }}> for graphql::{{ EntityName }} {
    fn convert_from(value: proto::{{ EntityName }}) -> Self {
        graphql::{{ EntityName }} {
            id: value.id.map(|id| ID::from(id)),
            {%- for field_key in fields -%}
            {%- set field = fields[field_key] %}
            {{ field["field_name"] }}: value.{{ field["field_name"] }},
            {%- endfor %}
        }
    }
}