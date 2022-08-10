mod utils;

// use utils::grammar::*;
// use utils::nfa::*;
// use utils::state_utils::*;
use utils::boyer_moore::*;
use std::env;
use utils::regex::*;
use utils::earley_parse::*;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let regex = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("File Not Found");
    let matching_str = get_match(regex, &contents.to_string());
    for m in matching_str{
        println!("{}", m);
    }
}

pub fn run_cases(cases: Vec<(&str, &str, bool)>){
    for (regex, to_match, compare) in cases{
        let matching_str = get_match(regex, to_match);
        for m in &matching_str{
            println!("{}", m);
        }
        assert!(!matching_str.is_empty() == compare);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_arith() {
        let mut g = CFG::new("EXP");
        g.add_rule("EXP", vec![nt("EXP"), tr('-'), nt("EXP")]);
        g.add_rule("EXP", vec![nt("TERM")]);

        g.add_rule("TERM", vec![nt("TERM"), tr('/'), nt("TERM")]);
        g.add_rule("TERM", vec![nt("FACTOR")]);

        g.add_rule("FACTOR", vec![tr('('), nt("EXP"), tr(')')]);
        for a in '0'..='9' {
            g.add_rule("FACTOR", vec![tr(a)]);
        }

        assert!(parse("5--5", &g).is_none());
        assert!(parse("5-5", &g).is_some());

        let result = parse("(5-5)/(2-3/4)", &g);
        assert!(result.is_some());
        println!("{:#?}", PrettyPrint(&result.unwrap().collapse()));
    }
    #[test]
    fn test_ours() {
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("\\D?", "123", false));
        run_cases(cases);
    }

    #[test]
    fn test_basic() {
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("a|b|c|123", "123", true));
        cases.push(("a|b|c|d", "123", false));
        cases.push(("123", "abcde 123", true));
        
        run_cases(cases);
    }

    //e-transition weirdness
    #[test]
    fn test_dot(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push((".?", "asdalo1234kjl123bnrfqw192578rfb''sadfasdfnbn\\/asd;lm'", true));
        run_cases(cases);
    }

    #[test]
    fn test_slashes(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("\\s", "123", false));
        cases.push(("\\w", "123", false));
        cases.push(("\\d", "123", true));

        cases.push(("\\s", "abc", true));
        cases.push(("\\w", "abc", false));
        cases.push(("\\d", "abc", false));

        cases.push(("\\s", "   ", false));
        cases.push(("\\w", "   ", true));
        cases.push(("\\d", "   ", false));

        run_cases(cases);
    }

    #[test]
    fn test_slashes_more(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("\\s", "123abc   ", true));
        cases.push(("\\w", "123abc   ", true));
        cases.push(("\\d", "123abc   ", true));
        run_cases(cases);
    }

    #[test]
    pub fn test_not_slashes(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("\\D", "123", false));
        cases.push(("\\D", "abc", true));
        cases.push(("\\D", "   ", true));

        cases.push(("\\S", "123", true));
        cases.push(("\\S", "abc", false));
        cases.push(("\\S", "   ", true));

        cases.push(("\\W", "123", true));
        cases.push(("\\W", "abc", true));
        cases.push(("\\W", "   ", false));

        run_cases(cases);
    }

    #[test]
    pub fn test_python(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("Python|Perl", "Perl", true));
        cases.push(("(Python|Perl)", "Perl", true));
        cases.push(("(\\s\\d)|(\\s\\s)", "a1", true));
        cases.push(("(\\s\\d)|(\\s\\s)", "aa", true));

        run_cases(cases);
    }

    #[test]
    pub fn test_complicated(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("(ab)+", "abababwhatabab", true));
        cases.push(("(ab)+", "abababwhatabab\nwtfab", true));
        cases.push(("(ab)+|(cd)?", "abababwhatabab\nwtfab\nkljbkasjldfasdabababababababab", true));
        cases.push(("(ab)+", "abababwhatabab\nwtfab\nkljbkasjldfasdabababababababab", true));
        cases.push(("(ab)?", "abababwhatabab\nwtfab", true));
        run_cases(cases);
    }

    #[test]
    pub fn test_boyer(){
        let idxs = string_search("Hello I am Bobby Daigle. ABABABAB", &"Bobby".to_string());
        println!("{:?}", idxs);
    }

    #[test]
    pub fn test_prefix(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("(ab)+wh", "abababwhatabab", true));
        run_cases(cases);
    }

    #[test]
    pub fn test_dot_specials(){
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("(\\.)*", ".aasdasdasd\n.asdgik...das", true));
        cases.push(("\\?", "asdasdasd?123123", true));
        run_cases(cases);
    }

    #[test]
    pub fn test_theirs() {
        let mut cases: Vec<(&str, &str, bool)> = Vec::new();
        cases.push(("a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaa", true));
        cases.push(("a(1q?)|b2(abcq)?|3p+|3(abcp)+|4s*|4(abcs)*", "a\na1\na1q\nb\nb2\nb2abcq\n3\n3p3p\n3pp\n3abcpabcp\n4\n4s4s\n4ssss\n4abcsabcs", true));
        cases.push(("	 !\"#$&'\\(\\)\\*\\+,-\\./0123456789:;<=>\\?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]^_`abcdefghijklmnopqrstuvwxyz\\|", "	 !\"#$&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz|", true));
        cases.push(("\\W+ENDw|\\S+ENDs|\\D+ENDd", "	 	 ENDw\nABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzENDs\n0123456789ENDd\nABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ENDw\n	 	 0123456789ENDs\n	 	 ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzENDd", true));
        run_cases(cases);
    }
}
