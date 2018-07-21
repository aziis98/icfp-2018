mod nmms;

use nmms::*;

fn main() {
    let m = Matrix::from_file("./problemsL/LA059_tgt.mdl");

    println!("resolution: {}", m.resolution);
    println!("filled: {}", m.count_filled());

    println!("{}", m.is_grounded(Coordinate::new(2, 25, 25)))

    // let mut model = String::new();

    // for y in (0..20).rev() {
    //     model.push('.');
    //     for x in 0..20 {
    //         if m.get_voxel(nmms::Coordinate::new(x, y, 7)) {
    //             model.push('#');
    //         } else {
    //             model.push(' ');
    //         }
    //     }
    //     model.push_str(".\n");
    // }

    // println!("");
    // println!("{}", model);
}
