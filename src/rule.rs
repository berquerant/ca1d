use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StateError {
    #[error("{0} is too big for State")]
    TooBig(u8),
}

#[derive(Debug, PartialEq)]
pub struct State(u8);

impl TryFrom<u8> for State {
    type Error = StateError;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        if item < 1 << 3 {
            Ok(State(item))
        } else {
            Err(Self::Error::TooBig(item))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule(u8);

impl Rule {
    pub fn new(wolfram_code: u8) -> Self {
        Rule(wolfram_code)
    }
    pub fn apply(&self, state: State) -> bool {
        (self.0 & (1 << state.0)) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_state_try_from {
        ($name:ident, $input:expr, $want:expr) => {
            #[test]
            fn $name() {
                let got = State::try_from($input);
                assert_eq!($want, got);
            }
        };
    }

    test_state_try_from!(state_from_zero, 0, Ok(State(0)));
    test_state_try_from!(state_from_one, 1, Ok(State(1)));
    test_state_try_from!(state_from_seven, 7, Ok(State(7)));
    test_state_try_from!(state_from_eight, 8, Err(StateError::TooBig(8)));

    #[test]
    fn rule_30_apply() {
        let rule = Rule::new(30);
        let want = [false, true, true, true, true, false, false, false];
        for (i, w) in want.iter().enumerate() {
            let state = State::try_from(i as u8).unwrap();
            let got = rule.apply(state);
            assert_eq!(*w, got);
        }
    }
}
