use krpc_client::services::space_center::SpaceCenter;

use crate::script::Script;

pub struct Taerobee;
impl Script for Taerobee {
    fn run(&self, client: std::sync::Arc<krpc_client::Client>) {
        print!("Press enter once you're ready to launch...");
        let _ = std::io::stdin().read_line(&mut String::new());

        let center = SpaceCenter::new(client);
        let active_vessel = center.get_active_vessel();
        if active_vessel.is_err() {
            println!("Failed getting active vessel! Aborting...");
        }
        let active_vessel = active_vessel.unwrap();

        let ut_stream = center.get_ut_stream().unwrap();
        ut_stream.set_rate(1.0).unwrap();

        for i in 10..0 {
            ut_stream.wait();
            println!("T-{i}");
        }
    }
}
