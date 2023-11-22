/*
 * This file is for processing the Jacobi Iteration
 *
 * Author: Michael Plekan
 */

fn main() {
    //create a struct for the Field
    #[derive(Debug)]//for printing
    #[derive(Clone)]//for copying
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
                "boundary" => Boundary,
                "diagonal" => Diagonal,
                "random" => Random,
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
            for (x,y) in (0..size).zip(0..size) {
                if rng.gen() {
                    set!(x, y);
                }
            }
        },
    }
    //iterate until the matrix is stable
    //TODO: add a max iteration count
    //TODO: add a check for a stable matrix

}
