fn main() {

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    
    let s : String = " 1 2 3\n 4 5 6\n 7 8 9".to_string();
    for line in s.lines() {
        
        let mut row : Vec<i32> = Vec::new();
        
        for val in line.split_whitespace() {
            row.push(val.parse::<i32>().unwrap());
        }
        matrix.push(row);
    }
    
    
    print_matrix(&matrix);


    for row in matrix.iter_mut() {
        &row.reverse();
    }

    print_matrix(&matrix);

    let n = matrix.len();
    let m = matrix[0].len();

    for i in 0..=n/2 {
        for j in 0..=m/2 {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[m - j - 1][n - i - 1];
            matrix[m - j - 1][n - i - 1] = tmp;
        }
    }

    print_matrix(&matrix);

    /*
    expected:
    1 2 3      7 4 1
    4 5 6  ->  8 5 2
    7 8 9      9 6 3
    get:
    
    1 2 3 
    4 5 6 
    7 8 9 
    3 2 1 
    6 5 4 
    9 8 7 
    7 4 1 
    8 5 2 
    9 6 3 
    
    */
    
}

fn print_matrix(matrix : &Vec<Vec<i32>>) {
    println!();
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
             print!("{} ", matrix[i][j]);
        }
        println!();
    }
}