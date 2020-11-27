use async_std::net::TcpStream;
use quaint::{prelude::*, single::Quaint};
use tiberius::{Client, Config};

static CONN_STR: &str = "jdbc:sqlserver://localhost:1433;database=master;user=SA;password=<YourStrong@Passw0rd>;trustServerCertificate=true;";

static QUERY: &str = r#"
    DECLARE @now datetime;
    SET @now = GETDATE();
    EXEC [USPSampleRequestStateUpdate]
        @SampleRequestNumber = 2533055,
        @RequestTimeStamp = @now,
        @ToState = 'Cancelled',
        @Comment = 'Stop Sampling Orchard Hazard added'
"#;

async fn t_query() -> anyhow::Result<()> {
    let config = Config::from_jdbc_string(CONN_STR)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let res = client.query(QUERY, &[]).await?.into_results().await?;

    dbg!(res);

    Ok(())
}

async fn q_query() -> anyhow::Result<()> {
    let conn = Quaint::new(CONN_STR).await?;
    let res = conn.query_raw(QUERY, &[]).await?;

    dbg!(res);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    t_query().await?;
    q_query().await?;
    Ok(())
}
