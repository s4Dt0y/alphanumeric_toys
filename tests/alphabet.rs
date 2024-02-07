use alphanumeric_toys::types::alphabet;

#[test]
fn alpha_lower_length() {
	let result = alphabet::ALPHA_LOWER.len();
	assert_eq!(result, 26)
}

#[test]
fn alpha_upper_length() {
	let result = alphabet::ALPHA_UPPER.len();
	assert_eq!(result, 26)
}

#[test]
fn test_alpha() {
	let true_lower = alphabet::is_valid_alpha(&'a');
	let true_upper = alphabet::is_valid_alpha(&'A');

	let false_alpha = alphabet::is_valid_alpha(&'1');

	assert_eq!(true_lower, true);
	assert_eq!(true_upper, true);

	assert_ne!(false_alpha, true)
}

#[test]
fn test_lower() {
	let true_lower = alphabet::is_valid_lower(&'a');
	let false_lower = alphabet::is_valid_lower(&'A');

	assert_eq!(true_lower, true);

	assert_ne!(false_lower, true);
}

#[test]
fn test_upper() {
	let true_upper = alphabet::is_valid_upper(&'A');
	let false_upper = alphabet::is_valid_upper(&'a');

	assert_eq!(true_upper, true);

	assert_ne!(false_upper, true);
}

