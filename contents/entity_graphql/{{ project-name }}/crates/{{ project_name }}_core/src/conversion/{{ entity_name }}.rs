use async_graphql::ID;
use crate::{ConvertFrom, ConvertTo, graphql, proto};

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

impl ConvertFrom<proto::Get{{ EntityName | pluralize }}Response> for graphql::{{ EntityName | pluralize }}Page {
    fn convert_from(value: proto::Get{{ EntityName | pluralize }}Response) -> Self {
        let records = value.records.into_iter()
            .map(proto::{{ EntityName }}::convert_to)
            .collect();
        graphql::{{ EntityName | pluralize }}Page {
            records,
            index: value.index,
            next: value.next,
            has_next: value.has_next,
            previous: value.previous,
            has_previous: value.has_previous,
            total: value.total,
            total_records: value.total_records,
        }
    }
}