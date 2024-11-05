use std::collections::HashMap;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let window_len = p.len();
    if window_len > s.len() {
        return vec![];
    }

    let needle = group_string_chars(&p);
    let mut window = group_string_chars(&s[0..window_len]);

    let mut res = if needle == window { vec![0] } else { vec![] };

    let chars: Vec<_> = s.chars().collect();
    for (i, c) in chars.iter().copied().enumerate().skip(window_len) {
        let old_i = i - window_len;
        let old_c = chars.get(old_i).unwrap();
        let old_entry = window.remove_entry(old_c).unwrap();
        if old_entry.1 > 1 {
            window.insert(old_entry.0, old_entry.1 - 1);
        }

        window.entry(c).and_modify(|n| *n += 1).or_insert(1);

        if needle == window {
            res.push(old_i as i32 + 1);
        }
    }

    res
}

pub fn group_string_chars(s: impl AsRef<str>) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    for c in s.as_ref().chars() {
        res.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }
    res
}

#[test]
fn test() {
    assert_eq!(find_anagrams("cbaebabacd".into(), "abc".into()), vec![0, 6]);
    assert_eq!(find_anagrams("abab".into(), "ab".into()), vec![0, 1, 2]);
    assert_eq!(find_anagrams("baa".into(), "aa".into()), vec![1]);
}
