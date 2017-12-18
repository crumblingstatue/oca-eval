use super::*;
use super::NormGroup::*;

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
                let raw_score = raw.raw_score_for_category(Category::from_index(i as u8).unwrap());
                assert_eq!(raw_score.value(), correct_raw);
                let weighted_score = raw_score.weigh($group);
                assert_eq!(weighted_score, correct_weighted_scores[i]);
            }
        }
    };
}

define_test!(
    test_1,
    AdultFemale,
    [80, 107, 93, 79, 96, 86, 94, 104, 90, 109],
    [-38, 74, 24, 32, 90, 48, -18, 16, 34, 84]
);
define_test!(
    test_2,
    AdultMale,
    [85, 83, 85, 86, 71, 79, 102, 90, 79, 71],
    [-40, -93, -86, 48, -12, 12, 36, -58, -76, -76]
);
define_test!(
    test_3,
    YoungMale,
    [67, 78, 87, 66, 81, 82, 80, 90, 78, 77],
    [-91, -91, -80, -16, 20, 18, -91, -58, -80, -68]
);
define_test!(
    test_4,
    YoungFemale,
    [72, 90, 92, 69, 75, 88, 90, 92, 77, 74],
    [-68, -54, -30, -8, -6, 52, 14, -10, -90, -86]
);
