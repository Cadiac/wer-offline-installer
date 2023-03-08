use std::env;
use std::fs;

use std::path::PathBuf;
use std::net::SocketAddr;

use axum::{
    body::{Bytes, Full},
    http::header,
    http::StatusCode,
    response::Response,
    routing::post,
    Router,
};

use tower_http::{
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wer=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    init_countries_folder()?;

    tokio::join!(
        serve(mock_backend(), 5901),
        serve(mock_error_reporter(), 5701),
    );

    Ok(())
}

fn init_countries_folder() -> std::io::Result<()> {
    let app_data_path = env::var("APPDATA").unwrap();
    if app_data_path.len() == 0 {
        tracing::error!("Couldn't find target %APPDATA% folder where countries should be copied to.");
        tracing::warn!(r#"Copy the 'countries' folder to "C:\Users\<USERNAME>\AppData\Roaming\Wizards of the Coast\Event Reporter\countries" path manually."#);
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "%APPDATA% folder not found"));
    }

    let cwd = env::current_dir().unwrap();
    let mut src_path = PathBuf::from(cwd);
    src_path.push("countries");

    if !src_path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "source 'countries' folder not found - run this .exe at the same directory!"));
    }

    // %AppData%\Wizards of the Coast\Event Reporter\countries
    let mut dest_path = PathBuf::from(app_data_path);
    dest_path.push("Wizards of the Coast");
    dest_path.push("Event Reporter");
    dest_path.push("countries");

    if !dest_path.exists() {
        fs::create_dir_all(dest_path.clone())?;
    }

    // Copy country files to destination
    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        let entry_path = entry.path();

        if entry_type.is_file() {
            let new_dest = dest_path.join(entry.file_name());
            if !new_dest.exists() {
                fs::copy(entry_path, new_dest.clone())?;
                tracing::info!("{} created.", new_dest.display());
            } else {
                tracing::info!("{} already exists, skipping...", new_dest.display());
            }
        }
    }

    Ok(())
}

fn mock_error_reporter() -> Router {
    Router::new().route("/Orchestration/EventReporter/Async", post(empty_response))
}

fn mock_backend() -> Router {
    Router::new().route("/Orchestration/EventReporter/V2", post(country_response))
}

async fn country_response() -> Response<Full<Bytes>> {
    let xml = r#"<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <GetCountriesAndSubdivisionsResponse xmlns="http://www.wizards.com/Service/2013-03">
            <CountriesAndSubdivisionsRes xlmns="http://www.wizards.com/Service/2013-03">
                <Countries xlmns="http://www.wizards.com/Service/2013-03">
                </Countries>
            </CountriesAndSubdivisionsRes>
        </GetCountriesAndSubdivisionsResponse>
    </s:Body>
</s:Envelope>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/xml"),
        )
        .body(Full::from(xml))
        .unwrap()
}

async fn empty_response() -> Response<Full<Bytes>> {
    let xml = r#"<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
    </s:Body>
</s:Envelope>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/xml"),
        )
        .body(Full::from(xml))
        .unwrap()
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
