use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut bt = BTreeMap::new();

    for (p, chars) in h {
        for ch in chars {
            bt.insert(ch.to_ascii_lowercase(), *p);
        }
    }
    bt
}
