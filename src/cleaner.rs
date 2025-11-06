use crate::language::Language;
use regex::Regex;

pub fn clean_comments(content: &str, language: Language) -> String {
    let cleaned = match language {
        Language::C
        | Language::Cpp
        | Language::Java
        | Language::JavaScript
        | Language::TypeScript
        | Language::Rust => clean_c_style_comments(content),
        Language::Python => clean_python_comments(content),
        Language::Html => clean_html_comments(content),
        Language::Css => clean_css_comments(content),
        Language::Php => clean_php_comments(content),
        Language::Basic => clean_basic_comments(content),
    };

    clean_empty_lines(&cleaned)
}

fn clean_empty_lines(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut prev_empty = false;

    for line in lines {
        let is_empty = line.trim().is_empty();

        if is_empty {
            if !prev_empty {
                result.push(line);
            }
            prev_empty = true;
        } else {
            result.push(line);
            prev_empty = false;
        }
    }

    let mut output = result.join("\n");
    if content.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn clean_c_style_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut in_char = false;
    let mut string_delimiter = '"';
    let mut escape_next = false;
    let mut in_regex = false;
    let mut regex_escape_next = false;
    let mut in_regex_char_class = false;
    let mut prev_non_ws: Option<char> = None;
    let mut in_template = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            result.push('\\');
            result.push(ch);
            escape_next = false;
            continue;
        }

        if ch == '\\' {
            if in_string || in_char || in_template {
                escape_next = true;
                continue;
            }
            if in_regex {
                regex_escape_next = true;
                result.push(ch);
                continue;
            }
            result.push(ch);
            if !ch.is_whitespace() {
                prev_non_ws = Some(ch);
            }
            continue;
        }

        if in_template {
            result.push(ch);
            if escape_next {
                escape_next = false;
                if !ch.is_whitespace() {
                    prev_non_ws = Some(ch);
                }
                continue;
            }
            if ch == '`' {
                in_template = false;
                if !ch.is_whitespace() {
                    prev_non_ws = Some(ch);
                }
                continue;
            }
            if !ch.is_whitespace() {
                prev_non_ws = Some(ch);
            }
            continue;
        }

        if in_regex {
            result.push(ch);
            if regex_escape_next {
                regex_escape_next = false;
            } else {
                if ch == '[' {
                    in_regex_char_class = true;
                } else if ch == ']' {
                    in_regex_char_class = false;
                } else if ch == '/' && !in_regex_char_class {
                    in_regex = false;
                } else if ch == '\\' {
                    regex_escape_next = true;
                }
            }
            if !ch.is_whitespace() {
                prev_non_ws = Some(ch);
            }
            continue;
        }

        if !in_string && !in_char {
            if ch == '"' {
                in_string = true;
                string_delimiter = '"';
                result.push(ch);
                continue;
            } else if ch == '\'' {
                in_char = true;
                result.push(ch);
                continue;
            } else if ch == '`' {
                in_template = true;
                result.push(ch);
                continue;
            }

            if ch == '/' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        chars.next();
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                        continue;
                    } else if next_ch == '*' {
                        chars.next();
                        let mut prev = '*';
                        while let Some(c) = chars.next() {
                            if prev == '*' && c == '/' {
                                break;
                            }
                            prev = c;
                        }
                        continue;
                    } else if next_ch != '=' {
                        let prev = prev_non_ws;
                        let likely_regex_start = match prev {
                            None => true,
                            Some(p) => {
                                !(p.is_ascii_alphanumeric()
                                    || p == ')'
                                    || p == ']'
                                    || p == '}'
                                    || p == '.')
                            }
                        };
                        if likely_regex_start {
                            in_regex = true;
                            result.push(ch);
                            if !ch.is_whitespace() {
                                prev_non_ws = Some(ch);
                            }
                            continue;
                        }
                    }
                }
            }
        } else if in_string && ch == string_delimiter {
            in_string = false;
            result.push(ch);
            continue;
        } else if in_char && ch == '\'' {
            in_char = false;
            result.push(ch);
            continue;
        }

        result.push(ch);
        if !ch.is_whitespace() {
            prev_non_ws = Some(ch);
        }
    }

    result
}

