use anyhow::Result;

pub enum Provider {
    Ollama,
}
impl TryFrom<String> for Provider {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "ollama" => Ok(Provider::Ollama),
            _ => anyhow::bail!("Invalid provider"),
        }
    }
}

pub struct Config {
    pub provider: Provider,
    pub model: String,
}
impl Config {
    pub fn init() -> Result<Self> {
        Ok(Self {
            provider: std::env::var("PROVIDER")?.try_into()?,
            model: std::env::var("MODEL")?,
        })
    }
}
