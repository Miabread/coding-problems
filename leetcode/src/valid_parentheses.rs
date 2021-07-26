pub fn valid_parentheses(input: &str) -> bool {
    let mut stack = Vec::new();

    for char in input.chars() {
        match char {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            _ => {
                if stack.pop() != Some(char) {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

#[test]
fn test_valid_parentheses() {
    assert_eq!(true, valid_parentheses("()"));
    assert_eq!(true, valid_parentheses("()[]{}"));
    assert_eq!(false, valid_parentheses("(]"));
    assert_eq!(false, valid_parentheses("([)]"));
    assert_eq!(true, valid_parentheses("{[]}"));
}
