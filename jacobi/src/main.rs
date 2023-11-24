/*
 * This file is for processing the Jacobi Iteration
 *
 * Author: Michael Plekan
 */
//create a struct for the Field

//derive traits for the struct debug, clone, and copy
#[derive(Debug, Clone, Copy)]
struct Field {
    value: f64,
    permanent: bool,
}
//create an enum for the start type
enum Start {
    Boundary,//all edges are 1
    Diagonal,//diagonal is 1
    Random,//random points are 1
}

fn jacobi_row(id:usize,size:usize, row: &mut Vec<Field>, matrix: & Vec<Vec<Field>>) {
    //iterate through the row with the index equal to the id
    for (c,f) in row.iter_mut().enumerate() {
        //check if the field is permanent
        if !f.permanent{
            //set the value to the average of the surrounding fields
            f.value = (matrix[id][c].value +
                        if id+1 < size {matrix[id+1][c].value} else {0.0} +
                        if id > 0 {matrix[id-1][c].value} else {0.0} +
                        if c+1 < size {matrix[id][c+1].value} else {0.0} +
                        if c > 0 {matrix[id][c-1].value} else {0.0}) / 5.0;
        }       
    }
}
//macro to print the matrix to stdout, parameter is the matrix
macro_rules! print_matrix {
    ($matrix:expr) => {
        println!();
        for row in $matrix.iter() {
            print!("| ");
            for field in row.iter() {
                print!("{:.2} | ", field.value);
            }
            println!();
        }
        println!();
    };
}
fn main() {
    use Start::*;
    let args: Vec<String> = std::env::args().collect();
    let size: usize;
    let mut start: Start = Random;
    let mut matrix1: Vec<Vec<Field>>;
    let mut matrix2: Vec<Vec<Field>>;

    //get the size of the matrix from args
    match args.len() {
        3 => {
            size = args[1].parse().unwrap();
            start = match args[2].as_str() {
                "boundary"|"b" => Boundary,
                "diagonal"|"d" => Diagonal,
                "random"|"r" => Random,
                _ => {
                    panic!("Usage: jacobi <size> <start type>");
                }
            }
        },
        2 => size = args[1].parse().unwrap(),
        _ => {
            panic!("Usage: jacobi <size> <start type>");
        }
    }
    
    //create the 2 2d matrices
    matrix1 = vec![vec![Field{value: 0.0, permanent: false}; size]; size];
    matrix2 = vec![vec![Field{value: 0.0, permanent: false}; size]; size];

    //edit the matrices based on the start type
    //macro to set a point to 1.0 and permanent
    macro_rules! set {
        ($x:expr, $y:expr) => {
            matrix1[$x][$y].value = 1.0;
            matrix1[$x][$y].permanent = true;
        };
    }
    //match the start type
    match start {
        Boundary => {
            for i in 0..size {
                set!(0, i);
                set!(size-1, i);
                set!(i, 0);
                set!(i, size-1);
            }
        },
        Diagonal => {
            for i in 0..size {
                set!(i, i);
            }
        },
        Random => {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            for i in 0..size {
                for j in 0..size {
                    if rng.gen() {
                        set!(i, j);
                    }
                }
            }
        },
    }
    //print the matrix
    print_matrix!(matrix1);
    //run 100 iterations of the jacobi method on the matrix going from matrix1 to matrix2 then matrix2 to matrix1
    for _ in 0..100 {
        for (id, row) in matrix1.iter_mut().enumerate() {
            jacobi_row(id,size, row, &matrix2)
        }
        for (id, row) in matrix2.iter_mut().enumerate() {
            jacobi_row(id,size, row, &matrix1)
        }
    }
    //print the matrix
    print_matrix!(matrix1);

}
