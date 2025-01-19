use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Filter {
    pub page: Option<usize>,
    pub limit: Option<usize>
}