use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum State {
    Idle,
    Attack,
    Dodge,
}

trait StateHandler {
    fn on_enter(&self) {}
    fn on_exit(&self) {}
    //Those 2 updates will need to be changed so you can pass more args, like characters values and such
    fn on_update(&self) {}
    fn on_physics_update(&self) {}
}

impl StateHandler for State {
    fn on_enter(&self) {
        match self {
            State::Idle => println!("Enter Idle"),
            State::Attack => println!("Enter Attack"),
            State::Dodge => println!("Enter Dodge")
        }
    }
    fn on_exit(&self) {
        match self {
            State::Idle => println!("Exit Idle"),
            State::Attack => println!("Exit Attack"),
            State::Dodge => println!("Exit Dodge")
        }
    }
    fn on_physics_update(&self) {
        match self {
            State::Idle => println!("Physics Update Idle"),
            State::Attack => println!("Physics Update Attack"),
            State::Dodge => println!("Physics Update Dodge")
        }
    }
    fn on_update(&self) {
        match self {
            State::Idle => println!("Update Idle"),
            State::Attack => println!("Update Attack"),
            State::Dodge => println!("Update Dodge")
        }
    }
}

pub struct StateMachine {
    pub current_state: State,
    pub transitions: HashMap<State, Vec<State>>, //Valid transitions
}

impl StateMachine {
    pub fn new(initial_state: State) -> Self {
        Self {
            current_state: initial_state,
            transitions: HashMap::new()
        }
    }
    fn add_transitions(&mut self, from: State, to: State) {
        if let Some(s) = self.transitions.get_mut(&from) {
            if !s.contains(&to) {
                s.push(to);
            }
        } else {
            self.transitions.insert(from, vec![to]);
        }
    }
    fn transition_to(&mut self, new_state: State) -> Result<(), String>{
        if let Some(valid_states) = self.transitions.get(&self.current_state) {
            if valid_states.contains(&new_state) {
                println!(
                    "Transitioning from {:?} to {:?}",
                    self.current_state, new_state
                );

                self.current_state.on_exit();

                self.current_state = new_state;

                self.current_state.on_enter();
                return Ok(());
            }
        }
        Err(format!(
            "Invalid transition from {:?} to {:?}",
            self.current_state, new_state
        ))
    }
}

fn main() {

}