fn clean_python_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut string_delimiter = '"';
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            if in_string {
                result.push('\\');
                result.push(ch);
            }
            escape_next = false;
            continue;
        }

        if ch == '\\' && in_string {
            escape_next = true;
            continue;
        }

        if !in_string {
            if ch == '"' || ch == '\'' {
                let quote = ch;
                let mut count = 1;

                if chars.peek() == Some(&quote) {
                    let next1 = chars.next();
                    count += 1;
                    if chars.peek() == Some(&quote) {
                        let next2 = chars.next();
                        count += 1;

                        if count == 3 {
                            let has_assignment = result.trim_end().ends_with('=');

                            if has_assignment {
                                for _ in 0..3 {
                                    result.push(quote);
                                }
                                while let Some(c) = chars.next() {
                                    result.push(c);
                                    if c == quote && chars.peek() == Some(&quote) {
                                        result.push(chars.next().unwrap());
                                        if chars.peek() == Some(&quote) {
                                            result.push(chars.next().unwrap());
                                            break;
                                        }
                                    }
                                }
                            } else {
                                while let Some(c) = chars.next() {
                                    if c == quote && chars.peek() == Some(&quote) {
                                        chars.next();
                                        if chars.peek() == Some(&quote) {
                                            chars.next();
                                            break;
                                        }
                                    }
                                }
                            }
                            continue;
                        } else {
                            result.push(quote);
                            result.push(next1.unwrap());
                            result.push(next2.unwrap());
                            continue;
                        }
                    } else {
                        result.push(quote);
                        result.push(next1.unwrap());
                        continue;
                    }
                }

                in_string = true;
                string_delimiter = quote;
                result.push(ch);
                continue;
            }

            if ch == '#' {
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
        } else {
            if ch == string_delimiter {
                in_string = false;
                result.push(ch);
                continue;
            }
        }

        result.push(ch);
    }

    result
}

fn clean_html_comments(content: &str) -> String {
    let re = Regex::new(r"<!--[\s\S]*?-->").unwrap();
    let mut result = content.to_string();

    let script_re = Regex::new(r"(?s)<script[^>]*>(.*?)</script>").unwrap();
    let style_re = Regex::new(r"(?s)<style[^>]*>(.*?)</style>").unwrap();

    let mut scripts = Vec::new();
    let mut styles = Vec::new();

    for cap in script_re.captures_iter(&result) {
        let script_content = cap.get(1).unwrap().as_str();
        let cleaned = clean_c_style_comments(script_content);
        scripts.push((script_content.to_string(), cleaned));
    }

    for cap in style_re.captures_iter(&result) {
        let style_content = cap.get(1).unwrap().as_str();
        let cleaned = clean_css_comments(style_content);
        styles.push((style_content.to_string(), cleaned));
    }

    result = re.replace_all(&result, "").to_string();

    for (original, cleaned) in scripts {
        result = result.replace(&original, &cleaned);
    }

    for (original, cleaned) in styles {
        result = result.replace(&original, &cleaned);
    }

    result
}

fn clean_css_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut string_delimiter = '"';
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            if in_string {
                result.push('\\');
                result.push(ch);
            }
            escape_next = false;
            continue;
        }

        if ch == '\\' && in_string {
            escape_next = true;
            continue;
        }

        if !in_string {
            if ch == '"' || ch == '\'' {
                in_string = true;
                string_delimiter = ch;
                result.push(ch);
                continue;
            }

            if ch == '/' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '*' {
                        chars.next();
                        let mut prev = '*';
                        while let Some(c) = chars.next() {
                            if prev == '*' && c == '/' {
                                break;
                            }
                            prev = c;
                        }
                        continue;
                    }
                }
            }
        } else if ch == string_delimiter {
            in_string = false;
            result.push(ch);
            continue;
        }

        result.push(ch);
    }

    result
}

