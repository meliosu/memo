#[derive(Debug)]
pub struct Questionaire {
    pub questions: Vec<Question>,
}

#[derive(Debug)]
pub struct Question {
    pub problem: String,
    pub answers: Answers,
}

#[derive(Debug)]
pub enum Answers {
    Text(String),
    Choice((Vec<String>, usize)),
    Multi(Vec<(String, bool)>),
}
