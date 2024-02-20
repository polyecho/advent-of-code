/*
https://adventofcode.com/2023/day/1

Your calculation isn't quite right. It looks like some of the digits are actually
spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also
count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on
each line. For example:
```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
Adding these together produces 281.

What is the sum of all of the calibration values?
*/

pub mod trebuchet {
    #[allow(dead_code)]
    pub fn get_number_sum(input: &str) -> usize {
        let lines = input.split('\n');
        let mut two_digit_vec: Vec<String> = Vec::new();

        for line in lines {
            two_digit_vec.push(get_line_digits(line));
        }

        let mut sum: usize = 0;
        for number in two_digit_vec {
            let number = number
                .parse::<usize>()
                .expect("Failed to parse two digits into a usize value.");
            sum += number;
        }

        sum
    }

    fn get_line_digits(line: &str) -> String {
        let first_digit = get_first_line_digit(line).expect("Failed to read the first digit.");
        let last_digit = get_first_line_digit(&line.chars().rev().collect::<String>())
            .expect("Failed to read the last digit.");

        format!("{}{}", first_digit, last_digit)
    }

    const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const NUMBER_LETTERS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn get_first_line_digit(input: &str) -> Option<usize> {
        let mut temp_string = String::new();

        for item in input.chars() {
            temp_string.push(item);

            for index in 0..10 {
                if temp_string.contains(NUMBERS[index])
                    || temp_string.contains(NUMBER_LETTERS[index])
                {
                    return Some(index);
                }

                let reversed_string = temp_string.chars().rev().collect::<String>();
                if reversed_string.contains(NUMBERS[index])
                    || reversed_string.contains(NUMBER_LETTERS[index])
                {
                    return Some(index);
                }
            }
        }

        None
    }

    mod test {
        #[cfg(test)]
        mod get_number_sum {
            use super::super::get_number_sum;

            #[test]
            fn test_example() {
                let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
                let output = get_number_sum(input);
                let expected_answer: usize = 142;

                assert_eq!(output, expected_answer);
            }
        }

        #[cfg(test)]
        mod replace_number_letters {
            mod basic_cases {
                use super::super::super::get_first_line_digit;

                #[test]
                fn basic_numbers() {
                    assert_eq!(get_first_line_digit("zero"), Some(0));
                    assert_eq!(get_first_line_digit("one"), Some(1));
                    assert_eq!(get_first_line_digit("two"), Some(2));
                    assert_eq!(get_first_line_digit("three"), Some(3));
                    assert_eq!(get_first_line_digit("four"), Some(4));
                    assert_eq!(get_first_line_digit("five"), Some(5));
                    assert_eq!(get_first_line_digit("six"), Some(6));
                    assert_eq!(get_first_line_digit("seven"), Some(7));
                    assert_eq!(get_first_line_digit("eight"), Some(8));
                    assert_eq!(get_first_line_digit("nine"), Some(9));
                }

                #[test]
                fn test_overlapping_numbers() {
                    assert_eq!(get_first_line_digit("nine"), Some(9));
                }
            }

            mod overlapping_numbers {
                use super::super::super::get_line_digits;

                #[test]
                fn test_overlapping_numbers() {
                    assert_eq!(get_line_digits("twone"), "21");
                }

                #[test]
                fn test_overlapping_numbers_2() {
                    assert_eq!(get_line_digits("eightwo"), "82");
                }

                #[test]
                fn test_overlapping_numbers_3() {
                    assert_eq!(get_line_digits("nineight"), "98");
                }

                #[test]
                fn test_overlapping_numbers_4() {
                    assert_eq!(get_line_digits("eighthreee"), "83");
                }

                #[test]
                fn test_overlapping_numbers_5() {
                    assert_eq!(get_line_digits("nineeight"), "98");
                }
            }

            mod other_edge_cases {
                use super::super::super::get_line_digits;

                #[test]
                fn test_other_edge_cases_1() {
                    assert_eq!(get_line_digits("eeeight"), "88");
                }

                #[test]
                fn test_other_edge_cases_2() {
                    assert_eq!(get_line_digits("oooneone"), "11");
                }
            }
        }
    }
}
