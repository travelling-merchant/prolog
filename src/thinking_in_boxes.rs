use rand;
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

pub struct OneExamQuestionAndAnswers<'a> {
    pub question: &'a Question,
    pub answers: Vec<&'a Answer>,
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
    pub fn get_data_list_for_topic(&self, topic: &str) -> Vec<OneExamQuestionAndAnswers> {
        let mut topic_list: Vec<OneExamQuestionAndAnswers> = Vec::new();
        for question in &self.exam_data.questions.question {
            if question.topic == topic {
                let answers: Vec<&Answer> = self
                    .exam_data
                    .answers
                    .answer
                    .iter()
                    .filter(|a| a.question_id == question.id)
                    .collect();
                let question_n_answers: OneExamQuestionAndAnswers = OneExamQuestionAndAnswers {
                    question: question,
                    answers: answers,
                };
                topic_list.push(question_n_answers);
            }
        }

        topic_list
    }
    pub fn get_random_data_by_topic<'a>(
        &self,
        q_n_a_collection: &'a [OneExamQuestionAndAnswers<'a>],
    ) -> &OneExamQuestionAndAnswers<'a> {
        let max_number = q_n_a_collection.len();
        let random_number = rand::random_range(0..=max_number);
        &q_n_a_collection[random_number]
    }
}
