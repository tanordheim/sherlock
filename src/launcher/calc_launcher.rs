
#[derive(Clone, Debug)]
pub struct Calc{
    pub alias: Option<String>,
    pub method: String ,
    pub name: String,
    pub r#async: bool,
    pub priority: u32,
}

