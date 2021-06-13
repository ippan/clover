use clover::runtime::state::State;
use crate::random::Random;
use clover::runtime::object::make_reference;
use crate::math::Math;

mod io;
mod random;
mod math;


pub fn clover_std_inject_to(state: &mut State) {
    state.add_native_function("print", io::print);
    state.add_native_model("Random", make_reference(Random {}));
    state.add_native_model("Math", make_reference(Math {}));
}