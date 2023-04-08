use rustrict::CensorStr;

pub(super) fn execute(args: Vec<String>) -> String {
	let sentence: String = args.join(" ");
	return if sentence.is_inappropriate() {
		sentence.censor()
	} else {
		sentence
	}
}
