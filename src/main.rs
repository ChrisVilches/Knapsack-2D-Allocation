#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate text_io;

mod types;
mod math;
mod genetic_algorithm;
mod util;

use genetic_algorithm::GeneticAlgorithm;
use types::item::Item;
use types::container::Container;
use types::stats::Stats;

use clap::{Arg, App, AppSettings, SubCommand};
use text_io::read;
use std::io::Read;
use std::sync::Mutex;

fn random_scenario(container_square_size: i64, item_count: i64, item_max_side: i64, item_max_benefit: i64) -> (Container, Vec<Item>) {
  util::ensure_positive(container_square_size);
  util::ensure_positive(item_count);
  util::ensure_positive(item_max_side);
  util::ensure_positive(item_max_benefit);

  let container: Container = Container::new(container_square_size, container_square_size);
  let mut items: Vec::<Item> = Vec::<Item>::new();

  for _ in 0..item_count {
    items.push(Item::make_random(item_max_side, item_max_side, item_max_benefit));
  }

  (container, items)
}

fn file_scenario(filename: String) -> (Container, Vec<Item>) {
  let mut file = std::fs::File::open(filename).unwrap().bytes().map(|ch| ch.unwrap());
  let container_width: i64 = read!("{}", file);
  let container_height: i64 = read!("{}", file);
  let item_count: i64 = read!("{}", file);

  let container = Container::new(container_width, container_height);

  let mut items: Vec<Item> = Vec::<Item>::new();

  println!("------ Data from file ------");
  println!("{:?}", container);

  for _ in 0..item_count {
    let w: i64 = read!("{}", file);
    let h: i64 = read!("{}", file);
    let b: i64 = read!("{}", file);
    let item = Item::new(w, h, b);
    println!("{:?}", &item);
    items.push(item);
  }

  // TODO: How to close file?
  println!("------ File read finished ------");
  (container, items)
}

static PROGRAM_DESCRIPTION: &str = "Genetic algorithm for knapsack 2D rectangle allocation.";

fn file_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("file") 
    .arg(Arg::with_name("file input")
    .long("file-input")
    .value_name("FILE")
    .takes_value(true)
    .required(true))
}

fn random_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("random") 
    .arg(Arg::with_name("item count")
      .long("item-count")
      .value_name("ITEM_COUNT")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("container square side")
      .long("container-square-side")
      .value_name("CONTAINER_SQUARE_SIDE")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("item max square side")
      .long("item-max-square-side")
      .value_name("ITEM_MAX_SQUARE_SIDE")
      .takes_value(true).required(true))
    .arg(Arg::with_name("max benefit")
      .long("max-benefit")
      .value_name("MAX_BENEFIT")
      .takes_value(true)
      .required(true))
}

fn build_scenario_from_opts() -> (Container, Vec<Item>) {
  let matches = App::new(PROGRAM_DESCRIPTION).setting(AppSettings::SubcommandRequiredElseHelp)
                                             .subcommand(file_subcommand())
                                             .subcommand(random_subcommand())
                                             .get_matches();

  match matches.subcommand() {
    ("random", Some(matches)) => {
      random_scenario(
        util::parse(matches.value_of("container square side")),
        util::parse(matches.value_of("item count")),
        util::parse(matches.value_of("item max square side")),
        util::parse(matches.value_of("max benefit"))
      )
    },
    ("file", Some(matches)) => {
      let file_name = match matches.value_of("file input") {
        Some(string) => string,
        None => { panic!() }
      };
      file_scenario(file_name.to_string())
    },
    _ => {
      panic!();
    }
  }
}

lazy_static! {
  static ref STATS: Mutex<Stats> = Mutex::new(Stats::new());
}

fn main() {
  let (container, items): (Container, Vec<Item>) = build_scenario_from_opts();
  let mut genetic_algorithm: GeneticAlgorithm = GeneticAlgorithm::new(container, &items);
  STATS.lock().unwrap().initialize(&items);

  println!("Items: {}", items.len());
  println!("Max score assuming infinite container: {}", STATS.lock().unwrap().max_possible_score);

  ctrlc::set_handler(move || {
    println!("");
    println!("Stopping...");
    println!("");
    let stats = STATS.lock().unwrap();
    stats.print();
    std::process::exit(0);
  })
  .expect("Error setting Ctrl-C handler");

  loop {
    let mut stats = STATS.lock().unwrap();
    genetic_algorithm.execute_population(&mut stats);
  }
}
