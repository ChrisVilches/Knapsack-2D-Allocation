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
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use crossbeam;

fn set_signal_handler(term_now: &Arc<AtomicBool>) -> Result<(), Error> {
  let mut signals: Signals = Signals::new(TERM_SIGNALS)?;
  for sig in TERM_SIGNALS {
    // Terminate with 2 signals. (This would only be notorious if we add a "sleep" after each generation.)
    // Since generations execute fast, there's no delay
    // when pressing CTRL+C, but if this line is removed and a "sleep" is added
    // in the generations loop, and we press CTRL+C several times, it would not exit.
    // Having this line "forces" the termination when executing the second termination.
    // More info: https://docs.rs/signal-hook/0.3.10/signal_hook/#a-complex-signal-handling-with-a-background-thread
    flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
    flag::register(*sig, Arc::clone(&term_now))?;
  }
  for signal in &mut signals {
    eprintln!("\nReceived a signal {:?}", signal);
    match signal {
      term_sig => {
        eprintln!("\nTerminating");
        assert!(TERM_SIGNALS.contains(&term_sig));
        break;
      }
    }
  }

  Ok(())
}

fn main() -> Result<(), Error> {
  let (container, items): (Container, Vec<Item>) = dataset_loader::build_scenario_from_opts()?;
  let mut genetic_algorithm: GeneticAlgorithm = GeneticAlgorithm::new(container, &items);
  let mut stats: Stats = Stats::new(&items);
  let term_now = Arc::new(AtomicBool::new(false));

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
    });

    scope.spawn(|_| {
      set_signal_handler(&term_now).unwrap();
    });
  }).expect("threads did not complete successfully");

  Ok(())
}
