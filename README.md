# ca1d

```
$ ca1d --help
Execute one-dimensional cellular automaton.

Environmental Variables:

  CA1D_RANDOM_SEED:
    Seed integer of the random number generator.

  CA1D_FIELD_SEED:
    Initial state of the automaton.
    0 means off state and 1 means on state.
    If the string is short to fill the field, the rest is filled by off state.
    e.g. field size is 5 and the string is "001" then the initial state will be "00100".

Usage: ca1d [OPTIONS]

Options:
  -f, --size <SIZE>
          Field width
          
          [default: 120]

  -r, --rule-code <RULE_CODE>
          Wolfram code
          
          [default: 30]

  -n, --generation <GENERATION>
          Number of turns to proceed
          
          [default: 40]

  -0, --off-mark <OFF_MARK>
          Off state mark
          
          [default: " "]

  -1, --on-mark <ON_MARK>
          On state mark
          
          [default: 1]

  -h, --help
          Print help information (use `-h` for a summary)
```
