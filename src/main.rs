use num_rust::matrices::Matrix;
fn main() {
    let mut m2: Matrix<i32> = vec![vec![0, 2, 4]; 3].try_into().unwrap();

    for r in 0..3 {
        for c in 0..3 {
            m2[(r, c)] /= 2;
        }
    }
    println!("{}", m2);
}
