use crate::{lists::Blacklist, config::Config};

use std::{
    error::Error,
    net::{SocketAddr, UdpSocket, IpAddr},
    str::FromStr,
};

use domain::{
    self,
    base::{
        iana::{Class, Rcode},
        Message, MessageBuilder, Record, ToDname, Ttl
    },
    rdata::A,
};

/// Struct to hold the recv_addr query id and
#[derive(Debug)]
struct Request {
    addr: SocketAddr,
    id: u16,
}

/// Takes a DNS request and builds an answer that returns the address 0.0.0.0
fn build_answer(message: &Message<Vec<u8>>) -> Vec<u8> {
    // Create DNS message builder and turn it into an answer builder for the DNS request
    let mut builder = MessageBuilder::new_vec()
        .start_answer(message, Rcode::NoError)
        .unwrap();

    // For each question in message
    for question in message.question() {
        // Get domain name
        let dname = question.unwrap().qname().to_dname::<Vec<_>>().unwrap();

        // Create record for answer with address 0.0.0.0
        let record = Record::new(
            dname,
            Class::In,
            Ttl::from_secs(86400),
            A::from_octets(0, 0, 0, 0),
        );

        // Add record to answer builder
        builder.push(record).unwrap();
    }

    // Convert answer builder to vector of bytes and return
    builder.finish()
}

/// Takes a reference to a [`Blacklist`] and starts server loop
pub fn server(list: &Blacklist, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Starting DNS server...");

    // Buffer size of DNS message buffer
    let buffer_size = 1024;

    // Address of UDP socket
    let socket_addr = "0.0.0.0:53";

    // Create UDP socket
    let socket: UdpSocket = UdpSocket::bind(socket_addr)?;

    // Address of upstream DNS server
    let dns_addr = IpAddr::from_str(config.upstream_dns.as_str())?;

    // Upstream DNS server as SocketAddr
    let dns_sock_addr = SocketAddr::new(dns_addr, 53);

    // DNS request que
    let mut requests: Vec<Request> = vec![];

    loop {
        // Reset buffer
        let mut buf = vec![0; buffer_size];

        // Get DNS request from devices
        let (size, recv_addr) = socket.recv_from(&mut buf)?;

        // Remove trailing zeros from buffer
        buf.truncate(size);

        // Construct DNS message from buffer
        let message = Message::from_octets(buf.clone())?;

        // If messages comes from upstream DNS server (which means that the message is a response)
        if recv_addr == dns_sock_addr {
            // Finds DNS request and its index in the request que that has a matching transaction id (because we need its address so we know where to send the response and its index to remove it from the que)
            let request = requests
                .iter()
                .enumerate()
                .find(|(_index, req)| req.id == message.header().id());

            // Check if the request and its index were found. If not continue...
            let (index, request) = match request {
                Some(r) => r,
                None => continue,
            };

            // Send response from upstream DNS server to requesting device
            socket.send_to(&buf, request.addr)?;

            // Remove request from the requests que
            requests.swap_remove(index);
        } else {
            // Get questions from DNS request
            let questions: Vec<_> = message.question().collect();

            // Whether blacklist contains domains from any of the questions
            let filter = questions.iter().any(|&q| {
                // Get domain name from question
                let domain = q.unwrap().qname().to_string();

                // Check if blacklist contains domain name
                list.list.contains(&domain)
            });

            if filter {
                // Builds an answer with address 0.0.0.0
                let answer = build_answer(&message);

                // Sends answer to requesting device
                socket.send_to(&answer, recv_addr)?;

                println!(
                    "{} filtered domain: {}",
                    recv_addr.ip(),
                    message.first_question().unwrap().qname().to_string(),
                );

                continue;
            } else {
                println!(
                    "{} requested domain: {}",
                    recv_addr.ip(),
                    message.first_question().unwrap().qname().to_string(),
                );
            }

            // Add request to requests que
            requests.push(Request {
                addr: recv_addr,
                id: message.header().id(),
            });

            // Send request to upstream DNS server
            socket.send_to(&buf, dns_sock_addr)?;
        }
    }
}
