use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    about = "Base is the Base of the Converter; Hex, Octal, Binary are supported as of now
    \nNumber is when you want to number to base, and code is the opposite"
)]
pub struct ClapOptions {
    #[arg(long, short)]
    pub base: String,

    #[arg(long, conflicts_with = "code")]
    pub number: Option<usize>,

    #[arg(short, long, conflicts_with = "number")]
    pub code: Option<String>,

    #[arg(long, short, requires = "code")]
    pub target: Option<String>,
}

/*
Target: [BASE] with input code [CODE] with base [BASE]
Output: [OUTPUT]
*/
