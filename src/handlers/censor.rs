use rustrict::CensorStr;

pub(super) fn execute(args: Vec<String>) -> String {
	let sentence: String = args.join(" ");
	if sentence.is_inappropriate() {
		sentence.censor()
	} else {
		sentence
	}
}
