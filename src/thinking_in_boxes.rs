use serde::Deserialize;
use std::collections::HashMap;
use toml;
#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
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
    pub questions: Questions,
    pub answers: Answers,
}
pub struct AppData {
    pub exam_data: ExamData,
}

#[derive(Debug, Deserialize)]
pub struct Questions {
    pub question: Vec<Question>,
}
#[derive(Debug, Deserialize)]
pub struct Answers {
    pub answer: Vec<Answer>,
}
#[derive(Debug, Deserialize)]
pub struct FilteredTopicData {
    pub answer: Vec<Answer>,
}

pub fn everything_is_an_abstraction() -> Result<ExamData, Box<dyn std::error::Error>> {
    let free_data = std::fs::read_to_string("./questions.toml")?;
    let why_are_we_questions: Questions = toml::from_str(&free_data)?;
    let free_data = std::fs::read_to_string("./answers.toml")?;
    let why_are_we_answers: Answers = toml::from_str(&free_data)?;
    let data = ExamData {
        questions: why_are_we_questions,
        answers: why_are_we_answers,
    };
    Ok(data)
}
impl AppData {
    pub fn get_data_list_for_topic<'a>(
        &'a self,
        topic: &str,
    ) -> HashMap<&'a Question, Vec<&'a Answer>> {
        let mut subtopic_list: HashMap<&Question, Vec<&Answer>> = HashMap::new();
        for question in &self.exam_data.questions.question {
            if question.topic == topic {
                let answers: Vec<&Answer> = self
                    .exam_data
                    .answers
                    .answer
                    .iter()
                    .filter(|a| a.question_id == question.id)
                    .collect();
                subtopic_list.insert(question, answers);
            }
        }

        subtopic_list
    }
    pub fn get_random_data_by_topic(&self) -> &Question {
        &self.exam_data.questions.question[0]
    }
}
