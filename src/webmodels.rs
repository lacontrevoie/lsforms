
#[derive(Serialize, Deserialize)]
pub struct ClientStatus {
    pub code: i32,
    pub message: String,
}

// Given as GET argument
#[derive(Serialize, Deserialize)]
pub struct OwnToken {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct OwnTokenPost {
    pub token: String,
    pub username: String,
    pub message: String,
    pub stars: Vec<NewStarPost>,
}

#[derive(Serialize, Deserialize)]
pub struct NewStarPost {
    pub startype: i32,
    pub position_x: f32,
    pub position_y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GenericId {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CallbackKey {
    pub callback_key: String,
}
