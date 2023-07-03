use markdown_to_html_rust::conversion::convert_markdown_to_html;

fn main() {
    // println!("Input the Markdown text:");
    // let mut input = String::new();
    // std::io::stdin()
    //     .read_line(&mut input)
    //     .expect("Error in reading the text");
    // let markdown_text = input.trim().to_owned();
    let markdown_text = "# Header1
    ## Header2
    ### Header3
    alsjdas ldas
    [link](https://www.google.com)
    ![image](https://www.sublimeinnovationtechnologies.com/assets/images/logo.png)
    - List item 1
    - List item 2"
        .to_string();
    println!("Input Markdown: \n{}", markdown_text);
    let html_text = convert_markdown_to_html(markdown_text);
    println!("Output HTML: \n{}", html_text);
}
