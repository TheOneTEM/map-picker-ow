use core::fmt;

#[derive(Clone, PartialEq)]
pub enum GameMode {
    Assault,
    Escort,
    Hybrid,
    Flashpoint, 
    Push,
    Clash,
    Control,
}
impl GameMode {
    #[inline]
    pub fn from_string(s: String) -> Option<GameMode> {
        match s.to_ascii_lowercase() {
            val if val == "assault" => Some(Self::Assault),
            val if  val == "escort" => Some(Self::Escort),
            val if val == "hybrid" => Some(Self::Hybrid),
            val if val == "flashpoint" => Some(Self::Flashpoint),
            val if val == "push" => Some(Self::Push),
            val if val == "clash" => Some(Self::Clash),
            val if val == "control" => Some(Self::Control),
            _ => None
        }
    }
    #[inline]
    #[allow(dead_code, clippy::wrong_self_convention)]
    pub fn is_asymmetric(self) -> bool {
        matches!(self, Self::Assault | Self::Escort | Self::Hybrid)
    }
}
impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Assault => write!(f, "assault (Attack-Defense)"),
            Self::Escort => write!(f, "escort (Attack-Defense)"),
            Self::Hybrid => write!(f, "hybrid (Attack-Defense)"),
            Self::Flashpoint => write!(f, "flashpoint"),
            Self::Push => write!(f, "push"),
            Self::Clash => write!(f, "clash"),
            Self::Control => write!(f, "control"),

        }
    }
}