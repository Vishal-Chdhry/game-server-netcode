use clap::Parser;

#[derive(Parser)]
#[clap()]
pub struct ServerArgs {
    #[clap(long = "port_start", short = 's', default_value_t = 42069)]
    pub port_start: u16,

    #[clap(long = "port_end", short = 'e', default_value_t = 43069)]
    pub port_end: u16,

    #[clap(long = "players", short = 'p', default_value_t = 100)]
    pub players_per_game: usize,
}
