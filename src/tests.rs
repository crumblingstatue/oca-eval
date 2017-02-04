use super::*;
use super::WeighGroup::*;

macro_rules! test_answer {
    (0) => { Answer::Yes };
    (1) => { Answer::Maybe };
    (2) => { Answer::No };
}

macro_rules! test_answers {
    ($($ans:tt) +) => { [$(test_answer!($ans)),+] };
}

macro_rules! define_test {
    ($name:ident, $group:expr, $correct_raw:expr, $correct_weighted:expr) => {
        #[test]
        fn $name() {
            let data = include!(concat!("../test-samples/", stringify!($name)));
            let raw = eval_raw(&data);
            let correct_raw_scores = $correct_raw;
            let correct_weighted_scores = $correct_weighted;
            for (i, &correct_raw) in correct_raw_scores.iter().enumerate() {
                let raw_score = raw.raw_score_for_category(i as u8);
                assert_eq!(raw_score.0, correct_raw);
                let weighted_score = raw_score.weigh($group, i as u8);
                assert_eq!(weighted_score, correct_weighted_scores[i]);
            }
        }
    };
}

define_test!(test_1,
             AdultFemale,
             [80, 107, 93, 79, 96, 86, 94, 104, 90, 109],
             [-38, 74, 24, 32, 90, 48, -18, 16, 34, 84]);
define_test!(test_2,
             AdultMale,
             [85, 83, 85, 86, 71, 79, 102, 90, 79, 71],
             [-40, -93, -86, 48, -12, 12, 36, -58, -76, -76]);
