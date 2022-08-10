use std::{collections::HashSet};

use super::earley_parse::*;
use super::state_utils::*;

pub fn construct_nfa(curr: &ASTNode, states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    match curr.sym {
        Symbol::Terminal(c) => {
            if c == '.' {
                create_dot_state(states, frag_stack);
            }
            else {
                terminal_state_from_char(c, states, frag_stack);
            }
        }
        Symbol::NonTerminal(ref s) => {
            match s.as_str() {
                "PAREN" => {
                    construct_nfa(&curr.children[1], states, frag_stack);
                }
                "CONCAT" => {
                    construct_nfa(&curr.children[0], states, frag_stack);
                    construct_nfa(&curr.children[1], states, frag_stack);
                    let e2 = frag_stack.pop().unwrap();
                    let e1 = frag_stack.pop().unwrap();
                    patch(e1, e2, states);
                    frag_stack.push(e1);
                }
                "UNION" => {
                    construct_nfa(&curr.children[0], states, frag_stack);
                    construct_nfa(&curr.children[2], states, frag_stack);
                    let e2 = frag_stack.pop().unwrap();
                    let e1 = frag_stack.pop().unwrap();
                    let new_state = split_state(vec![Some(e1), Some(e2)]);
                    frag_stack.push(states.len());
                    states.push(new_state);
                }
                "COUNTS" => {
                    construct_nfa(&curr.children[0], states, frag_stack);
                    let e = frag_stack.pop().unwrap();
                    let mut new_state = split_state(vec![Some(e)]);
                    let new_state_location = states.len();
                    match curr.children[1].sym {
                        Symbol::Terminal('*') => {
                            patch(e, new_state_location, states);
                            frag_stack.push(new_state_location);
                        }
                        Symbol::Terminal('?') => {
                            frag_stack.push(new_state_location);
                        }
                        Symbol::Terminal('+') => { 
                            patch(e, new_state_location, states);
                            frag_stack.push(e);
                        }
                        _ => {}
                    }
                    new_state.out.push(Transition{c: None, state: None});
                    states.push(new_state)
                }
                "LET" => {
                    let_states(states, frag_stack);
                }
                "DGT" => {
                    dgt_states(states, frag_stack);
                }
                "WS" => {
                    ws_states(states, frag_stack);
                }
                "NOTLET" => {
                    dgt_states(states, frag_stack);
                    ws_states(states, frag_stack);
                    sp_states(states, frag_stack);

                    let mut states_vec: Vec<State> = Vec::new();
                    for _ in 0..3 {
                        states_vec.push(states[frag_stack.pop().unwrap()].clone());
                    }
                    let new_state = merge_non_match_states(states_vec);
                    frag_stack.push(states.len());
                    states.push(new_state);
                }
                "NOTDGT" => {
                    let_states(states, frag_stack);
                    ws_states(states,  frag_stack);
                    sp_states(states, frag_stack);

                    let mut states_vec: Vec<State> = Vec::new();
                    for _ in 0..3 {
                        states_vec.push(states[frag_stack.pop().unwrap()].clone());
                    }
                    let new_state = merge_non_match_states(states_vec);
                    frag_stack.push(states.len());
                    states.push(new_state);
                }
                "NOTWS" => {
                    let_states(states, frag_stack);
                    dgt_states(states, frag_stack);
                    sp_states(states, frag_stack);

                    let mut states_vec: Vec<State> = Vec::new();
                    for _ in 0..3 {
                        states_vec.push(states[frag_stack.pop().unwrap()].clone());
                    }
                    let new_state = merge_non_match_states(states_vec);
                    frag_stack.push(states.len());
                    states.push(new_state);
                }
                "SP" => {
                    match &curr.children[1].sym {
                        Symbol::Terminal(c) => {
                            if *c == '.' {
                                terminal_state_from_char(*c, states, frag_stack);
                            }
                            else {
                                construct_nfa(&curr.children[1], states, frag_stack);
                            }
                        }
                        Symbol::NonTerminal(_) => {}
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn ep_expansion(states: &mut Vec<State>) {
    let mut change:bool = true;

    while change {
        change = false;

        let state_len = states.len();
        let states_clone = states.clone();
        for i in 0..state_len {
            let curr_state = &mut states[i];
            let curr_out_len = curr_state.out.len();
            for j in 0..curr_out_len {
                let curr_transition = &curr_state.out[j];

                if curr_transition.c == None {
                    let next_state = &states_clone[curr_transition.state.unwrap()];
                    for next_transition in next_state.out.iter() {
                        if next_transition.c == None {
                            let temp_set: HashSet<Transition> = curr_state.out.iter().cloned().collect();
                            if !temp_set.contains(next_transition) {
                                curr_state.out.push(next_transition.clone());
                                change = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn ep_match_fix(states: &mut Vec<State>) {
    let mut change:bool = true;

    while change {
        change = false;

        let state_len = states.len();
        let states_clone = states.clone();
        for i in 0..state_len {
            let mut curr_state = &mut states[i];
            let curr_out_len = curr_state.out.len();
            for j in 0..curr_out_len {
                let curr_transition = &curr_state.out[j];

                if curr_transition.c == None {
                    let next_state = &states_clone[curr_transition.state.unwrap()];
                    if next_state.is_match && !curr_state.is_match {
                        curr_state.is_match = true;
                        change = true;
                    }
                }
            }
        }
    }
}

pub fn ep_removal(states: &mut Vec<State>) {
    let mut change:bool = true;

    while change {
        change = false;

        let state_len = states.len();
        let states_clone = states.clone();
        for i in 0..state_len {
            let curr_state = &mut states[i];
            let curr_out_len = curr_state.out.len();
            let mut to_remove: Vec<usize> = Vec::new();
            for j in 0..curr_out_len {
                let curr_transition = &curr_state.out[j];

                if curr_transition.c == None {
                    to_remove.push(j);
                    let next_state = &states_clone[curr_transition.state.unwrap()];
                    for next_transition in next_state.out.iter() {
                        let temp_set: HashSet<Transition> = curr_state.out.iter().cloned().collect();
                        if !temp_set.contains(next_transition) {
                            curr_state.out.push(next_transition.clone());
                            change = true;
                        }
                    }
                }
            }
            let mut offset = 0;
            for j in to_remove.iter() {
                curr_state.out.remove(j - offset);
                offset += 1;
            }
        }
    }    
}

// TODO Made ASTNode and strval for symbols public, is that ok? Not sure what else to do
// except matching which seems excessive
//getters? setters? sadge sadge
pub fn cfg2nfa(grammar: ASTNode) -> (usize, Vec<State>) {
    let mut states: Vec<State> = Vec::new();
    let mut frag_stack: Vec<usize> = Vec::new();

    let grammar = grammar.collapse();

    construct_nfa(&grammar, &mut states, &mut frag_stack);

    let match_state = match_state();
    states.push(match_state);
    patch(frag_stack[0], states.len() - 1, &mut states);
    ep_expansion(&mut states);
    ep_match_fix(&mut states);
    ep_removal(&mut states);
    return (frag_stack.pop().unwrap(), states);
}

pub fn prefix_extraction(start: usize, states: &mut Vec<State>) -> (usize, String, Vec<State>) {
    let mut next_states: HashSet<Transition> = HashSet::new();
    let mut curr_states: HashSet<Transition> = HashSet::new();
    let mut states: Vec<State> = states.clone();
    let mut seen: HashSet<usize> = HashSet::new();
    let mut new_start = start;
    let mut prefix:Vec<char> = Vec::new();
    let mut diff_transition = false;

    let start_transition =Transition {c: None, state: Some(start)};
    curr_states.insert(start_transition.clone());
    next_states.insert(start_transition);
    while !diff_transition && !next_states.is_empty(){
        next_states = HashSet::new();
        let mut transition: Option<char> = None;
        for curr in curr_states.iter() {
            let state_loc = curr.state.unwrap();
            let curr_state = &states[state_loc];
            for out in curr_state.out.iter() {
                let c = out.c.unwrap();
                if transition == None {
                    transition = Some(c);
                }
                else if transition.unwrap() != c {
                    diff_transition = true
                }
                if !seen.contains(&out.state.unwrap()) {
                    next_states.insert(out.clone());
                    seen.insert(out.state.unwrap());
                }
            }
        }
        if !diff_transition && transition != None && !next_states.is_empty() {
            prefix.push(transition.unwrap());
            curr_states = next_states.clone();
        }
    }

    if prefix.len() != 0 {
        new_start = states.len();
        let mut is_match: bool = false;
        for curr in curr_states.iter() {
            let curr_state = &states[curr.state.unwrap()];
            if curr_state.is_match {
                is_match = true;
            }
        }
        next_states = HashSet::new();
        for curr in curr_states.iter() {
            let state_loc = curr.state.unwrap();
            let curr_state = &states[state_loc];
            for out in curr_state.out.iter() {
                    next_states.insert(out.clone());
                }
            }
        let mut new_state = split_state_with_transitions(next_states.into_iter().collect());
        new_state.is_match = is_match;
        states.push(new_state);
    }

    return (new_start, prefix.into_iter().collect(), states);
}

pub fn matching(start: usize, states: &mut Vec<State>, string: &str, prefix_length: usize, match_starts: Vec<usize>) -> Vec<String> {
    let mut curr_states: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut match_substr: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut linenum = 1;

    match_substr.push(Vec::new());
    let mut match_starts_count = 0;

    // Iterate through the string, moving forward in all present states as necessary
    for (i, c) in string.chars().enumerate() {
        // Adds start state of NFA
        // if there is no prefix, do so at every index
        // if there is an index, do so only where that prefix has been identified
        if prefix_length > 0 {
            if !match_starts.is_empty() && match_starts_count != match_starts.len(){
                if i == match_starts[match_starts_count] {
                    curr_states.insert((start, linenum, i - prefix_length));
                    match_starts_count += 1;
                }
            }
        }
        else {
            curr_states.insert((start, linenum, i));
        }

        // Newline logic
        if c == '\n'{
            for curr in curr_states.iter() {
                let curr_str_start = curr.2;
                let curr_state = &states[curr.0];
                if curr_state.is_match {
                    match_substr[linenum-1].push((curr_str_start, i));
                }
            }
            match_substr.push(Vec::new());
            linenum += 1;
            curr_states = HashSet::new();
            continue;
        }


        // If there exists a transition in any current state using the current 
        // character, traverse it, otherwise drop the state
        let mut next_states = HashSet::new();
        for curr in curr_states.iter() {
            let curr_str_start = curr.2;
            let curr_state = &states[curr.0];
            for transition in curr_state.out.iter() {
                if transition.c.unwrap() == c {
                    next_states.insert((transition.state.unwrap(), linenum, curr_str_start));
                }
            }
            if curr_state.is_match {
                match_substr[linenum-1].push((curr_str_start, i));
            }
        }
        curr_states = next_states;
    }

    // If any current state is a match state, add the substring so far to the 
    // list of matches
    for curr in curr_states.iter() {
        let curr_state = &states[curr.0];
        let curr_str_start = curr.2;
        let curr_str_line = curr.1;
        if curr_state.is_match {
            match_substr[curr_str_line-1].push((curr_str_start, string.len()));
        }
    }

    // If a match is just the prefix, this handles the case if the prefix is the
    // last n characters of the string
    if !match_starts.is_empty() {
        if match_starts[match_starts.len()-1] == string.len(){
            match_substr[linenum-1].push((string.len() - prefix_length, string.len()));
        }
    }

    // Remove overlapping matches, preferring the longest match
    let mut matching: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut curr_line = 0;
    for line in match_substr.iter_mut(){
        if line.is_empty(){
            curr_line += 1;
            matching.push(Vec::new());
            continue;
        }
        matching.push(Vec::new());
        line.sort_by_key(|k| k.0);
        let match_len = line.len();
        matching[curr_line].push(line[0]);
        for i in 1..match_len {
            let curr_len = matching[curr_line].len()-1;
            let curr = matching[curr_line][curr_len];
            if curr.1 <= line[i].0{
                matching[curr_line].push(line[i]);
            }
            else if curr.1 < line[i].1{
                let curr = (line[i].0, line[i].1);
                matching[curr_line].pop();
                matching[curr_line].push(curr);
            }
        }
        curr_line += 1;
    }

    // Correctly formats the match strings and returns them
    let mut result: Vec<String> = Vec::new();
    let mut curr_line = 1;
    for line in matching.iter() {
        for pair in line.iter() {
            let start = pair.0;
            let end = pair.1;
            let matching_str = (&string)[start..end].to_string();
            if !matching_str.is_empty(){
                result.push(format!("{}:{}", curr_line, matching_str));
            }
        }
        curr_line += 1;
    }
    
    return result;
}