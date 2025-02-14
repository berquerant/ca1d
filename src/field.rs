use crate::rule::{Rule, State};
use std::iter::Iterator;

pub struct Field<'a> {
    values: Vec<bool>,
    on_mark: &'a str,
    off_mark: &'a str,
    buffer: Vec<bool>,
    rule: Rule,
}

impl<'a> Field<'a> {
    pub fn new(values: Vec<bool>, rule: Rule, on_mark: &'a str, off_mark: &'a str) -> Self {
        let buffer = vec![false; values.len()];
        Field {
            values,
            on_mark,
            off_mark,
            rule,
            buffer,
        }
    }
    pub fn field_iter(self, n: usize) -> FieldIterator<'a> {
        FieldIterator {
            field: self,
            i: 0,
            n,
        }
    }
    pub fn as_readable_string(&self) -> String {
        let v: Vec<String> = self
            .values
            .iter()
            .map(|x| match *x {
                true => self.on_mark.to_owned(),
                false => self.off_mark.to_owned(),
            })
            .collect();
        v.join("")
    }
    fn get(&self, index: i64) -> bool {
        if index < 0 {
            return false;
        }
        let i = index as usize;
        i < self.values.len() && self.values[i]
    }
    fn new_state(prev: bool, center: bool, next: bool) -> State {
        let mut v = 0u8;
        if prev {
            v += 0b100;
        }
        if center {
            v += 0b010;
        }
        if next {
            v += 0b001;
        }
        State::try_from(v).unwrap()
    }
    pub fn update(&mut self) {
        for i in 0..self.values.len() {
            let ii = i as i64;
            let prev = self.get(ii - 1);
            let center = self.get(ii);
            let next = self.get(ii + 1);
            let state = Self::new_state(prev, center, next);
            let next_state = self.rule.apply(state);
            self.buffer[i] = next_state;
        }
        self.values = self.buffer.clone();
    }
}

pub struct FieldIterator<'a> {
    field: Field<'a>,
    i: usize,
    n: usize,
}

impl Iterator for FieldIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n {
            return None;
        }
        self.i += 1;
        self.field.update();
        Some(self.field.as_readable_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_readable_string() {
        let f = Field::new(vec![false, true, true], Rule::new(0), "1", "0");
        let got = f.as_readable_string();
        assert_eq!("011".to_owned(), got);
    }
}
