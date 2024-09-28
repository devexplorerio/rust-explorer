
#[path = "00_intro/hello.rs"] mod hello;

#[path = "01_data_type/types.rs"] mod types;
#[path = "01_data_type/variables.rs"] mod variables;
#[path = "01_data_type/arithmetic.rs"] mod arithmetic;
#[path = "01_data_type/consts.rs"] mod consts;
#[path = "01_data_type/enums.rs"] mod enums;
#[path = "01_data_type/structs.rs"] mod structs;

#[path = "02_function/fns.rs"] mod fns;

#[path = "03_flow_control/ifs.rs"] mod ifs;
#[path = "03_flow_control/operators.rs"] mod operators;
#[path = "03_flow_control/loops.rs"] mod loops;
#[path = "03_flow_control/whiles.rs"] mod whiles;
#[path = "03_flow_control/fors.rs"] mod fors;
#[path = "03_flow_control/matches.rs"] mod matches;

#[path = "04_ownership/ownership.rs"] mod ownership;

#[path = "05_reference/references.rs"] mod references;

#[path = "09_file/files.rs"] mod files;

fn main() {
    hello::hello_world();

    types::primitive_types();
    types::string_type();
    types::tuple_type();
    variables::variable_immutable();
    variables::variable_mutable();
    arithmetic::basic_arithmetic();
    enums::enum_steepness();
    consts::enum_city();
    structs::object_person();

    fns::calling_fns();
    fns::returning_fn();
    fns::scope_fn();

    ifs::if_else();
    ifs::if_elseif_else();
    operators::logic_operators();
    loops::loop_equal();
    whiles::while_diff();
    whiles::while_countdown();
    fors::for_basic();
    fors::for_index();
    fors::for_break();
    fors::for_range();
    fors::for_nested();
    fors::for_nested_mut();
    fors::for_calc();
    matches::match_dest();

    ownership::ownership_copy();
    ownership::ownership_move();

    references::borrowing();
    references::slice_string();
    references::slice_array();
    references::slice_fn_params();
    references::trim_spaces_all();

    files::writing_file();
    files::reading_file();
}
