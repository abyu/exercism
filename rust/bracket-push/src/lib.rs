use std::collections::LinkedList;
use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
	let mut list: LinkedList<char> = LinkedList::new();

    for current_char in string.chars() {
    	if current_char == '[' || current_char == '{' || current_char == '(' {
    		list.push_back(current_char)
    	}
    	if current_char == ']' || current_char == '}' || current_char == ')' {    		
	    	match list.back() {
	    		None => return false,
	    		Some(&previous_char) => if matching_close(previous_char, current_char) { list.pop_back();} else {return false; },
	    	}
		}
    }
    return list.is_empty();
}

fn matching_close(match_with: char, match_to: char) -> bool {

	let bracket_pairs: HashMap<char, char> =
    [('}', '{'),
     (']', '['),
     (')', '(')]
     .iter().cloned().collect();

     match bracket_pairs.get(&match_to) {
     	None => return false,
     	Some(&x) => if x == match_with { return true; },
     }

    return false
}