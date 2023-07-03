#[derive(Debug, PartialEq, Clone, Copy)]
enum MarkdownDictionaryKey {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Breakline,
    Paragraph,
    List,
    Link,
    Image,
}

pub fn convert_markdown_to_html(markdown_text: String) -> String {
    let markdown_text_vec = markdown_text.split("\n");
    let mut html_format = String::new();
    for _line in markdown_text_vec {
        let line = _line.trim();
        html_format += &converstion_dictionary_to_html(line);
    }
    html_format
}

fn converstion_dictionary_to_html(markdown_line: &str) -> String {
    let identifier: MarkdownDictionaryKey = {
        if markdown_line.starts_with("######") {
            MarkdownDictionaryKey::H6
        } else if markdown_line.starts_with("#####") {
            MarkdownDictionaryKey::H5
        } else if markdown_line.starts_with("####") {
            MarkdownDictionaryKey::H4
        } else if markdown_line.starts_with("###") {
            MarkdownDictionaryKey::H3
        } else if markdown_line.starts_with("##") {
            MarkdownDictionaryKey::H2
        } else if markdown_line.starts_with("#") {
            MarkdownDictionaryKey::H1
        } else if markdown_line.starts_with("-") || markdown_line.starts_with("*") {
            MarkdownDictionaryKey::List
        } else if markdown_line.starts_with("[") && markdown_line.ends_with(")") {
            MarkdownDictionaryKey::Link
        } else if markdown_line.starts_with("!") && markdown_line.ends_with(")") {
            MarkdownDictionaryKey::Image
        } else if markdown_line.is_empty() {
            MarkdownDictionaryKey::Breakline
        } else {
            MarkdownDictionaryKey::Paragraph
        }
    };
    match identifier {
        MarkdownDictionaryKey::H1 => {
            format!(
                "<h1>{}</h1>",
                markdown_line.replace("#", "").trim().to_string()
            )
        }
        MarkdownDictionaryKey::H2 => {
            format!(
                "<h2>{}</h2>",
                markdown_line.replace("##", "").trim().to_string()
            )
        }
        MarkdownDictionaryKey::H3 => {
            format!(
                "<h3>{}</h3>",
                markdown_line.replace("###", "").trim().to_string()
            )
        }
        MarkdownDictionaryKey::H4 => {
            format!(
                "<h4>{}</h4>",
                markdown_line.replace("####", "").trim().to_string()
            )
        }
        MarkdownDictionaryKey::H5 => {
            format!(
                "<h5>{}</h5>",
                markdown_line.replace("#####", "").trim().to_string()
            )
        }
        MarkdownDictionaryKey::H6 => {
            format!(
                "<h6>{}</h6>",
                markdown_line.replace("######", "").trim().to_string()
            )
        }
        MarkdownDictionaryKey::List => {
            format!(
                "<ul><li>{}</li></ul>",
                markdown_line
                    .replace("-", "")
                    .replace("*", "")
                    .trim()
                    .to_string()
            )
        }
        MarkdownDictionaryKey::Link => {
            let mut link_text = markdown_line.to_string();
            link_text.remove(0);
            link_text.remove(link_text.len() - 1);
            let link_text_vec: Vec<&str> = link_text.split("](").collect();
            format!(
                "<a href=\"{}\">{}</a>",
                link_text_vec[1].trim(),
                link_text_vec[0].trim()
            )
        }
        MarkdownDictionaryKey::Image => {
            let mut image_text = markdown_line.to_string();
            image_text.remove(0);
            image_text.remove(image_text.len() - 1);
            let image_text_vec: Vec<&str> = image_text.split("](").collect();
            format!(
                "<img src=\"{}\" alt=\"{}\">",
                image_text_vec[1].trim(),
                image_text_vec[0].replace("[", "").trim()
            )
        }
        MarkdownDictionaryKey::Breakline => "<br/>".to_string(),
        MarkdownDictionaryKey::Paragraph => {
            format!("<p>{}</p>", markdown_line.trim().to_string())
        }
    }
}
