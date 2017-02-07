//! Library for evaluating
//! [OCA](https://en.wikipedia.org/wiki/Oxford_Capacity_Analysis) personality tests.
//!
//! ```
//! use oca_eval::{Answer, NormGroup, Category};
//!
//! // First, we need the answers of course.
//! // Looks like this person is very uncertain.
//! // That, or they're just messing around.
//! let answers = [Answer::Maybe; 200];
//!
//! let raw = oca_eval::eval_raw(&answers);
//! // Let's see how "happy" our customer is.
//! let raw_happiness = raw.raw_score_for_category(Category::B);
//! assert_eq!(raw_happiness.value(), 83);
//! // Hmm. The above doesn't tell us much.
//! // We want the "final" score that we would see on the OCA graph.
//!
//! // In order to get the final score, we need to weigh the raw score
//! // against the "norm" of the age/sex group our customer belongs to.
//! // There are different norms for young/adult males/females.
//! // Let's say that our customer is an adult male.
//! // Now let's get that happiness score!
//! assert_eq!(raw_happiness.weigh(NormGroup::AdultMale), -93);
//! // Well... That doesn't look good. You should try some of our courses.
//! ```

#![feature(pub_restricted, conservative_impl_trait)]
#![warn(missing_docs)]

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

/// An answer to an OCA question.
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Answer {
    Yes,
    Maybe,
    No,
}

/// A category that measures a certain aspect of personality.
#[derive(Clone, Copy)]
pub enum Category {
    /// Stable / unstable
    A = 0,
    /// Happy / depressed
    B = 1,
    /// Composed / nervous
    C = 2,
    /// Certainty / uncertainty
    D = 3,
    /// Active / inactive
    E = 4,
    /// Aggressive / inhibited
    F = 5,
    /// Responsible / irresponsible
    G = 6,
    /// Correct estimation / critical
    H = 7,
    /// Appreciative / lack of accord
    I = 8,
    /// Comm level / withdrawn
    J = 9,
}

impl Category {
    /// Creates a new `Category` from an index, in the order of categories (0-9).
    pub fn from_index(index: u8) -> Option<Self> {
        use Category::*;

        Some(match index {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            4 => E,
            5 => F,
            6 => G,
            7 => H,
            8 => I,
            9 => J,
            _ => return None,
        })
    }
    /// Iterates over all categories in order.
    pub fn iter() -> impl Iterator<Item = Category> + 'static {
        use Category::*;

        static CATEGS: [Category; 10] = [A, B, C, D, E, F, G, H, I, J];

        CATEGS.into_iter().cloned()
    }
    /// Returns the letter of this category.
    pub fn letter(self) -> char {
        use Category::*;

        match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
        }
    }
}

/// Evaluates the raw scores for all the answers.
pub fn eval_raw(answers: &[Answer; N_QUESTIONS]) -> RawScores {
    let mut scores = [0; N_QUESTIONS];
    for i in 0..N_QUESTIONS {
        let ans = answers[i];
        let scoring = &QUESTION_SCORINGS[i];
        scores[i] = scoring.score_for_answer(ans);
    }
    RawScores(scores)
}

/// The raw scores for the questions.
///
/// This is not the final score that would appear on the graph.
pub struct RawScores(pub [u8; N_QUESTIONS]);

impl RawScores {
    /// Gets the raw score for a specific category.
    pub fn raw_score_for_category(&self, category: Category) -> RawScoreForCategory {
        let categ_questions = CATEGORY_MAP[category as usize];
        let mut total = 0;
        for &index in &categ_questions {
            total += self.0[index as usize];
        }
        RawScoreForCategory {
            value: total,
            category: category,
        }
    }
}

/// The raw score for a specific category.
pub struct RawScoreForCategory {
    value: u8,
    category: Category,
}

impl RawScoreForCategory {
    /// Weighs this score against a norm group to get the final score that will appear in the graph.
    pub fn weigh(&self, group: NormGroup) -> i8 {
        let group_array = match group {
            NormGroup::YoungMale => WEIGHT_BOY,
            NormGroup::YoungFemale => WEIGHT_GIRL,
            NormGroup::AdultMale => WEIGHT_ADULT_MALE,
            NormGroup::AdultFemale => WEIGHT_ADULT_FEMALE,
        };
        let categ_array = group_array[self.category as usize];
        categ_array[(self.value - 37) as usize]
    }
    /// Returns the raw value of this score.
    pub fn value(&self) -> u8 {
        self.value
    }
}

/// An age/sex group for which there is a "norm".
pub enum NormGroup {
    /// Male, 14-18 years old.
    YoungMale,
    /// Male, 18+.
    AdultMale,
    /// Female, 14-18 years old.
    YoungFemale,
    /// Female, 18+.
    AdultFemale,
}

impl NormGroup {
    /// Constructs a `NormGroup` from an age and sex.
    ///
    /// Returns `None` if age is less than 14.
    pub fn from_age_and_sex(age: u8, sex: Sex) -> Option<Self> {
        match sex {
            Sex::Male => {
                match age {
                    14...17 => Some(NormGroup::YoungMale),
                    18...255 => Some(NormGroup::AdultMale),
                    _ => None,
                }
            }
            Sex::Female => {
                match age {
                    14...17 => Some(NormGroup::YoungFemale),
                    18...255 => Some(NormGroup::AdultFemale),
                    _ => None,
                }
            }
        }
    }
}

/// The two sexes that a norm group can have.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}

#[cfg(test)]
mod tests;

type CategQuestions = [u8; QUESTIONS_PER_CATEGORY];

type CategWeights = [i8; 81];
