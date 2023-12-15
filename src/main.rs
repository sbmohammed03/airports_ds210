use std::io::{self, Write};
mod travel;
use travel::Flights;

pub fn test (a: &mut Flights) {

    // ran tests from source to destintion with edge cases
    // edge cases: same src same dst, wrong IACO
    a.get_flight( "KBOS", "OTHH");
    a.get_flight( "OTHH", "KBOS");
    a.get_flight( "KBOS", "KBOS");
    a.get_flight( "VABB", "OTHH");
    a.get_flight( "KSFO", "VABB");
    a.get_flight( "fsgfdsf", "kjadkjakd");
}

fn main () {

    // let airport = Airport::dummy();
    // airport.display();
    let mut airports: Flights = Flights::new();
    airports.create_graph();

    // let LOG_DOH:f64 = airports.distance(3448, 11051);

    // println!("Logan to Doha: {:.2} km\n", LOG_DOH);

    // airports.get_flight("KBOS", "OTHH");

    loop {
        let mut input:[String; 2] = ["".to_string(), "".to_string()];
        io::stdout().flush().expect("Failed to flush stdout");
        println!("\nEnter IACO Airport Source");
        io::stdin().read_line(&mut input[0]).expect("Failed to read line");
        
        if (input[0] == "TEST") {
            println!("RUNNING TESTS"); 
            test(&mut airports); 
            return;
        }
        
        io::stdout().flush().expect("Failed to flush stdout");
        println!("\nEnter IACO Airport Destination");
        io::stdin().read_line(&mut input[1]).expect("Failed to read line");

        println!("You are flying from {} to {}", input[0].trim(), input[1].trim());
        airports.get_flight( &input[0].trim(), &input[1].trim());
    }
    // airports.get_flight_id( 11051, 3448);
}