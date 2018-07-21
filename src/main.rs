mod nmms;

fn main() {
    let m = nmms::Matrix::from_file("./problemsL/LA013_tgt.mdl");

    println!("resolution: {}", m.resolution);
    println!("filled: {}", m.count_filled());

    let mut model = String::new();

    for y in (0..20).rev() {
        model.push('.');
        for x in 0..20 {
            if m.get_voxel(nmms::Coordinate::new(x, y, 7)) {
                model.push('#');
            } else {
                model.push(' ');
            }
        }
        model.push_str(".\n");
    }

    println!("");
    println!("{}", model);
}
