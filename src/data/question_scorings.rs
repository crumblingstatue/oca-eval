use {QuestionScoring, N_QUESTIONS};

macro_rules! scores {
    ($($y:tt $m:tt $n:tt),+) => {
        [$(QuestionScoring { yes: $y, maybe: $m, no: $n }),+]
    }
}

// Score table for the OCA answers
pub(crate) const QUESTION_SCORINGS: [QuestionScoring; N_QUESTIONS] = scores! {
    //     y m n, y m n, ... y=yes, m=maybe, n=no
    2 4 6, 6 5 3, 6 4 3, 3 4 6, 4 4 5, 3 3 6, 3 4 5, 2 4 5, 3 4 4, 5 4 4,
    3 4 5, 2 4 6, 5 4 3, 2 4 5, 3 3 6, 3 4 4, 2 4 5, 2 6 6, 5 3 3, 5 5 3,
    2 3 6, 1 5 6, 5 4 3, 3 5 6, 3 1 5, 2 3 6, 5 4 4, 6 2 2, 6 3 3, 3 4 6,
    5 4 4, 2 4 6, 6 5 3, 6 3 3, 5 5 2, 3 5 6, 3 4 6, 4 4 5, 3 4 5, 2 3 5,
    6 4 4, 5 3 3, 4 4 5, 2 3 6, 3 4 5, 3 3 5, 2 5 6, 3 4 5, 6 4 4, 4 4 3,
    5 4 2, 3 4 4, 3 4 6, 6 3 3, 2 5 6, 5 3 3, 5 4 4, 6 3 3, 1 3 5, 2 5 6,
    2 4 6, 5 4 4, 5 4 4, 2 2 6, 4 4 5, 6 4 3, 2 4 5, 6 4 3, 6 3 3, 4 4 5,
    3 5 6, 3 4 4, 2 4 5, 2 4 6, 5 3 3, 1 3 6, 6 2 2, 2 5 6, 5 4 3, 6 3 3,
    6 3 3, 3 4 6, 3 3 6, 6 3 2, 6 3 3, 2 4 6, 3 3 5, 5 4 3, 2 5 6, 3 3 5,
    3 4 6, 3 4 4, 5 3 3, 5 4 4, 3 4 5, 6 4 2, 5 5 3, 4 4 5, 2 4 5, 4 3 1,
    5 4 4, 2 3 5, 4 4 6, 2 3 6, 6 4 3, 2 3 7, 2 3 6, 3 4 5, 3 4 5, 5 4 4,
    2 4 5, 6 2 2, 6 4 3, 6 4 3, 3 4 6, 6 4 3, 3 4 5, 7 2 2, 5 4 3, 5 4 3,
    4 4 5, 3 4 6, 5 4 3, 2 3 6, 1 3 4, 4 4 6, 5 4 4, 3 4 6, 2 5 6, 3 5 6,
    5 3 2, 2 4 6, 6 4 3, 4 4 3, 2 4 5, 3 5 5, 3 3 6, 5 3 3, 5 4 4, 2 5 6,
    2 5 6, 1 3 5, 2 4 6, 5 4 3, 5 5 3, 1 3 5, 6 4 4, 3 4 6, 3 5 5, 2 5 6,
    2 5 6, 6 5 3, 2 4 6, 3 3 5, 7 1 1, 6 4 2, 7 5 2, 5 5 3, 5 4 2, 2 2 6,
    2 5 6, 6 4 3, 5 4 4, 5 5 2, 6 3 3, 3 3 6, 6 4 2, 6 4 3, 2 3 6, 3 4 5,
    3 3 6, 3 4 6, 6 3 2, 5 4 3, 0 1 5, 2 2 6, 6 4 4, 5 3 3, 0 1 5, 3 5 6,
    3 3 6, 3 4 6, 2 4 6, 3 4 6, 6 6 3, 1 3 5, 3 3 5, 2 2 6, 3 5 5, 6 2 2,
    1 4 5, 2 4 6, 5 4 3, 4 5 6, 6 4 2, 2 5 6, 2 4 5, 2 5 6, 0 3 5, 5 3 3
};