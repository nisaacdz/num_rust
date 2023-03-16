use num_rust::{mat, matrix, matrices::{Get, indices::{ColumnIndex, RowIndex}}};

fn main() {
    let m1 = mat![(3, 3), 2, 3, 4, 5, 3, 6, 4, 1, 7];
    let m2 = matrix![(3, 3), 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

    println!("{}\n", m1.get(RowIndex::from(-1)).unwrap());
    print!("{}", m2.get(ColumnIndex::from(-1)).unwrap()[0]);
}
