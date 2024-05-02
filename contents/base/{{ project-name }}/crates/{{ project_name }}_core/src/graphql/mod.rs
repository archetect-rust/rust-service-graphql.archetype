mod cart;
mod my_obj;
mod roots;
mod schema;

{%- for appliction_key in applications %}
{%- set application = applications[application_key] %}
{%- for entity_key in application.model.entities %}
{%- set entity = application.model.entities[entity_key] %}
pub mod {{ entity["entity_name"] }};
{%- endfor %}
{%- endfor %}


pub use roots::QueryRoot;
pub use schema::create_schema;
pub use cart::Cart;
pub use my_obj::MyObj;

{%- for appliction_key in applications %}
{%- set application = applications[application_key] %}
{%- for entity_key in application.model.entities %}
{%- set entity = application.model.entities[entity_key] %}
pub mod {{ entity["entity_name"] }}::{{ entity["EntityName"] }};
{%- endfor %}
{%- endfor %}
