
#[path = "0_intro/hello.rs"] mod hello;

#[path = "1_data_type/types.rs"] mod types;
#[path = "1_data_type/variables.rs"] mod variables;
#[path = "1_data_type/arithmetic.rs"] mod arithmetic;
#[path = "1_data_type/consts.rs"] mod consts;
#[path = "1_data_type/enums.rs"] mod enums;
#[path = "1_data_type/structs.rs"] mod structs;

#[path = "2_function/fns.rs"] mod fns;

#[path = "3_flow_control/ifs.rs"] mod ifs;
#[path = "3_flow_control/loops.rs"] mod loops;
#[path = "3_flow_control/whiles.rs"] mod whiles;
#[path = "3_flow_control/fors.rs"] mod fors;
#[path = "3_flow_control/matches.rs"] mod matches;

#[path = "9_file/files.rs"] mod files;

fn main() {
    hello::hello_world();

    types::primitive_types();
    variables::variable_immutable();
    variables::variable_mutable();
    arithmetic::basic_arithmetic();
    enums::enum_steepness();
    consts::enum_city();
    structs::object_person();

    fns::calling_fns();
    fns::returning_fn();

    ifs::if_else();
    ifs::if_elseif_else();
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

    files::writing_file();
    files::reading_file();
}
