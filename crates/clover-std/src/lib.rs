use clover::runtime::state::State;
use clover::runtime::object::make_reference;

mod io;
mod random;
mod math;


pub fn clover_std_inject_to(state: &mut State) {
    state.add_native_function("print", io::print);

    state.add_native_model("IO", make_reference(io::IO {}));
    state.add_native_model("Random", make_reference(random::Random {}));
    state.add_native_model("Math", make_reference(math::Math {}));
}