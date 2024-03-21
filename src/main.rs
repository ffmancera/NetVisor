fn main() {
    let app = clap::Command::new("netvisor")
        .version(clap::crate_version!())
        .author("Fernando F. Mancera <ffmancera@riseup.net>")
        .about("NetVisor command line tool")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("show")
            .about("Show a picture of the current network topology")
        );
}
