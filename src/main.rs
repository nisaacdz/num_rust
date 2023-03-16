use num_rust::mat;
fn main() {
    let mat = mat![(4, 4), 0, 2, 3, 1, 6, 7, 8, 9, 5, 7, 2, 5, 6, 7, 7, 6];

    for en in mat.entries() {
        print!("{}", en);
    }

    println!();

    println!("{}", mat);
}
