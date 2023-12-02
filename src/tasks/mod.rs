use crate::api::get_input;

mod day1;
mod day2;
pub fn solve(day: u16, level: u8) -> String {
    let result = match day {
        1 => {
            let result = day1::solve_day_01(get_input(day), level);
            println!("Day 1 solution: {}", result);
            result.to_string()
        }
        2 => {
            let result = day2::solve_day_02(get_input(day), level);
            result.to_string()
        }
        _ => panic!("Day not implemented"),
    };
    println!("Day {}, level {} solution: {}", day, level, result);
    return result;
}
