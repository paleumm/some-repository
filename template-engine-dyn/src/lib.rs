#[derive(PartialEq, Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    pub expression: String,
    pub var_map: Vec<String>,
    pub gen_html: String,
}

pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");
    let is_for_tag = (check_symbol_string(&input_line, "for")
        && check_symbol_string(&input_line, "in"))
        || check_symbol_string(&input_line, "endfor");
    let is_if_tag =
        check_symbol_string(&input_line, "if") || check_symbol_string(&input_line, "endif");
    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");

    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }

    return_val
}

pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

pub fn check_matching_pair(input: &str, symbol1: &str, symbol2: &str) -> bool {
    input.contains(symbol1) && input.contains(symbol2)
}

pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let expression_iter = input_line.split_whitespace();
    let mut template_var_map: Vec<String> = vec![];
    for word in expression_iter {
        if check_symbol_string(word, "{{") && check_symbol_string(word, "}}") {
            template_var_map.push(word.to_string());
        }
    }

    ExpressionData {
        expression: input_line.into(),
        var_map: template_var_map,
        gen_html: "".into(),
    }
}

pub fn get_index_for_symbol(input: &str, symbol: char) -> (bool, usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;
    while let Some((c, d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }
    (does_exist, index)
}

use std::collections::HashMap;

pub fn generate_html_template_var(
    content: &mut ExpressionData,
    context: HashMap<String, String>,
) -> &mut ExpressionData {
    content.gen_html = content.expression.clone();
    for var in &content.var_map {
        let (_h, i) = get_index_for_symbol(&var, '{');
        let (_j, k) = get_index_for_symbol(&var, '}');
        let var_without_braces = &var[i + 2..k];
        let val = context.get(var_without_braces).unwrap();
        content.gen_html = content.gen_html.replace(var, val)
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello World</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    // #[test]
    // fn check_template_var_test() {
    //     let content = ExpressionData {
    //         head: Some("Hi ".to_string()),
    //         variable: "name".to_string(),
    //         tail: Some(", welcome".to_string()),
    //     };

    //     assert_eq!(
    //         ContentType::TemplateVariable(content),
    //         get_content_type("Hi {{name}}, welcome")
    //     );
    // }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %},welcome")
        );
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %}")
        );
    }

    #[test]
    fn check_symbol_string_test() {
        assert_eq!(true, check_symbol_string("{{Hello}}", "{{"));
    }

    #[test]
    fn check_symbol_pair_test() {
        assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"));
    }

    // #[test]
    // fn check_get_expression_data_test() {
    //     let expression_data = ExpressionData {
    //         head: Some("Hi ".to_string()),
    //         variable: "name".to_string(),
    //         tail: Some(", welcome".to_string()),
    //     };
    //     assert_eq!(expression_data, get_expression_data("Hi {{name}}, welcome"));
    // }

    #[test]
    fn check_get_index_for_symbol_test() {
        assert_eq!((true, 3), get_index_for_symbol("Hi {name}, welcome", '{'));
    }
}
