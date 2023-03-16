#[macro_export]
macro_rules! mat {
    [($rows:expr, $cols:expr), $($elem:expr),*] => {
        {
            use std::convert::TryInto;
            use num_rust::dimension::Dimension;
            use num_rust::matrices::{GenericMatrix, MatrixContent};

            let dim = Dimension::new($rows.try_into().unwrap(), $cols.try_into().unwrap());
            let content_vec = vec![$($elem.try_into().unwrap()),*];
            if $rows * $cols != content_vec.len() {
                panic!("Dimension and size of the matrix must correspond");
            }
            let content = MatrixContent::new(dim, content_vec);
            GenericMatrix::from_content(content)
        }
    }
}

#[macro_export]
macro_rules! matrix {
    [($rows:expr, $cols:expr), $($elem:expr),*] => {
        {
            use std::convert::TryInto;
            use num_rust::dimension::Dimension;
            use num_rust::matrices::{Matrix, MatrixContent};

            let dim = Dimension::new($rows.try_into().unwrap(), $cols.try_into().unwrap());
            let content_vec = vec![$($elem),*];
            if $rows * $cols != content_vec.len() {
                panic!("Dimension and size of the matrix must correspond");
            }
            let content = MatrixContent::new(dim, content_vec);
            Matrix::from_content(content)
        }
    }
}
