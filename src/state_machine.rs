use crate::OWMap;
pub enum State {
    Team1MapPick,
    Team1HeroBan,
    Team2HeroBan,
    GameInProgress,
    Team2MapPick,
}
pub enum Team {
    Team1,
    Team2
}
pub enum Event {
    MapPick(OWMap),
    HeroBan(Hero),
    GameStart,
    GameEnd(Team),
}

pub fn event_handler(e: Event, s: State) {
    
}