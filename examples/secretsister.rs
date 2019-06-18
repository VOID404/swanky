use clap::{App, Arg};
use itertools::Itertools;
use popsicle::{MultiPartyReceiver, MultiPartySender};
use serde::Deserialize;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead};
use scuttlebutt::Block;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
enum PartyConfig {
    sender { address: String, port: String },
    receiver { address: String, port: String },
}

fn main() {
    let matches = App::new("secretsister")
        .version("1.0")
        .author("Brent Carmer <bcarmer@galois.com>")
        .about("Multi-Party IPV6 Address Private Set Intersection using KMPRT")
        .arg(
            Arg::with_name("CONFIG_FILE")
                .short("c")
                .long("config")
                .help("Yaml config file.")
                .takes_value(true)
                .default_value("secretsister.yaml"),
        )
        .arg(Arg::with_name("PARTY_ID").help("Party id.").required(true))
        .arg(
            Arg::with_name("INPUT_FILE")
                .help("Sets the input file to use.")
                .required(true),
        )
        .setting(clap::AppSettings::ColorAlways)
        .get_matches();

    let config: Vec<PartyConfig> = serde_yaml::from_reader(
        &mut std::fs::File::open(matches.value_of("CONFIG_FILE").unwrap()).unwrap(),
    )
    .unwrap();

    let my_id = matches.value_of("PARTY_ID").unwrap();

    let input_file = std::fs::File::open(matches.value_of("INPUT_FILE").unwrap()).unwrap();
    let inputs = BufReader::new(input_file).lines().map(|s| {
        ipv6_to_block(&s.unwrap())
    }).collect_vec();

    println!("{:?}", config);
}

fn ipv6_to_block(addr: &str) -> Block {
    let mut nums = [0_u8; 16];
    for (i, hex) in addr.split(":").enumerate() {
        let x = u16::from_str_radix(hex, 16).unwrap();
        nums[2*i + 1] = (x & 0xFF) as u8;
        nums[2*i]     = ((x >> 8) & 0xFF) as u8;
    }
    Block::from(nums)
}

fn block_to_ipv6(b: Block) -> String {
    let bs = <[u8; 16]>::from(b).into_iter().map(|byte| {
        format!("{:02X}", byte)
    }).collect_vec();
    bs.chunks(2).map(|pair| pair.concat()).join(":")
}
