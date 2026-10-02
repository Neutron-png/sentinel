use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SequencerError {
    Empty,
    TooFewSamples { got: usize, required: usize },
}

impl fmt::Display for SequencerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "no token samples provided"),
            Self::TooFewSamples { got, required } => {
                write!(f, "got {got} token samples, at least {required} are required for statistical analysis")
            }
        }
    }
}

impl std::error::Error for SequencerError {}
