pub mod problems_1_50;

pub trait Problem {
    fn solve(&self) -> String;
}

pub fn get_problem(id: u32) -> Option<Box<dyn Problem>> {
    match id {
        1 => Some(Box::new(problems_1_50::Problem1)),
        2 => Some(Box::new(problems_1_50::Problem2)),
        3 => Some(Box::new(problems_1_50::Problem3)),
        4 => Some(Box::new(problems_1_50::Problem4)),
        5 => Some(Box::new(problems_1_50::Problem5)),
        6 => Some(Box::new(problems_1_50::Problem6)),
        7 => Some(Box::new(problems_1_50::Problem7)),
        8 => Some(Box::new(problems_1_50::Problem8)),
        9 => Some(Box::new(problems_1_50::Problem9)),
        10 => Some(Box::new(problems_1_50::Problem10)),

        _ => None,
    }
}
