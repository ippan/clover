use clover::runtime::state::State;

mod io;


pub fn clover_inject_to(state: &mut State) {
    state.add_native_function("print", io::print);
}