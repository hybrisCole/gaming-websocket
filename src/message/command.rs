#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub payload: String,
}
