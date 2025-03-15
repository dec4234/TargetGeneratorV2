use log::LevelFilter;
use simple_logger::SimpleLogger;
use targetgen_lib::generator;
use targetgen_lib::generator::TargetGenerator;

fn main() {
    
}

#[ignore]
#[test]
fn create_live_test() {
	SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();

	generator::util::cleanup_output("output").unwrap();

	let mut tg = TargetGenerator::new("backgrounds", "objects", "output/annotations.json").unwrap();
	tg.config.permit_collisions = false;
	tg.config.do_random_rotation = true;
	tg.generate_targets(1, ..6, "output").unwrap();

	tg.close();
}