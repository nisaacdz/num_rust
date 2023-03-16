use num_rust::mat;
fn main() {
    let mut mat = mat![(4, 4), 0, 2, 3, 81, 6, 7, 8, 99, 5, 7, 2, 5, 6, 7, 17, 6];

    println!("{}", mat);
    mat *= 2.0;
    println!();
    println!("{}", mat);
    println!();

    let mat2 = &mat + &mat.clone();

    println!("{}", mat2);
}
