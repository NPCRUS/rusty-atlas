pub struct FlagDescription(&'static str, &'static str);

pub static VERBOSITY_FLAG_DESC: FlagDescription = FlagDescription("v", "verbose");