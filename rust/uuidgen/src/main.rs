use std::time::{SystemTime, UNIX_EPOCH};

use clap::{arg, command};
use uuid::{NoContext, Timestamp, Uuid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .subcommand(command!("-4"))
        .subcommand(
            command!("-5")
                .arg(arg!(<namespace> "namespace").index(1))
                .arg(arg!(<name> "name").index(2)),
        )
        .subcommand(command!("-7").arg(arg!(<timestamp> "timestamp").index(1).required(false)))
        .get_matches();

    match matches.subcommand() {
        Some(("-4", _)) => {
            println!("{}", generate_uuid_v4());
            Ok(())
        }
        Some(("-5", args)) => {
            let namespace = args
                .get_one::<String>("namespace")
                .expect("Missing namespace argument");
            let name = args
                .get_one::<String>("name")
                .expect("Missing name argument");
            let uuid_namespace = Uuid::parse_str(&namespace)?;

            println!("{}", generate_uuid_v5(uuid_namespace, &name));
            Ok(())
        }
        Some(("-7", args)) => {
            let timestamp_arg = args.get_one::<String>("timestamp");
            let timestamp = match timestamp_arg {
                Some(timestamp_value) => {
                    let timestamp_value = timestamp_value.parse::<u64>()?;
                    Timestamp::from_unix(NoContext, timestamp_value, 0)
                }
                None => {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    Timestamp::from_unix(NoContext, now.as_secs(), now.subsec_nanos())
                }
            };

            println!("{}", generate_uuid_v7(timestamp));

            Ok(())
        }
        Some(_) => unreachable!(),
        None => {
            println!("{}", generate_uuid_v4());
            Ok(())
        }
    }
}

fn generate_uuid_v4() -> Uuid {
    Uuid::new_v4()
}

fn generate_uuid_v5(namespace: Uuid, name: &str) -> Uuid {
    Uuid::new_v5(&namespace, name.as_bytes())
}

fn generate_uuid_v7(timestamp: Timestamp) -> Uuid {
    Uuid::new_v7(timestamp)
}
