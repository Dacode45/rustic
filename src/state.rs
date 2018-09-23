//! Utilities for game state management.
pub struct StateData<'a, T>
where
    T: 'a,
{
    /// User defined game data
    pub data: &'a mut T,
}

impl<'a, T> StateData<'a, T>
where
    T: 'a,
{
    /// Create a new state data
    pub fn new(data: &'a mut T) -> Self {
        StateData { data }
    }
}

/// Types of state transitions.
pub enum Trans<T> {
    /// Continue as normal.
    None,
    /// Remove the active state and resume the next state on the stack or stop
    /// if there are none.
    Pop,
    /// Pause the active state and push a new state onto the stack.
    Push(Box<State<T> + Sync>),
    /// Remove the current state on the stack and insert a different one.
    Switch(Box<State<T> + Sync>),
    /// Stop and remove all states and shut down the engine.
    Quit,
}

/// A trait which defines game states that can be used by the state machine.
pub trait State<T> {
    /// Executed when the game state begins. Trans is ignored
    fn on_start(&mut self, _data: StateData<T>) -> Trans<T> {
        Trans::None
    }

    /// Executed when the game state exits.
    fn on_stop(&mut self, _data: StateData<T>) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, _data: StateData<T>) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _data: StateData<T>) {}

    fn is_blocking(&self) -> bool {
        true
    }
    fn state_name(&self) -> String {
        "State".to_owned()
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(&mut self, _dt: f32, _data: StateData<T>) -> Trans<T> {
        Trans::None
    }

    fn draw(&mut self, _data: StateData<T>) {}
}

/// A simple stack-based state machine (pushdown automaton).
#[derive(Derivative)]
#[derivative(Debug)]
pub struct StateMachine<T> {
    running: bool,
    #[derivative(Debug = "ignore")]
    pub state_stack: Vec<Box<State<T> + Sync>>,
}

impl<T> StateMachine<T> {
    /// Creates a new state machine with the given initial state.
    pub fn new(initial_state: Box<State<T> + Sync>) -> StateMachine<T> {
        StateMachine {
            running: false,
            state_stack: vec![initial_state],
        }
    }

    /// Checks whether the state machine is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Initializes the state machine.
    ///
    /// # Panics
    /// Panics if no states are present in the stack.
    pub fn start(&mut self, data: StateData<T>) {
        if !self.running {
            // run through iterators in reverse
            if self.state_stack.len() == 0 {
                panic!("state stack is empty");
            }
            let StateData { data } = data;
            for state in self.state_stack.iter_mut().rev() {
                state.on_start(StateData { data });
                if state.is_blocking() {
                    break;
                }
            }
            self.running = true;
        }
    }

    /// Updates the currently active state immediately.
    pub fn update(&mut self, dt: f32, data: StateData<T>) {
        let StateData { data } = data;
        if self.running {
            let mut trans = Trans::None;
            {
                let mut blocking = false;
                let mut substack: Vec<&mut Box<State<T> + Sync>> = self
                    .state_stack
                    .iter_mut()
                    .rev()
                    .take_while(|state| {
                        if blocking {
                            return false;
                        }
                        // debug!("Blocking? {} {}", state.state_name(), state.is_blocking());
                        blocking = state.is_blocking();
                        true
                    }).collect();
                if let Some((first, rest)) = substack.split_first_mut() {
                    trans = (&mut *first).update(dt, StateData { data });
                    // update everything
                    for state in rest.iter_mut() {
                        state.update(dt, StateData { data });
                        if state.is_blocking() {
                            break;
                        }
                    }
                } else {
                    panic!("empty state list");
                }
            }
            self.transition(trans, StateData { data });
        }
    }

    /// Draw the currently active state immediately.
    pub fn draw(&mut self, data: StateData<T>) {
        let StateData { data } = data;

        if self.running {
            // draw everything
            for state in self.state_stack.iter_mut() {
                // println!("running: {}", state.state_name());
                state.draw(StateData { data });
            }
        }
    }

    /// Performs a state transition, if requested by either update() or
    /// fixed_update().
    pub fn transition(&mut self, request: Trans<T>, data: StateData<T>) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(data),
                Trans::Push(state) => self.push(state, data),
                Trans::Switch(state) => self.switch(state, data),
                Trans::Quit => self.stop(data),
            }
        } else {
            panic!("Not running");
        }
    }

    /// Removes the current state on the stack and inserts a different one.
    fn switch(&mut self, state: Box<State<T> + Sync>, data: StateData<T>) {
        if self.running {
            let StateData { data } = data;
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(StateData { data });
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(StateData { data });
        }
    }

    /// Pauses the active state and pushes a new state onto the state stack.
    fn push(&mut self, next: Box<State<T> + Sync>, data: StateData<T>) {
        println!("pushing");
        if self.running {
            let StateData { data } = data;
            if let Some(state) = self.state_stack.last_mut() {
                if next.is_blocking() {
                    state.on_pause(StateData { data });
                }
            }

            self.state_stack.push(next);
            let next = self.state_stack.last_mut().unwrap();
            next.on_start(StateData { data });
        }
    }

    /// Stops and removes the active state and un-pauses the next state on the
    /// stack (if any).
    fn pop(&mut self, data: StateData<T>) {
        if self.running {
            let StateData { data } = data;
            let mut was_blocking = true;
            if let Some(mut state) = self.state_stack.pop() {
                was_blocking = state.is_blocking();
                state.on_stop(StateData { data });
            }

            if let Some(state) = self.state_stack.last_mut() {
                if was_blocking {
                    state.on_resume(StateData { data });
                }
            } else {
                self.running = false;
            }
        }
    }

    pub fn current(&self) -> &Box<State<T> + Sync> {
        self.state_stack.last().unwrap()
    }

    /// Shuts the state machine down.
    pub(crate) fn stop(&mut self, data: StateData<T>) {
        if self.running {
            let StateData { data } = data;
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(StateData { data });
            }

            self.running = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct State1(u8);
    struct State2;

    impl State<()> for State1 {
        fn update(&mut self, _: f32, _: StateData<()>) -> Trans<()> {
            if self.0 > 0 {
                self.0 -= 1;
                Trans::None
            } else {
                Trans::Switch(Box::new(State2))
            }
        }
    }

    impl State<()> for State2 {
        fn update(&mut self, _: f32, _: StateData<()>) -> Trans<()> {
            Trans::Pop
        }
    }

    #[test]
    fn switch_pop() {
        let mut sm = StateMachine::new(Box::new(State1(7)));
        sm.start(StateData::new(&mut ()));

        for _ in 0..8 {
            sm.update(0.5, StateData::new(&mut ()));
            assert!(sm.is_running());
        }

        sm.update(0.5, StateData::new(&mut ()));
        assert!(!sm.is_running());
    }
}