fn clean_php_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut in_char = false;
    let mut string_delimiter = '"';
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            if !in_string && !in_char {
                result.push('\\');
            }
            if in_string || in_char {
                result.push('\\');
                result.push(ch);
            }
            escape_next = false;
            continue;
        }

        if ch == '\\' {
            escape_next = true;
            continue;
        }

        if !in_string && !in_char {
            if ch == '"' {
                in_string = true;
                string_delimiter = '"';
                result.push(ch);
                continue;
            } else if ch == '\'' {
                in_char = true;
                result.push(ch);
                continue;
            }

            if ch == '/' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        chars.next();
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                        continue;
                    } else if next_ch == '*' {
                        chars.next();
                        let mut prev = '*';
                        while let Some(c) = chars.next() {
                            if prev == '*' && c == '/' {
                                break;
                            }
                            prev = c;
                        }
                        continue;
                    }
                }
            }

            if ch == '#' {
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
        } else if in_string && ch == string_delimiter {
            in_string = false;
            result.push(ch);
            continue;
        } else if in_char && ch == '\'' {
            in_char = false;
            result.push(ch);
            continue;
        }

        result.push(ch);
    }

    result
}

fn clean_basic_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut string_delimiter = '"';

    while let Some(ch) = chars.next() {
        if !in_string {
            if ch == '"' {
                in_string = true;
                string_delimiter = '"';
                result.push(ch);
                continue;
            }

            if ch == '\'' {
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            }

            if ch == 'R' || ch == 'r' {
                let mut temp = String::new();
                temp.push(ch);

                let mut matched = true;
                for expected in ['E', 'M'] {
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch.to_ascii_uppercase() == expected {
                            temp.push(chars.next().unwrap());
                        } else {
                            matched = false;
                            break;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch.is_whitespace() || next_ch == '\n' {
                            while let Some(&c) = chars.peek() {
                                if c == '\n' {
                                    break;
                                }
                                chars.next();
                            }
                            continue;
                        }
                    }
                }

                result.push_str(&temp);
                continue;
            }
        } else if ch == string_delimiter {
            in_string = false;
            result.push(ch);
            continue;
        }

        result.push(ch);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_single_line_comment() {
        let input = "int x = 5; // this is a comment\nint y = 10;";
        let expected = "int x = 5; \nint y = 10;";
        assert_eq!(clean_c_style_comments(input), expected);
    }

    #[test]
    fn test_c_multi_line_comment() {
        let input = "int x = 5; /* this is a\nmulti-line comment */ int y = 10;";
        let expected = "int x = 5;  int y = 10;";
        assert_eq!(clean_c_style_comments(input), expected);
    }

    #[test]
    fn test_python_comment() {
        let input = "x = 5  # this is a comment\ny = 10";
        let expected = "x = 5  \ny = 10";
        assert_eq!(clean_python_comments(input), expected);
    }

    #[test]
    fn test_string_with_comment_chars() {
        let input = r#"String s = "// not a comment";"#;
        let output = clean_c_style_comments(input);
        assert!(output.contains("// not a comment"));
    }

    #[test]
    fn test_js_regex_preserve_escape() {
        let input = r#"const re = /[^\d]/g;"#;
        let output = clean_c_style_comments(input);
        assert!(output.contains(r"/[^\d]/g"));
    }

    #[test]
    fn test_js_regex_with_double_slash() {
        let input = r#"const re = /https?:\/\/[^\s]+/;"#;
        let output = clean_c_style_comments(input);
        assert!(output.contains(r"/https?:\/\/[^\s]+/"));
    }

    #[test]
    fn test_js_regex_with_comment_markers() {
        let input = r#"const re = /\/\*[^]*\*\//;"#;
        let output = clean_c_style_comments(input);
        assert!(output.contains(r"/\/\*[^]*\*\//"));
    }

    #[test]
    fn test_js_template_string_with_url() {
        let input = r#"const apiUrl = `https://example.com/path?x=${1+2}`;"#;
        let output = clean_c_style_comments(input);
        assert!(output.contains("`https://example.com/path?x=${1+2}`"));
    }
}
