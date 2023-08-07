// Takes a topic that looks like "msh/2/c/LongFast/!abf849b0" and returns the user_id
pub fn get_user_id(topic: &str) -> Option<&str> {
	let parts: Vec<&str> = topic.split('/').collect();
	parts.last().cloned()
}
