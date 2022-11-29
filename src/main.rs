mod field;
mod random;
mod rule;
use clap::Parser;
use random::BoolRng;
use std::env::var;

fn main() {
    let cli = Cli::parse();

    let rand_seed = match var("CA1D_RANDOM_SEED") {
        Ok(seed) => Some(seed.parse::<u64>().unwrap()),
        Err(_) => None,
    };
    let mut rgn = BoolRng::new(rand_seed);

    let field_seed = match var("CA1D_FIELD_SEED") {
        Ok(seed) => {
            let mut v = vec![false; cli.size];
            for (x, s) in v.iter_mut().zip(seed.chars()) {
                *x = s == '1';
            }
            v
        }
        Err(_) => rgn.gen_bools(cli.size),
    };

    let f = field::Field::new(
        field_seed,
        rule::Rule::new(cli.rule_code),
        &cli.on_mark,
        &cli.off_mark,
    );
    println!("{}", f.into_readable_string()); // output initial field
    let fit = f.field_iter(cli.generation);
    fit.for_each(|x| println!("{}", x));
}

#[derive(Parser, Debug)]
/// Execute one-dimensional cellular automaton.
///
/// Environmental Variables:
///
///   CA1D_RANDOM_SEED:
///     Seed integer of the random number generator.
///
///   CA1D_FIELD_SEED:
///     Initial state of the automaton.
///     0 means off state and 1 means on state.
///     If the string is short to fill the field, the rest is filled by off state.
///     e.g. field size is 5 and the string is "001" then the initial state will be "00100".
#[clap(verbatim_doc_comment)]
struct Cli {
    /// Field width.
    #[clap(short = 'f', long, default_value_t = 120)]
    size: usize,
    /// Wolfram code.
    #[clap(short = 'r', long, default_value_t = 30)]
    rule_code: u8,
    /// Number of turns to proceed.
    #[clap(short = 'n', long, default_value_t = 40)]
    generation: usize,
    /// Off state mark.
    #[clap(short = '0', long, default_value = " ")]
    off_mark: String,
    /// On state mark.
    #[clap(short = '1', long, default_value = "1")]
    on_mark: String,
}
