use std::{collections::HashMap, cmp::max};

pub fn match_length(pattern: &String, mut idx1: usize, mut idx2: usize) -> usize {
    if idx1 == idx2 {
        return pattern.len() - idx1;
    }
    let pattern_i = pattern.as_bytes();
    let mut match_count:usize = 0;
    while idx1 < pattern.len() && idx2 < pattern.len() && pattern_i[idx1] == pattern_i[idx2] {
        match_count += 1;
        idx1 += 1;
        idx2 += 1;
    }
    return match_count;

}

pub fn preprocess(pattern: String) -> Vec<usize> {
    if pattern.len() == 0 {
        return Vec::new();
    }
    if pattern.len() == 1 {
        return vec![1]
    }
    
    let mut preprocessed:Vec<usize> = Vec::new();
    for _ in 0..pattern.len() {
        preprocessed.push(0)
    }
    preprocessed[0] = pattern.len();
    preprocessed[1] = match_length(&pattern, 0, 1);
    for i in 2..(1+preprocessed[1]) {
        preprocessed[i] = preprocessed[1] + 1 - i;
    }

    let mut lower_b:usize = 0;
    let mut upper_b:usize = 0;
    let mut offset:usize;
    let mut offset_from_upper:usize;
    let mut existing:usize;

    for i in (2 + preprocessed[1])..(pattern.len()) {
        if i <= upper_b {
            offset = i - lower_b;
            existing = preprocessed[offset];
            offset_from_upper = upper_b + 1 - i;
            if existing < offset_from_upper {
                preprocessed[i] = existing
            }
            else {
                preprocessed[i] = match_length(&pattern, offset_from_upper, upper_b + 1);
                if preprocessed[i] > 0 {
                    lower_b = i;
                    upper_b = i + preprocessed[i] - 1;
                }
            }
        }
    }
    return preprocessed;
}

pub fn bad_char_table(pattern: String) -> HashMap<char, usize> {
    let mut table = HashMap::new();
    for (i, c) in pattern.chars().enumerate(){
        let mut offset = 1;
        let temp_offset = pattern.len()-i-1;
        if temp_offset > offset{
            offset = temp_offset;
        }
        table.insert(c, offset);
    }
    return table;
}

pub fn good_suff_table(pattern: String) -> Vec<i32> {
    let mut l:Vec<i32> = Vec::new();
    for _ in 0..pattern.len() {
        l.push(-1);
    }
    let mut preproc = preprocess(pattern.chars().rev().collect::<String>());
    preproc.reverse();
    let mut i:usize;
    for j in 0..(pattern.len() - 1) {
        i = pattern.len() - preproc[j];
        if i != pattern.len() {
            l[i] = j as i32;
        }
    }
    return l;
}

pub fn full_shift_table(pattern: String) -> Vec<usize> {
    let mut f:Vec<usize> = Vec::new();
    for _ in 0..pattern.len() {
        f.push(0)
    } 
    let mut z = preprocess(pattern);
    z.reverse();
    let mut longest = 0;
    for (i, zv) in z.into_iter().enumerate() {
        if zv == i + 1 {
            longest = max(zv, longest);
        }
        let len = f.len();
        f[len - i - 1] = longest;
    }
    return f
}

pub fn string_search(source: &str, pattern: &String) -> Vec<usize>{
    let mut indices: Vec<usize> = Vec::new();
    if source.len() == 0 || pattern.len() == 0 || source.len() < pattern.len(){
        return indices;
    }
    let bad_char = bad_char_table(pattern.clone().to_string());
    let good_suff = good_suff_table(pattern.clone().to_string());
    let full = full_shift_table(pattern.clone().to_string());
    let mut idx = (pattern.len() - 1) as i32;
    let mut previdx = None;

    let pattern_b = pattern.as_bytes();
    let source_b = source.as_bytes();
    while idx < source.len() as i32{
        let mut i = (pattern.len() - 1) as i32;
        let mut j = idx;
        while i >= 0 && (pattern_b[i as usize] as char) == (source_b[j as usize] as char){
            if previdx.is_none() || (previdx.is_some() && j > previdx.unwrap()){
                i -= 1;
                j -= 1;
            }
        }
        if i == -1 || (previdx.is_some() && j == previdx.unwrap()) {
            indices.push(idx as usize + 1);
            if pattern.len() > 1{
                idx += pattern.len() as i32 - full[1] as i32;
            }
            else{
                idx += 1;
            }
        }
        else{
            let bad = bad_char.get(&(source_b[j as usize] as char));
            let char_shift:i32;
            if bad.is_none(){
                char_shift = i + 1;
            }
            else{
                char_shift = i - *bad.unwrap() as i32;
            }
            let suffix_shift:i32;
            if i + 1 == pattern.len() as i32{
                suffix_shift = 1;
            }
            else if good_suff[i as usize + 1] == -1{
                suffix_shift = pattern.len() as i32 - full[i as usize + 1] as i32
            }
            else{
                suffix_shift = pattern.len() as i32 - good_suff[i as usize + 1] as i32
            }
            let mut shift = char_shift;
            if shift < suffix_shift{
                shift = suffix_shift;
            }
            if shift >= i + 1{
                previdx = Some(idx);
            }
            idx += shift;
        }
    }
    return indices;
}