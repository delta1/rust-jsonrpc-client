use anyhow::Result;

#[jsonrpc_client::api]
pub trait Math {
    fn subtract(&self, subtrahend: i64, minuend: i64) -> i64;
}

#[jsonrpc_client::r#impl(Math)]
struct Client {
    inner: reqwest::blocking::Client,
    base_url: reqwest::Url,
}

impl Client {
    fn new(base_url: String) -> Result<Self> {
        Ok(Self {
            inner: reqwest::blocking::Client::new(),
            base_url: base_url.parse()?,
        })
    }

    fn with_path(self, path: &str) -> Result<Self> {
        Ok(Self {
            base_url: self.base_url.join(path)?,
            ..self
        })
    }
}

fn main() -> Result<()> {
    let client = Client::new("http://example-jsonrpc.org/".to_owned())?;
    let client_foobar = client.with_path("foobar")?;

    let _ = client_foobar.subtract(10, 5)?;

    Ok(())
}