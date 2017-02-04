#![feature(pub_restricted)]

mod data {
    pub mod question_scorings;
    pub mod weights;
    pub mod category_map;
}

use data::question_scorings::QUESTION_SCORINGS;
use data::weights::*;
use data::category_map::CATEGORY_MAP;

struct QuestionScoring {
    yes: u8,
    maybe: u8,
    no: u8,
}

impl QuestionScoring {
    fn score_for_answer(&self, answer: Answer) -> u8 {
        match answer {
            Answer::Yes => self.yes,
            Answer::Maybe => self.maybe,
            Answer::No => self.no,
        }
    }
}

const N_CATEGORIES: usize = 10;
const QUESTIONS_PER_CATEGORY: usize = 20;
const N_QUESTIONS: usize = N_CATEGORIES * QUESTIONS_PER_CATEGORY;

#[derive(Clone, Copy)]
pub enum Answer {
    Yes,
    Maybe,
    No,
}

pub struct RawScores([u8; N_QUESTIONS]);

// Evaluates raw scores for each question
pub fn eval_raw(answers: &[Answer; N_QUESTIONS]) -> RawScores {
    let mut scores = [0; N_QUESTIONS];
    for i in 0..N_QUESTIONS {
        let ans = answers[i];
        let scoring = &QUESTION_SCORINGS[i];
        scores[i] = scoring.score_for_answer(ans);
    }
    RawScores(scores)
}

pub struct RawScore(u8);

impl RawScore {
    pub fn weigh(&self, group: WeighGroup, category: u8) -> i8 {
        let group_array = match group {
            WeighGroup::Boy => WEIGHT_BOY,
            WeighGroup::Girl => WEIGHT_GIRL,
            WeighGroup::AdultMale => WEIGHT_ADULT_MALE,
            WeighGroup::AdultFemale => WEIGHT_ADULT_FEMALE,
        };
        let categ_array = group_array[category as usize];
        categ_array[(self.0 - 37) as usize]
    }
}

impl RawScores {
    pub fn raw_score_for_category(&self, category: u8) -> RawScore {
        let categ_questions = CATEGORY_MAP[category as usize];
        let mut total = 0;
        for &index in categ_questions.iter() {
            total += self.0[index as usize];
        }
        RawScore(total)
    }
}

pub enum WeighGroup {
    Boy,
    AdultMale,
    Girl,
    AdultFemale,
}

#[cfg(test)]
mod tests;

type CategQuestions = [u8; QUESTIONS_PER_CATEGORY];

type CategWeights = [i8; 81];
