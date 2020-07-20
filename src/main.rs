fn main() {
    let lines = read_input("Input number of lines of pascal's triangle to generate: ");
    println!("generating {} lines", lines);
    let triangle = generate_triangle(lines);
    print_triangle(triangle)
}

fn read_input(msg: &str) -> u8 {
    let mut line = String::new();
    println!("{}", msg);
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();
    if let Ok(val) = line.parse::<u8>() {
        val
    } else {
        println!("{} is invalid input! Please enter a number.", line);
        read_input(msg)
    }
}

fn generate_triangle(lines: u8) -> Vec<Vec<u32>> {
    let mut current_line = 1;

    let mut triangle = Vec::new();
    let mut line = Vec::new();
    while {
        if current_line == 1 {
            line.push(1)
        } else {
            line = generate_line(line);   
        }
        triangle.push(line.clone());
        current_line += 1;

        current_line <= lines
    }{}

    triangle
}

fn generate_line(prev_line: Vec<u32>) -> Vec<u32> {
    let mut new_line = Vec::new();
    new_line.push(1);

    for i in 0..prev_line.len() - 1 {
        new_line.push(prev_line[i] + prev_line[i+1]);
    }

    new_line.push(1);
    new_line
}

fn print_triangle(triangle: Vec<Vec<u32>>) -> () {
    for i in &triangle {
        println!("{:?}", i);
    }
    ()
}
