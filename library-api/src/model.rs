#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub title: String,
    pub description: String,
    pub release_data: String,
    pub genre: String,
    pub rating: String,
    pub thumbnail: String,
    pub cover: String,
}