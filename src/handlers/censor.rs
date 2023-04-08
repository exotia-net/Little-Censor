use std::collections::HashMap;

use rustrict::CensorStr;

const POLISH_LETTERS: [char; 9] = ['ą', 'ć', 'ę', 'ł', 'ń', 'ś', 'ó', 'ź', 'ż'];

pub(super) fn execute(args: Vec<String>) -> String {
	let sentence: String = args.join(" ");
	
	if sentence.is_inappropriate() {
		let letter_places = find_polish_letters(sentence.clone());
		let mut censored = sentence.censor();
		fix_sentence(&mut censored, letter_places);
		censored
	} else {
		sentence
	}
}

fn find_polish_letters(sentence: String) -> HashMap<usize, String> {
	let mut letter_places: HashMap<usize, String> = HashMap::new();
	sentence.chars().enumerate().for_each(|(i, char)| {
		if POLISH_LETTERS.contains(&char) {
			letter_places.insert(i, char.to_owned().into());
		}
	});
	letter_places
}

fn fix_sentence(censored: &mut String, letter_places: HashMap<usize, String>) {
	for (key, value) in letter_places {
		if censored.chars().nth(key).unwrap() != '*' {
			censored.replace_range(
				censored
					.char_indices()
					.nth(key)
					.map(|(pos, ch)| (pos..pos + ch.len_utf8()))
					.unwrap(),
				value.as_str(),
			)
		}
	}
}
