use async_std::net::TcpStream;
use quaint::{prelude::*, single::Quaint};
use tiberius::{AuthMethod, Client, Config, EncryptionLevel};

static QUERY: &str = r#"
    DECLARE @now datetime;
    SET @now = GETDATE();
    EXEC [USPSampleRequestStateUpdate]
        @SampleRequestNumber = 2533055,
        @RequestTimeStamp = @now,
        @ToState = 'Cancelled',
        @Comment = 'Stop Sampling Orchard Hazard added'"#;

async fn t_query() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "<YourStrong@Passw0rd>"));
    config.trust_cert();
    config.encryption(EncryptionLevel::On);

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let res = client
        .query(QUERY, &[&"index_renaming_must_work_when_renaming_to_custom"])
        .await?
        .into_results()
        .await?;

    dbg!(res);

    Ok(())
}

async fn q_query() -> anyhow::Result<()> {
    let conn = Quaint::new("jdbc:sqlserver://localhost:1433;database=master;user=SA;password=<YourStrong@Passw0rd>;trustServerCertificate=true;").await?;

    let res = conn
        .query_raw(QUERY, &["index_renaming_must_work_when_renaming_to_custom".into()])
        .await?;

    dbg!(res);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    t_query().await?;
    q_query().await?;
    Ok(())
}
