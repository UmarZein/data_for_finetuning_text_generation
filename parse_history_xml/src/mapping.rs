use maplit::hashmap;

pub fn translate(s: &str) -> String {
    let hm = hashmap! {
        "&tab;" => "\t",
        "&newline;" => "\n",
        "&excl;" => "!",
        "&Quot;" => "\"",
        "&quot;" => "\"",
        "&Quot"  => "\"",
        "&quot"  => "\"",
        "&num;" => "#",
        "&dollar;" => "$",
        "&percnt;" => "%",
        "&amp;" => "&",
        "&Amp;" => "&",
        "&Amp"  => "&",
        "&amp"  => "&",
        "&apos;" => "'",
        "&lpar;" => "(",
        "&rpar;" => ")",
        "&ast;" => "*",
        "&midast;" => "*",
        "&plus;" => "+",
        "&comma;" => ",",
        "&period;" => ".",
        "&sol;" => "/",
        "&colon;" => ":",
        "&semi;" => ";",
        "&Lt;" => "<",
        "&lt;" => "<",
        "&Lt"  => "<",
        "&lt"  => "<",
        "&equals;" => "=",
        "&Gt;" => ">",
        "&gt;" => ">",
        "&gt"  => ">",
        "&Gt"  => ">",
        "&quest;" => "?",
        "&commat;" => "@",
        "&lsqb;" => "[",
        "&lbrack;" => "[",
        "&bsol;" => "\\",
        "&rqsb;" => "]",
        "&rbrack;" => "]",
        "&Hat;" => "^",
        "&circ;" => "^",
        "&lowbar;" => "_",
        "&UnderBar;" => "_",
        "&grave;" => "`",
        "&DiacriticalGrave;" => "`",
        "&lclub;" => "{",
        "&lbrace;" => "{",
        "&verbar;" => "|",
        "&vert;" => "|",
        "&VerticalLine;" => "|",
        "&rclub;" => "}",
        "&rbrace;" => "}",
        "&nbsp;" => " ",
        "&hyphen;" => "-",
        "&dash;" => "-",
        "&ndash;" => "-",

        "&ldquo;" => "\"",
        "&OpenCurlyDoubleQuote;" => "\"",
        "&rdquo;" => "\"",
        "&rdquor;" => "\"",
        "&CloseCurlyDoubleQuote;" => "\"",

        "&bull;" => "-",
        "&bullet;" => "-",

    };
    let tmp = s.to_lowercase(); 
    let tmp = tmp.as_str();
    match hm.get(&tmp){
        Some(v) => v.clone().into(),
        None => "".into(),
    }
}
