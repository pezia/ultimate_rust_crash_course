use ding_machine;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    ding_machine::print_difference( coords.0, coords.1 );

    let coords_arr = [coords.0, coords.1];
    ding_machine::print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding_machine::ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    ding_machine::on_off(mess.2[1].0);
    ding_machine::print_distance(coords);
}
