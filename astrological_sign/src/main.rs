use std::io;

fn main() {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("FAILED TO READ INPUT");
    let n_times: i32 = user_input.trim().parse().expect("Failed to parse a number");



    let cal = AstrologicalCalendar::new();

    for _ in 0..n_times {
        let mut bday_input: String = String::new();
        io::stdin().read_line(&mut bday_input).expect("FAILED TO READ USER INPUT");
        let mut split = bday_input.split_whitespace();
        let n = split.next().expect("NO VAL").parse().expect("FAILED TO PARSE A NUMBER");
        let s = split.next().expect("FAILED TO READ NEXT").to_string();
        let result = cal.search_date(n, s);
        println!("{}", result.name);
    }
}

#[derive(Debug, Clone)]
struct AstrologicalSign {
    start_num: u8,
    end_num: u8,
    start_mon: String,
    end_mon: String,
    name: String,
}

impl AstrologicalSign {
    pub fn new(start_num: u8, end_num: u8, start_mon: String, end_mon: String, name: String) -> Self {
        AstrologicalSign {
            start_num,
            end_num,
            start_mon,
            end_mon,
            name,
        }
    }
}

#[derive(Debug)]
struct AstrologicalCalendar {
    months: Vec<AstrologicalSign>,
}

impl AstrologicalCalendar {
    pub fn new() -> Self {
        let mut result: Vec<AstrologicalSign> = Vec::new();
        result.push(AstrologicalSign::new(21, 20, String::from("Mar"), String::from("Apr"), String::from("Aries")));
        result.push(AstrologicalSign::new(21, 20, String::from("Apr"), String::from("May"), String::from("Taurus")));
        result.push(AstrologicalSign::new(21, 21, String::from("May"), String::from("Jun"), String::from("Gemini")));
        result.push(AstrologicalSign::new(22, 22, String::from("Jun"), String::from("Jul"), String::from("Cancer")));
        result.push(AstrologicalSign::new(23, 22, String::from("Jul"), String::from("Aug"), String::from("Leo")));
        result.push(AstrologicalSign::new(23, 21, String::from("Aug"), String::from("Sep"), String::from("Virgo")));
        result.push(AstrologicalSign::new(22, 22, String::from("Sep"), String::from("Oct"), String::from("Libra")));
        result.push(AstrologicalSign::new(23, 22, String::from("Oct"), String::from("Nov"), String::from("Scorpio")));
        result.push(AstrologicalSign::new(23, 21, String::from("Nov"), String::from("Dec"), String::from("Saggitarius")));
        result.push(AstrologicalSign::new(22, 20, String::from("Dec"), String::from("Jan"), String::from("Capricorn")));
        result.push(AstrologicalSign::new(21, 19, String::from("Jan"), String::from("Feb"), String::from("Aquarius")));
        result.push(AstrologicalSign::new(20, 20, String::from("Feb"), String::from("Mar"), String::from("Pisces")));

        AstrologicalCalendar {
            months: result,
        }
    }

    pub fn search_date(&self, date_num: u8, month_str: String) -> AstrologicalSign {
        let months_to_check: Vec<&AstrologicalSign> = self.months.iter().filter(|&m| m.start_mon == month_str || m.end_mon == month_str).collect();
        let first_month = months_to_check[0].clone();
        let last_month = months_to_check[1].clone();
        println!("{:?}", first_month);
        println!("{:?}", last_month);
        if date_num >= last_month.end_num {
            last_month
        } else {
            first_month
        }
    }
}
