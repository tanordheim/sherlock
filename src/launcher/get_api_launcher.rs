#[derive(Clone, Debug)]
pub struct ApiGet{
    pub alias: Option<String>,
    pub method: String,
    pub name: String,
    pub url: String,
    pub key: String,
    pub icon: String,
    pub priority: u32,
}

