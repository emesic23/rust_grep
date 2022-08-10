use super::nfa::*;
use super::grammar::*;
use super::earley_parse::*;
use super::boyer_moore::*;

pub fn get_match(regex: &str, to_match: &str) -> Vec<String>{
    let g = our_grammar();

    let result = parse(regex, &g);
    assert!(result.is_some());

    let nfa = cfg2nfa(result.unwrap());
    let mut start = nfa.0;
    let mut states = nfa.1;
    let prefix_ex = prefix_extraction(start, &mut states);
    start = prefix_ex.0;
    let prefix = prefix_ex.1;
    states = prefix_ex.2;

    let match_starts = string_search(to_match, &prefix);
    // println!("Prefix: {:?}, Occur: {:?}", prefix, match_starts);
    
    let matching_str = matching(start, &mut states, to_match, prefix.len(), match_starts);
    return matching_str;
}
