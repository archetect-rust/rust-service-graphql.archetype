use std::sync::Arc;
use async_graphql::*;
use async_graphql::http::{GraphiQLSource, GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response, Router};
use axum::extract::State;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::debug;
use {{ project_name }}_core::graphql::{MutationRoot, QueryRoot};
use {{ project_name }}_core::{Auth, Claims, {{ ProjectName }}Core, RequestState};
use crate::settings::ServerSettings;

#[derive(Clone)]
pub struct {{ ProjectName }}Server {
    core: {{ ProjectName }}Core,
    service_port: u16,
    listener: Arc<Mutex<Option<TcpListener>>>,
}

pub struct Builder {
    settings: ServerSettings,
    core: {{ ProjectName }}Core,
}

impl Builder {
    pub fn new(core: {{ ProjectName }}Core) -> Builder {
        Builder {
            settings: ServerSettings::default(),
            core,
        }
    }

    pub fn with_settings(mut self, settings: &ServerSettings) -> Builder {
        self.settings = settings.clone();
        self
    }

    pub fn with_random_port(mut self) -> Builder {
        self.settings.service_mut().set_port(0);
        self
    }

    pub async fn build(self) -> anyhow::Result<{{ ProjectName }}Server> {
        let listener = TcpListener::bind((self.settings.host(), self.settings.service().port())).await?;
        let addr = listener.local_addr()?;

        Ok({{ ProjectName }}Server {
            core: self.core,
            service_port: addr.port(),
            listener: Arc::new(Mutex::new(Some(listener))),
        })
    }
}


impl {{ ProjectName }}Server {
    pub fn builder(core: {{ ProjectName }}Core) -> Builder {
        Builder::new(core)
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let schema = {{ project_name }}_core::graphql::create_schema()
            .finish();

        let router = Router::new()
            .route("/", get(graphiql))
            .route("/graphql", post(graphql_handler))
            .route("/graphiql", get(graphiql))
            .route("/playground", get(playground))
            .with_state(AppState { core: self.core.clone(), schema })
            ;

        let listener = self.listener.lock().await.take().expect("Listener Expected");
        let address = listener.local_addr()?;

        tracing::info!("{{ ProjectName }} starting on {address}");
        tracing::info!("GraphiQL: http://{address}/graphiql");
        tracing::info!("Playground: http://{address}/playground");

        axum::serve(listener, router).await?;

        Ok(())
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    match extract_auth(&headers) {
        Ok(auth) => {
            let core = app_state.core.clone();
            req = req.data(RequestState::new(core, auth));
            app_state.schema.execute(req).await.into()
        }
        Err(server_error) => {
            let response = async_graphql::Response::from_errors(
                vec![server_error]
            );
            GraphQLResponse::from(response)
        }
    }
}

fn extract_auth(headers: &HeaderMap) -> Result<Auth, ServerError> {
    let unauthorized = Err(ServerError::new("UNAUTHORIZED".to_string(), None));

    if let Some(header) = headers.get(AUTHORIZATION) {
        let header = match header.to_str() {
            Ok(header) => header,
            Err(_not_a_string_error) => {
                debug!("'AUTHORIZATION' header is not convertible to a string");
                return unauthorized;
            }
        }.trim();

        if !header
            .to_uppercase()
            .as_str()
            .starts_with("BEARER ")
        {
            debug!("'{header}' is not correctly formatted");
            return unauthorized;
        }

        let header_parts: Vec<&str> = header.splitn(2, ' ').collect();
        if header_parts.len() != 2 {
            debug!("'{header}' is not correctly formatted");
            return unauthorized;
        }

        let token = header_parts[1];
        if let Some(claims) = parse_claims(token) {
            Ok(Auth::new(token, claims))
        } else {
            unauthorized
        }
    } else {
        unauthorized
    }
}

fn parse_claims(token: &str) -> Option<Claims> {
    // This is turning off token validation, relying on the federated gateway to validate the token.
    // In the event we want to validate tokens at each service, we would do this here.
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    validation.validate_aud = false;
    match decode::<Claims>(&token, &key, &validation).map(|token_data| token_data.claims) {
        Ok(claims) => Some(claims),
        Err(error) => {
            debug!("Error parsing {token}: {error}");
            None
        }
    }
}

async fn playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql")
    ))
}

#[derive(Clone)]
struct AppState {
    core: {{ ProjectName }}Core,
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
}
