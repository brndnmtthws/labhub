// This file is auto-generated, do not edit.

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
    pub id: Option<i64>,
    pub status: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub sha: Option<String>,
    pub web_url: Option<String>,
}
