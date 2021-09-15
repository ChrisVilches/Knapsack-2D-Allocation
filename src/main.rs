extern crate lazy_static;
extern crate clap;
extern crate text_io;

mod types;
mod math;
mod genetic_algorithm;
mod util;
mod image_writer;
mod dataset_loader;

use genetic_algorithm::GeneticAlgorithm;
use types::item::Item;
use types::container::Container;
use types::stats::Stats;
use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use signal_hook::flag;
use signal_hook::consts::signal::*;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use crossbeam;

fn main() -> Result<(), Error> {
  let (container, items): (Container, Vec<Item>) = dataset_loader::build_scenario_from_opts()?;
  let mut genetic_algorithm: GeneticAlgorithm = GeneticAlgorithm::new(container, &items);
  let mut stats: Stats = Stats::new(&items);
  let term_now = Arc::new(AtomicBool::new(false));
  let mut signals: Signals = Signals::new(TERM_SIGNALS)?;

  for sig in TERM_SIGNALS {
    // Terminate with 2 signals.
    flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
    flag::register(*sig, Arc::clone(&term_now))?;
  }

  println!("Items: {}", items.len());
  println!("Max score assuming infinite container: {}", stats.max_possible_score);

  crossbeam::scope(|scope| {
    scope.spawn(|_| {
      while !term_now.load(Ordering::Relaxed){
        let global_optimum_found: bool = genetic_algorithm.execute_population(&mut stats);
        if global_optimum_found {
          println!("Global optimum found.");
          break;
        }
      }

      stats.print();
      image_writer::create_image("output.png".to_string(), &container, &items, &stats.optimal_solution);
      std::process::exit(0);
    });

    scope.spawn(|_| {
      //'outer: loop {
        //println!("INFINITE LOOP");
        for signal in signals.pending() {
          match signal {
            SIGINT => {
              println!("\n(SIGINT) Stopping...");
              break;// 'outer;
            },
            SIGTERM => {
              println!("\n(SIGTERM) Stopping...");
              break;// 'outer;
            },
            term_sig => {
              println!("\nSignal: {:?}", term_sig);
              break;// 'outer;
            }
          }
        //}
      }
    });
  }).expect("threads did not complete successfully");

  Ok(())
}
