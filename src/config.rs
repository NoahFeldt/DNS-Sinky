use std::{error::Error, io::{Write, Read}, fs::File};

use serde::{Serialize, Deserialize};

/// Struct containing configuration information
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// URLS of blacklists
    pub urls: Vec<String>,

    /// Wether to download blacklists again on startup
    pub update_on_startup: bool,
 
    /// Upstream DNS server
    pub upstream_dns: String,
}

impl Config {
    /// Saves config as JSON file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        println!("Saving config to file \"{}\"...", path);

        // Serialize config into JSON String
        let json_data = serde_json::to_string_pretty(&self)?;

        // Create save file
        let mut file = File::create(path)?;

        // Convert JSON String to UTF-8 and encode and write JSON to file
        file.write_all(json_data.as_bytes())?;

        println!("Finished saving config!");

        Ok(())
    }

    /// Loads config from file
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        println!("Loading config from file \"{}\"...", path);

        // Open file
        let mut file = File::open(path)?;

        // Create JSON data buffer
        let mut json_data = String::new();
        
        // Read data into buffer
        file.read_to_string(&mut json_data)?;

        // Create Config struct
        let config = serde_json::from_str(&json_data)?;

        println!("Finished loading config!");

        Ok(config)
    }
}

impl Default for Config {
    /// Default configuration
    fn default() -> Self {
        Config { 
            urls: vec![
                String::from("https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt"),
                String::from("https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts"),
                String::from("https://v.firebog.net/hosts/static/w3kbl.txt"),
                String::from("https://adaway.org/hosts.txt"),
                String::from("https://v.firebog.net/hosts/AdguardDNS.txt"),
                String::from("https://v.firebog.net/hosts/Admiral.txt"),
                String::from("https://raw.githubusercontent.com/anudeepND/blacklist/master/adservers.txt"),
                String::from("https://s3.amazonaws.com/lists.disconnect.me/simple_ad.txt"),
                String::from("https://v.firebog.net/hosts/Easylist.txt"),
                String::from("https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0&mimetype=plaintext"),
                String::from("https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts"),
                String::from("https://raw.githubusercontent.com/bigdargon/hostsVN/master/hosts"),
                String::from("https://v.firebog.net/hosts/Easyprivacy.txt"),
                String::from("https://v.firebog.net/hosts/Prigent-Ads.txt"),
                String::from("https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts"),
                String::from("https://raw.githubusercontent.com/crazy-max/WindowsSpyBlocker/master/data/hosts/spy.txt"),
                String::from("https://hostfiles.frogeye.fr/firstparty-trackers-hosts.txt"),
                String::from("https://raw.githubusercontent.com/DandelionSprout/adfilt/master/Alternate%20versions%20Anti-Malware%20List/AntiMalwareHosts.txt"),
                String::from("https://osint.digitalside.it/Threat-Intel/lists/latestdomains.txt"),
                String::from("https://s3.amazonaws.com/lists.disconnect.me/simple_malvertising.txt"),
                String::from("https://v.firebog.net/hosts/Prigent-Crypto.txt"),
                String::from("https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts"),
                String::from("https://bitbucket.org/ethanr/dns-blacklists/raw/8575c9f96e5b4a1308f2f12394abd86d0927a4a0/bad_lists/Mandiant_APT1_Report_Appendix_D.txt"),
                String::from("https://phishing.army/download/phishing_army_blocklist_extended.txt"),
                String::from("https://gitlab.com/quidsup/notrack-blocklists/raw/master/notrack-malware.txt"),
                String::from("https://v.firebog.net/hosts/RPiList-Malware.txt"),
                String::from("https://v.firebog.net/hosts/RPiList-Phishing.txt"),
                String::from("https://raw.githubusercontent.com/Spam404/lists/master/main-blacklist.txt"),
                String::from("https://raw.githubusercontent.com/AssoEchap/stalkerware-indicators/master/generated/hosts"),
                String::from("https://urlhaus.abuse.ch/downloads/hostfile/"),
                String::from("https://zerodot1.gitlab.io/CoinBlockerLists/hosts_browser"),
            ],
            update_on_startup: false,
            upstream_dns: String::from("1.1.1.1"),
        }
    }
}
