use colordata::Color;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct Colors {
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub colors: [Color; 16],
}
