use clap::Parser;
use gethostname::gethostname;
use rand::distributions::{Alphanumeric, DistString};
use rumqttc::{MqttOptions, Client, QoS};
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(author, about, version)]
struct Args {
    /// The file containing the temperature. Usually in /sys/class/thermal.
    #[arg(short, long)]
    file: String,

    /// The address of the MQTT broker to connect to.
    #[arg(short, long)]
    address: String,

    /// The port of the MQTT broker to connect to.
    #[arg(long, default_value = "1883")]
    port: u16,

    /// The username to use when connecting to the MQTT broker.
    #[arg(short, long)]
    username: String,

    /// The password to use when connecting to the MQTT broker.
    #[arg(short, long)]
    password: String,

    /// The topic to publish the temperature to.
    #[arg(short, long)]
    topic: String,

    /// Whether to log the temperature value being published.
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    /// Number of seconds to wait in between publishing temperature values.
    #[arg(short, long, default_value = "5")]
    interval: u64,

    /// Whether the broker should retain the last message sent.
    #[arg(short, long, default_value = "false")]
    retain: bool,
}

fn read_to_string(path: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_and_normalize(temperature: &String) -> Result<f32, std::io::Error> {
    let mut parsed: f32 = temperature.trim().parse()
        .expect("Failed to parse temperature");
    if parsed > 1000.0 {
        parsed /= 1000.0;
    }
    Ok(parsed)
}

fn format_to_string(temperature: f32) -> String {
    format!("{:.2}", temperature)
}

fn get_temperature(file_path: &String, verbose: bool) -> Result<String, std::io::Error> {
    let raw_temp = read_to_string(&file_path)
        .expect("Failed to read file");
    let normalized = parse_and_normalize(&raw_temp)
        .expect("Failed to parse temperature");
    let formatted = format_to_string(normalized);

    if verbose {
        print!("Temperature: {}\n", formatted);
    }

    Ok(formatted)
}

fn generate_client_id() -> String {
    let hostname = gethostname().into_string().unwrap();
    let suffix = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);
    format!("sysmon_{}_{}", hostname, suffix)
}

fn main() {
    let args = Args::parse();
    let file_path = args.file;
    let host = args.address;
    let port = args.port;
    let username = args.username;
    let password = args.password;
    let topic = args.topic;
    let verbose = args.verbose;
    let interval = args.interval;
    let retain = args.retain;

    let client_id = generate_client_id();
    print!("Connecting to {}:{} as client ID {}\n", host, port, client_id);
    let mut mqtt_options = MqttOptions::new(
        client_id,
        host,
        port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(interval));
    mqtt_options.set_credentials(username, password);

    let (client, mut connection) = Client::new(mqtt_options, 10);
    thread::spawn(move || loop {
        let temperature = get_temperature(&file_path, verbose)
            .expect("Failed to get temperature");

        client.publish(
            topic.clone(),
            QoS::AtLeastOnce,
            retain,
            temperature
        ).unwrap();
        thread::sleep(Duration::from_secs(interval));
    });

    // This comes straight out of Copilot...
    // But the only thing we're interested in is that the event loop is running,
    // which only happens when we iterate over the connection.
    // We don't really care about incoming payloads as this program is only
    // publishing temperature values.
    for (_, notification) in connection.iter().enumerate() {
        match notification {
            Ok(notification) => {
                match notification {
                    rumqttc::Event::Incoming(rumqttc::Packet::Publish(packet)) => {
                        let payload = std::str::from_utf8(&packet.payload).unwrap();
                        print!("Received: {}\n", payload);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                print!("Error: {}\n", e);
                break;
            }
        }
    }
}
