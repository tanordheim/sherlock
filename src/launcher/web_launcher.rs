

#[derive(Clone, Debug)]
pub struct Web{
    pub alias: Option<String>,
    pub method: String ,
    pub name: String,
    pub engine: String,
    pub priority: u32,
}

