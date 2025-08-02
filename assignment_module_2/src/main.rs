fn assignment_1(){
    fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
        *total = 0;

        if step == 0{
            return;
        }
        if step > 0{
            for i in (low..=high).step_by(step as usize){
                *total += i;
            }
        }else{
            for i in (high..=low).step_by(step.abs() as usize){
                *total += i;
            }
        }
    }

    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}

fn assignment_2(){
    fn most_frequent_word(text: &str) -> (String, usize) {
        let words: Vec<&str> = text.split_whitespace().collect();

        let mut max_word = String::new();
        let mut max_count = 0;

        for i in 0..words.len() {
            let current_word = words[i];
            let mut current_count = 0;

            for j in 0..words.len() {
                if current_word == words[j] {
                    current_count += 1;
                }
            }

            if current_count > max_count {
                max_count = current_count;
                max_word = current_word.to_string();
            }
        }

        (max_word, max_count) // return tuple
    }

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

fn main() {
    //assignment_1();
    assignment_2();
}