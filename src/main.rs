use regex::Regex;
use std::process::exit;
fn main() {
    let re = Regex::new(r"<([^>\s]+)([^>]*)>([^<]+)<\/([^>\s]+)>").unwrap();
    let hay = r#"
    <a onclick="callthis()">it works!!</a>
    <script>
    function callthis() {
        alert("You clicked!");
    }
    </script>
    "#;
    let mut lineno = 0;
    let mut tags: Vec<(&str, &str, &str, &str)> = Vec::new();
    
    for (_, [begin_tag, attributes, text, end_tag]) in re.captures_iter(hay).map(|x| x.extract()) {
        lineno += 1;
        if begin_tag != end_tag {
            eprintln!("ERROR ON LINE {lineno}, beginning tag is not the same as end_tag ({begin_tag} != {end_tag}");
            exit(1);
        }
        tags.push((begin_tag, attributes, text, end_tag));
    }
    for (lineno, (begin, attributes, text, end)) in tags.into_iter().enumerate() {
        let lineno = lineno + 1;
        println!("{lineno} <{begin}{attributes}>{text}</{end}>");
    }


}