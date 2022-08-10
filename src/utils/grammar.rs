use super::earley_parse::*;
use std::collections::HashSet;

pub fn our_grammar() -> CFG {
    let special = HashSet::from([ '|', '*', '(', ')', '.', '+', '?', '\\']);
    let mut g = CFG::new("RE");

    g.add_rule("RE", vec![nt("UNION")]);
    g.add_rule("UNION", vec![nt("UNION"), tr('|'), nt("CONCAT")]);
    g.add_rule("UNION", vec![nt("CONCAT")]);

    g.add_rule("CONCAT", vec![nt("CONCAT"), nt("COUNTS")]);
    g.add_rule("CONCAT", vec![nt("COUNTS")]);

    g.add_rule("COUNTS", vec![nt("COUNTS"), tr('*')]);
    g.add_rule("COUNTS", vec![nt("COUNTS"), tr('+')]);
    g.add_rule("COUNTS", vec![nt("COUNTS"), tr('?')]);
    g.add_rule("COUNTS", vec![nt("PAREN")]);

    g.add_rule("PAREN", vec![tr('('), nt("RE"), tr(')')]);
    g.add_rule("PAREN", vec![nt("TERM")]);

    g.add_rule("TERM", vec![nt("LET")]);
    g.add_rule("TERM", vec![nt("SP")]);
    g.add_rule("TERM", vec![nt("DGT")]);
    g.add_rule("TERM", vec![nt("WS")]);


    g.add_rule("WS", vec![tr(char::from_u32(0x09).unwrap())]);
    g.add_rule("WS", vec![tr(char::from_u32(0x20).unwrap())]);

    for n in 0x21..0x30 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        if !special.contains(&c) {
            g.add_rule("SP", vec![tr(c)]);
        }
    }

    for n in 0x30..0x3a {
        let option = char::from_u32(n);
        let c = option.unwrap();
        g.add_rule("DGT", vec![tr(c)])
    }

    for n in 0x3a..0x41 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        if !special.contains(&c) {
            g.add_rule("SP", vec![tr(c)]);
        }
    }

    for n in 0x41..0x5b {
        let option = char::from_u32(n);
        let c = option.unwrap();
        g.add_rule("LET", vec![tr(c)])
    }
    
    for n in 0x5b..0x61 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        if !special.contains(&c) {
            g.add_rule("SP", vec![tr(c)]);
        }
    }

    for n in 0x61..0x7b {
        let option = char::from_u32(n);
        let c = option.unwrap();
        g.add_rule("LET", vec![tr(c)])
    }

    for n in 0x7b..0x80 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        if !special.contains(&c) {
            g.add_rule("SP", vec![tr(c)]);
        }
    }

    g.add_rule("TERM", vec![nt("SP")]);
    for c in special.iter(){
        g.add_rule("SP", vec![tr('\\'), tr(*c)]);
    }

    g.add_rule("TERM", vec![nt("DOT")]);
    g.add_rule("DOT", vec![tr('.')]);

    g.add_rule("LET", vec![tr('\\'), tr('s')]);
    g.add_rule("DGT", vec![tr('\\'), tr('d')]);
    g.add_rule("WS", vec![tr('\\'), tr('w')]);

    g.add_rule("TERM", vec![nt("NOTLET")]);
    g.add_rule("TERM", vec![nt("NOTDGT")]);
    g.add_rule("TERM", vec![nt("NOTWS")]);

    g.add_rule("NOTLET", vec![tr('\\'), tr('S')]);
    g.add_rule("NOTDGT", vec![tr('\\'), tr('D')]);
    g.add_rule("NOTWS", vec![tr('\\'), tr('W')]);

    return g;
}