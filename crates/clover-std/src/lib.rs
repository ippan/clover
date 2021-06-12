use clover::runtime::state::State;
use crate::random::Random;
use clover::runtime::object::make_reference;

mod io;
mod random;


pub fn clover_inject_to(state: &mut State) {
    state.add_native_function("print", io::print);
    state.add_native_model("Random", make_reference(Random {}));
}