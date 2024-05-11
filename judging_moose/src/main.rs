use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("FAILED TO READ INPUT");
    let mut point_arr: Vec<u32> = input.split_whitespace().map(|x| x.parse().expect("FAILED TO PARSE A NUMBER")).collect();
    point_arr.sort();
    println!("{}", judging_moose(&point_arr));
}

 fn judging_moose(points: &[u32]) -> String {
    if points.iter().sum::<u32>() == 0 {
        return String::from("Not a moose");
    }

    if points[0] == points[1] {
        return format!("Even {}", points.iter().sum::<u32>());
    }
    format!("Odd {}", points[1] * 2)
}
