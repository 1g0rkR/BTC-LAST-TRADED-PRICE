mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port_number = std::env::var("PORT_NUMBER").unwrap_or("8000".to_owned());

    application::run_application(([0, 0, 0, 0], port_number.parse::<u16>()?)).await?;

    Ok(())
}
