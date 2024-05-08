// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::{print_difference, print_array, ding, on_off, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    let coords_arr = [coords.0, coords.1];
    let series = [1, 1, 2, 3, 5, 8, 13];
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    print_difference(coords.0, coords.1);
    print_array(coords_arr);
    ding(series[series.len() - 1]);
    on_off(mess.2[1].0);
    print_distance(coords);
}
