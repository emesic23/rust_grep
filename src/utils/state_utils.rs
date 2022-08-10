use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition {
    pub c: Option<char>,
    pub state: Option<usize>
}

#[derive(Debug, Clone)]
pub struct State {
    pub out: Vec<Transition>,
    pub is_match: bool,
}

pub fn transition_state(c: char, out: Option<usize>) -> State {
    State { out: vec![Transition {c: Some(c), state: out}], is_match: false}

}

pub fn split_state(out: Vec<Option<usize>>) -> State {
    let mut new_out: Vec<Transition> = Vec::new();
    for curr_out in out.iter() {
        new_out.push(Transition {c: None, state: *curr_out});
    }
    State {
        out: new_out,
        is_match: false
    }
}

pub fn split_state_with_transitions(out: Vec<Transition>) -> State {
    State {
        out: out,
        is_match: false
    }
}

pub fn merge_non_match_states(states: Vec<State>) -> State {
    let mut new_out: Vec<Transition> = Vec::new();
    for curr_state in states.iter() {
        new_out.extend(curr_state.out.clone()); 
    }
    State { out: new_out, is_match: false}
}

pub fn match_state() -> State {
    State{
        out: Vec::new(),
        is_match: true
    }
}

pub fn patch(e1: usize, e2: usize, state_vec: &mut Vec<State>) {
    let mut stack: Vec<usize> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    stack.push(e1);

    while !stack.is_empty() {
        let e: usize = stack.pop().unwrap();
        if e != e2 {
            let state: &mut State = &mut state_vec[e];
            let out = &mut state.out;
            
            for curr_out in out.iter_mut() {
                if curr_out.state == None{
                    curr_out.state = Some(e2);
                }
                else {
                    let next_state = curr_out.state.unwrap();
                    if !seen.contains(&next_state) {
                        stack.push((curr_out.state).unwrap());
                    }
                } 
            }
            seen.insert(e);
        }
    }
}
pub fn terminal_state_from_char(c: char, states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    let state = transition_state(c, None);
    frag_stack.push(states.len());
    states.push(state);
} 

pub fn let_states(states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    let mut out: Vec<Transition> = Vec::new();
    for n in 0x41..0x5b {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }
    for n in 0x61..0x7b {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }
    let new_state = split_state_with_transitions(out);
    frag_stack.push(states.len());
    states.push(new_state);
}

pub fn dgt_states(states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    let mut out: Vec<Transition> = Vec::new();
    for n in 0x30..0x3a {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }
    let new_state = split_state_with_transitions(out);
    frag_stack.push(states.len());
    states.push(new_state);
}

pub fn ws_states(states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    let mut out: Vec<Transition> = Vec::new();

    out.push(Transition {c: char::from_u32(0x09), state: None});
    out.push(Transition {c: char::from_u32(0x20), state: None});

    let new_state = split_state_with_transitions(out);
    frag_stack.push(states.len());
    states.push(new_state);
}


pub fn sp_states(states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    let mut out: Vec<Transition> = Vec::new();

    for n in 0x21..0x30 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }

    for n in 0x3a..0x41 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }

    for n in 0x5b..0x61 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }

    for n in 0x7b..0x80 {
        let option = char::from_u32(n);
        let c = option.unwrap();
        out.push(Transition {c: Some(c), state: None});
    }

    let new_state = split_state_with_transitions(out);
    frag_stack.push(states.len());
    states.push(new_state);
}

pub fn create_dot_state(states: &mut Vec<State>, frag_stack: &mut Vec<usize>) {
    let_states(states, frag_stack);
    dgt_states(states, frag_stack);
    ws_states(states, frag_stack);
    sp_states(states, frag_stack);

    let mut states_vec: Vec<State> = Vec::new();
    for _ in 0..4 {
        let curr_frag = frag_stack.pop().unwrap();
        states_vec.push(states[curr_frag].clone());
    }
    let new_state = merge_non_match_states(states_vec);
    frag_stack.push(states.len());
    states.push(new_state);
}