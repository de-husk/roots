use dirs;
use roots::Root;
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::time::SystemTime;
use std::{env, fs};

fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(cmd) = args.get(1) {
    match cmd.as_str() {
      "plant" => plant(args.get(2)),
      "water" => water(),
      "view" => view(),
      "help" => help(),
      _ => {
        println!("Error: Unknown command {}. See `roots help`", cmd);
        process::exit(1);
      }
    }
  } else {
    println!("Error: Missing command! See: `roots help`");
    process::exit(1);
  }
}

fn plant(name: Option<&String>) {
  let name = if let Some(n) = name {
    n.to_string()
  } else {
    String::from("Max")
  };

  let seed = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("Error getting system time")
    .as_secs();

  let r = Root::new(name, seed);

  // write root file, overwriting for now:
  let mut root_path = dirs::home_dir().expect("Error $HOME not set");
  root_path.push(".roots");

  if !root_path.as_path().exists() {
    fs::create_dir(root_path.as_path()).expect("Error creating roots dir");
  }

  root_path.push("root_0");
  let file = File::create(&root_path).expect("Error creating root file");
  serde_json::to_writer(file, &r).expect("Error writing root file");

  println!(
    "New root planted at {}",
    root_path.as_path().display().to_string()
  );

  println!("Be patient and watch it grow.");
}

fn view() {
  let mut path = dirs::home_dir().expect("Error $HOME not set");
  path.push(".roots");
  path.push("root_0");

  let file = File::open(path).expect("Error reading root file");
  let reader = BufReader::new(file);

  let mut r: Root = serde_json::from_reader(reader).unwrap();
  r.generate();
}

fn help() {}

fn water() {}
