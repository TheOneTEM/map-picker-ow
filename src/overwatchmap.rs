use core::fmt;
use crate::gamemode::GameMode;

#[derive(Clone, PartialEq)]
pub struct OWMap {
    pub name: String,
    pub game_mode: GameMode,
}
impl OWMap {
    #[allow(clippy::ptr_arg)]
    pub fn from_vec_string(v: &Vec<String>) -> Self {
        if v.len() != 2 {
            panic!("Vec length mismatch while converting Vec<String> to OWMap. Expected 2, got {}", v.len())
        }
        OWMap { name: v[0].clone(), game_mode: GameMode::from_string(v[1].clone()).expect("Bad gamemode") }
    }
}
impl fmt::Display for OWMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} ({})", self.name, self.game_mode)
    }
}