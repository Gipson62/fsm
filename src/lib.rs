pub trait State {
    fn handle_input() {}
    fn on_update(&self, delta: f32) {}
    fn on_enter(&self, /* there should be args here */) {}
    fn on_exist(&self) {}
}

pub struct StateMachine {}