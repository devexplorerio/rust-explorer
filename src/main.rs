mod hello;
mod types;
mod enums;
mod consts;
mod variables;
mod fns;
mod arithmetic;
mod ifs;
mod loops;
mod whiles;
mod fors;
mod matches;
mod structs;
mod files;


fn main() {
    hello::hello_world();
    types::primitive_types();
    enums::enum_steepness();
    consts::enum_city();
    variables::variable_immutable();
    variables::variable_mutable();
    fns::calling_fns();
    fns::returning_fn();
    arithmetic::basic_arithmetic();
    ifs::if_else();
    ifs::if_elseif_else();
    loops::loop_equal();
    whiles::while_diff();
    whiles::while_countdown();
    fors::for_basic();
    fors::for_index();
    fors::for_break();
    matches::match_dest();
    structs::object_person();
    files::writing_file();
    files::reading_file();
}

