use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub deleted: Option<bool>,
    #[serde(alias = "type")]
    pub item_type: Option<String>,
    pub by: Option<String>,
    pub time: Option<i64>,
    pub text: Option<String>,
    pub dead: Option<bool>,
    pub parent: Option<i32>,
    pub poll: Option<i32>,
    pub kids: Option<Vec<i32>>,
    pub url: Option<String>,
    pub score: Option<i32>,
    pub title: Option<String>,
    pub parts: Option<Vec<i32>>,
    pub descendants: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub created: i64,
    pub karma: i32,
    pub about: Option<String>,
    pub submitted: Option<Vec<i32>>
}

#[derive(Deserialize, Debug)]
pub struct Updates {
    pub items: Vec<i32>,
    pub profiles: Vec<i32>
}
