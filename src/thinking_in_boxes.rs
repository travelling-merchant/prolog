use serde::Deserialize;
use toml;
#[derive(Debug, Deserialize)]
pub struct Question {
    pub id: u16,
    pub topic: String,
    pub subtopic: String,
    pub text: String,
    pub correct_option: u32,
}

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub id: u32,
    pub text: String,
    pub question_id: u16,
}
#[derive(Debug, Deserialize)]
pub struct ExamData {
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}
//pub fn aquire_q_n_a_correspondin_to_subtopic(subtopic: &str) -> (Question, Vec<Answer>) {
//   let
//}
pub fn everything_is_an_abstraction() -> Result<ExamData, Box<dyn std::error::Error>> {
    let free_data = std::fs::read_to_string("./questions.toml")?;
    let why_are_we_questions: Vec<Question> = toml::from_str(&free_data)?;
    let free_data = std::fs::read_to_string("./answers.toml")?;
    let why_are_we_answers: Vec<Answer> = toml::from_str(&free_data)?;
    let data = ExamData {
        questions: why_are_we_questions,
        answers: why_are_we_answers,
    };
    Ok(data)
}
