fn main() {
    // simple pig latin converter
    // translates user input into pig latin
    use std::io;

    let mut user_input = String::new();

    println! {"I can convert your text to pig latin"}

    println! {"Please input text:"}

    // get user input
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");

    let str_input = user_input.to_string();
    // convert input to pig latin and get result
    let result: String = pig_latin(str_input);

    println!("here's your text: {}", result);
    // function for conversion
    fn pig_latin(str_input: String) -> String {
        let mut translation = String::new();
        let vowels = vec!["e", "u", "i", "o", "a"];
        for word in str_input.split_whitespace() {
            let mut current_word = String::new();
            // short words get different translation
            if word.len() < 3 {
                let full_word = format!("{word}yay");
                translation.push_str(&full_word);
                translation.push_str(" ");
            } else {
                // get variables for 1st and 2nd character to check for consonants, conversion
                // method will be different depending on if the word starts with a vowel, a single
                // consonant or two consonants.
                let fc = word.chars().next().unwrap().to_string();
                let sc = word.chars().nth(1).unwrap().to_string();
                let fcstr = fc.as_str();
                let scstr = sc.as_str();
                // converts words that start with a vowel. after conversion each word gets pushed
                // into 'translation' variable
                if vowels.contains(&fcstr) {
                    for (i, c) in word.chars().enumerate() {
                        if i > 0 {
                            current_word.push(c)
                        }
                    }
                    let full_word = format!("{current_word}h{fc}y");
                    translation.push_str(&full_word);
                    translation.push_str(" ");
                // converts words that start with a single consonant
                } else if vowels.contains(&scstr) {
                    for (i, c) in word.chars().enumerate() {
                        if i > 0 {
                            current_word.push(c)
                        }
                    }

                    let full_word = format!("{current_word}{fc}ay");
                    translation.push_str(&full_word);
                    translation.push_str(" ");
                } else {
                    // different conversion method for words that start with two consonants
                    for (i, c) in word.chars().enumerate() {
                        if i > 1 {
                            current_word.push(c)
                        }
                    }

                    let full_word = format!("{current_word}{fc}{sc}ay");
                    translation.push_str(&full_word);
                    translation.push_str(" ");
                }
            }
        }
        // return full translation
        translation
    }
}
