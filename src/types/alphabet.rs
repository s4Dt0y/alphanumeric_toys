pub const ALPHA_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub const ALPHA_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn is_valid_alpha(c: &char) -> bool {
    if ALPHA_LOWER.contains(c) || ALPHA_UPPER.contains(c) {
        return true;
    }
    false
}

pub fn is_valid_lower(c: &char) -> bool {
	if ALPHA_LOWER.contains(c) {
		return true;
	}
	false
}

pub fn is_valid_upper(c: &char) -> bool {
	if ALPHA_UPPER.contains(c) {
		return true;
	}
	false
}
