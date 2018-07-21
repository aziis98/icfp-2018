mod nmms;

fn main() {
    let m = nmms::Matrix::from_file("./problemsL/LA012_tgt.mdl");

    println!("resolution: {}", m.resolution);

    for y in (0..20).rev() {
        for x in 0..20 {
            if m.get_voxel(nmms::Coordinate::new(x, y, 8)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    for _ in 0 .. m.resolution {
        print!("-");
    }
    println!("\n");
}
