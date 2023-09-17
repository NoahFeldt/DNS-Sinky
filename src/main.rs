mod server;
mod lists;
mod config;

use config::Config;
use lists::Blacklist;

fn main() {
    // Path for storing config file
    let config_path = "config.json";

    // Creates config
    let config = match Config::load(config_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Using default config!");
            let cfg = Config::default();
            let _ = cfg.save(config_path);
            cfg
        }
    };

    // Path for storing compressed blacklist file
    let blacklist_path = "blacklist.gz";

    // Creates blacklist
    let blacklist = if config.update_on_startup == false {
        match Blacklist::load(blacklist_path) {
            Ok(b) => b,
            Err(_) => {
                let bl = Blacklist::create(&config).unwrap();
                let _ = bl.save(blacklist_path);
                bl
            },
        }
    } else {
        let bl = Blacklist::create(&config).unwrap();
        let _ = bl.save(blacklist_path);
        bl
    };

    // Starts server
    server::server(&blacklist, &config).unwrap();
}
