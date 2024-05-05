use anyhow::Error;
use async_graphql::ID;
use crate::{graphql, proto};
use crate::conversion::ConvertTo;

impl ConvertTo<proto::{{ EntityName }}, anyhow::Error> for graphql::{{ EntityName }} {
    fn convert_to(self) -> Result<proto::{{ EntityName }}, anyhow::Error> {
        Ok(proto::{{ EntityName }} {
            id: self.id.map(|v| v.0),
            {%- for field_key in fields -%}
            {%- set field = fields[field_key] %}
            {{ field["field_name"] }}: self.{{ field["field_name"] }},
            {%- endfor %}
        })
    }
}

impl ConvertTo<graphql::{{ EntityName }}, Error> for proto::{{ EntityName }} {
    fn convert_to(self) -> Result<graphql::{{ EntityName }}, Error> {
        Ok(
            graphql::{{ EntityName }} {
                id: self.id.map(|id| ID::from(id)),
                {%- for field_key in fields -%}
                {%- set field = fields[field_key] %}
                {{ field["field_name"] }}: self.{{ field["field_name"] }},
                {%- endfor %}
            }
        )
    }
}