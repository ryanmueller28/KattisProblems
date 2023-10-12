use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read");
    let mut iter = line.split_whitespace();
    let mut x_nums: Vec<f32> = Vec::new();
    let mut y_nums: Vec<f32> = Vec::new();
    x_nums.push(iter.next().unwrap().parse().unwrap());
    y_nums.push(iter.next().unwrap().parse().unwrap());
    x_nums.push(iter.next().unwrap().parse().unwrap());
    y_nums.push(iter.next().unwrap().parse().unwrap());
    x_nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    y_nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let x_len = x_nums[1] - x_nums[0];
    let y_len = y_nums[1] - y_nums[0];
    println!("{:.3}", calculate_rectangle_area(x_len, y_len));
}

fn calculate_rectangle_area(x_size: f32, y_size: f32) -> f32 {
    x_size * y_size
}