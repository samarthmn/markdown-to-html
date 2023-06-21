/*
   1. Input and Preparation: The first thing you need is to read the input markdown file. You will also need to prepare an empty output string that will hold your final HTML output.
   2. Line by Line Parsing: Loop through each line in the markdown file. For each line, you'll need to identify what type of content it holds (header, paragraph, list item, link, image, blockquote, etc.) to determine how to translate it to HTML.
   3. Identifying Markdown Syntax: For each line, identify the markdown syntax and map it to the equivalent HTML tag. Some of the conversions might be:
       - Headers: Lines starting with # are headers. The number of # symbols indicates the level of the header. For example, # maps to <h1>, ## maps to <h2>, etc.
       - Paragraphs: Any line that doesn't start with a special symbol (like #, *, -, >, etc.) is a paragraph. It should be wrapped in <p> tags.
       - List items: Lines starting with - or * are list items. If a list item immediately follows another list item, it should be included in the existing <ul> or <ol>. If not, a new <ul> or <ol> should be started.
       - Links: Markdown links are in the format [text](url). These should be converted to the equivalent HTML <a href="url">text</a>.
       - Images: Similar to links, but with an exclamation mark at the beginning, like ![alt text](url). These should be converted to <img src="url" alt="alt text">.
       - Blockquotes: Lines starting with > are blockquotes. They should be wrapped in <blockquote> tags.
       - Code: Text surrounded by back-ticks ` should be wrapped in <code> tags. For blocks of code surrounded by triple back-ticks ```, use <pre><code> tags.
   4. Nested Elements Handling: Certain elements can be nested inside others. For example, you can have a link inside a list item. Your algorithm needs to account for this by recursively checking for other elements within an element.
   5. Escaping HTML Characters: If the markdown includes HTML special characters (like <, >, &, etc.), these should be replaced with their corresponding HTML entities (&lt;, &gt;, &amp; respectively) to prevent them from being interpreted as HTML code.
   6. White Space and Empty Lines: You also need to account for white space and empty lines. Multiple consecutive empty lines should be converted into one, and leading/trailing white spaces should be removed.
   7. End Result: After the entire markdown text is parsed and the corresponding HTML is generated, the final HTML is written to an output file.
*/
fn main() {}
