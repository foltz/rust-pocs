use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ClientExtensions {
    /// The Client ID of the Order/Trade
    pub id: String,
    /// A tag associated with the Order/Trade
    pub tag: String,
    /// A comment associated with the Order/Trade
    pub comment: String
}
