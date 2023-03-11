use serde::ser::{Serialize, SerializeStruct};
use std::clone;
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct GameManager{
    board: [[PlayablePiece; 12]; 6],
    turn_queue: Mutex<VecDeque<[u16; 2]>>
}

#[derive(Clone)]
pub struct PlayablePiece{
    name: &'static str,
    position: [u16; 2],

}

impl GameManager {

    pub fn mock_gamemanager() -> GameManager{
        GameManager{
            board: GameManager::setup_board(),
            turn_queue: GameManager::setup_turn_queue(),
        }
    }

    fn setup_board() -> [[PlayablePiece; 12]; 6]{
        let row1 = [PlayablePiece::pawn_mockup([0,0]),PlayablePiece::pawn_mockup([0,1]), PlayablePiece::pawn_mockup([0,2]), PlayablePiece::pawn_mockup([0,3]), PlayablePiece::pawn_mockup([0,4]), PlayablePiece::pawn_mockup([0,5]), PlayablePiece::pawn_mockup([0,6]), PlayablePiece::pawn_mockup([0,7]), PlayablePiece::pawn_mockup([0,8]), PlayablePiece::pawn_mockup([0,9]), PlayablePiece::pawn_mockup([0,10]), PlayablePiece::pawn_mockup([0,11])];
        let row2 = [PlayablePiece::pawn_mockup([1,0]),PlayablePiece::pawn_mockup([1,1]), PlayablePiece::pawn_mockup([1,2]), PlayablePiece::pawn_mockup([1,3]), PlayablePiece::pawn_mockup([1,4]), PlayablePiece::pawn_mockup([1,5]), PlayablePiece::pawn_mockup([1,6]), PlayablePiece::pawn_mockup([1,7]), PlayablePiece::pawn_mockup([1,8]), PlayablePiece::pawn_mockup([1,9]), PlayablePiece::pawn_mockup([1,10]), PlayablePiece::pawn_mockup([1,11])];
        let row3 = [PlayablePiece::pawn_mockup([2,0]),PlayablePiece::pawn_mockup([2,1]), PlayablePiece::pawn_mockup([2,2]), PlayablePiece::pawn_mockup([2,3]), PlayablePiece::pawn_mockup([2,4]), PlayablePiece::pawn_mockup([2,5]), PlayablePiece::pawn_mockup([2,6]), PlayablePiece::pawn_mockup([2,7]), PlayablePiece::pawn_mockup([2,8]), PlayablePiece::pawn_mockup([2,9]), PlayablePiece::pawn_mockup([2,10]), PlayablePiece::pawn_mockup([2,11])];
        let row4 = [PlayablePiece::pawn_mockup([3,0]),PlayablePiece::pawn_mockup([3,1]), PlayablePiece::pawn_mockup([3,2]), PlayablePiece::pawn_mockup([3,3]), PlayablePiece::pawn_mockup([3,4]), PlayablePiece::pawn_mockup([3,5]), PlayablePiece::pawn_mockup([3,6]), PlayablePiece::pawn_mockup([3,7]), PlayablePiece::pawn_mockup([3,8]), PlayablePiece::pawn_mockup([3,9]), PlayablePiece::pawn_mockup([3,10]), PlayablePiece::pawn_mockup([3,11])];
        let row5 = [PlayablePiece::pawn_mockup([4,0]),PlayablePiece::pawn_mockup([4,1]), PlayablePiece::pawn_mockup([4,2]), PlayablePiece::pawn_mockup([4,3]), PlayablePiece::pawn_mockup([4,4]), PlayablePiece::pawn_mockup([4,5]), PlayablePiece::pawn_mockup([4,6]), PlayablePiece::pawn_mockup([4,7]), PlayablePiece::pawn_mockup([4,8]), PlayablePiece::pawn_mockup([4,9]), PlayablePiece::pawn_mockup([4,10]), PlayablePiece::pawn_mockup([4,11])];
        let row6 = [PlayablePiece::pawn_mockup([5,0]),PlayablePiece::pawn_mockup([5,1]), PlayablePiece::pawn_mockup([5,2]), PlayablePiece::pawn_mockup([5,3]), PlayablePiece::pawn_mockup([5,4]), PlayablePiece::pawn_mockup([5,5]), PlayablePiece::pawn_mockup([5,6]), PlayablePiece::pawn_mockup([5,7]), PlayablePiece::pawn_mockup([5,8]), PlayablePiece::pawn_mockup([5,9]), PlayablePiece::pawn_mockup([5,10]), PlayablePiece::pawn_mockup([5,11])];
                return [row1, row2, row3, row4, row5, row6];
    }

    fn setup_turn_queue() -> Mutex<VecDeque<[u16; 2]>> {
        let arr:[[u16; 2]; 6] = [[1,0],[0,1],[0,2],[0,3],[4,0],[5,0]];
        Mutex::new(VecDeque::from(arr))
    }

    pub fn get_board(&self) -> &[[PlayablePiece; 12]; 6] {
        return & self.board;
    }

    pub fn get_turn_owner(&self) -> [u16; 2]{
        let mut guard = self.turn_queue.lock().unwrap();
        let vec = &mut *guard;
        let value = vec.front().unwrap().clone();
        return value;
    }

    pub fn pass_turn(&self){
        let mut guard = self.turn_queue.lock().unwrap();
        let vec = &mut *guard;

        let front = vec.pop_front().unwrap();
        vec.push_back(front);
        println!("passing turn");
    }
}

impl PlayablePiece {
    pub fn pawn_mockup(pos: [u16;2]) -> PlayablePiece{
        PlayablePiece { name: "Pawn", position:pos }
    }

    pub fn _knight_mockup(pos: [u16;2]) -> PlayablePiece{
        PlayablePiece { name: "Kinght", position:pos  }
    }

    pub fn _blank_mockup(pos: [u16;2]) -> PlayablePiece{
        PlayablePiece { name: "Blank", position:pos  }
    }

    pub fn _print_name(&self) {
        println!("This piece name is {}", self.name.clone())
    }
}


impl Serialize for GameManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
            let mut state = serializer.serialize_struct("GameManager", 2)?;
            state.serialize_field("board", &self.board)?;
            state.serialize_field("turn_queue", &self.turn_queue)?;
            state.end()
    }
}

impl Serialize for PlayablePiece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("PlayablePiece", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("position", &self.position)?;
        state.end()
    }
}

