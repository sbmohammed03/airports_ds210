use std::error::Error;
use csv::Reader;
use std::collections::HashMap;
use libm::sin;
use libm::cos;
use libm::sqrt;
use libm::atan2;

/* define an airport vertex
    name: string
    airport type: string
    latitude: float
    longitude: float
    iso country: string
    iso region: string
    municipality: string
*/
pub struct Flights {
    airports: HashMap<i32, Airport>,
    connections: Vec<(i32, i32)>,
    name_idmap: HashMap<String, i32>,
}

impl Flights {
    pub fn new() -> Flights {
        Flights { airports: HashMap::new(), connections: Vec::new(), name_idmap: HashMap::new() }
    }

    pub fn create_graph(&mut self) {
        let _ = self.read_routes();
        let _ = self.read_airports();
    }

    pub fn read_airports(&mut self) -> Result<(), Box<dyn Error>> {
        let data_airports = "datasets/airports.csv";

        // Open the file
        let mut rdr = Reader::from_path(data_airports)?;

        // Iterate over records in the CSV file
        for res in rdr.records() {
            let record = res?;
            // println!("{:?}", record);

            let id: i32 = record[0].parse()?;
            let name = record[1].to_string();
            let latitude: f64 = record[6].parse()?;
            let longitude: f64 = record[7].parse()?;
            let city = record[2].to_string();
            let country = record[3].to_string();
            
            let mut airport = Airport 
                {
                    id,
                    name,
                    latitude,
                    longitude,
                    city,
                    country,
                    connections: Vec::new()
                };

            for pair in &self.connections {
                let src = pair.0;
                let dst = pair.1;

                if src == id {
                    airport.connections.push(dst);
                }

                if dst == id {
                    airport.connections.push(src);
                }
            }

            // println!("{} airport has {} connections", record[1].to_string(), airport.connections.len());

            // airport.display();
            if let Some(_) = self.airports.insert(id, airport) {
                println!("Duplicate id {}: {}", id, record[1].to_string());
            } 

            if let Some(_) = self.name_idmap.insert(record[5].to_string(), id) {
                println!("Duplicate id {}: {}", id, record[5].to_string());
            } 
        }

        println!("there are {} airports", self.airports.len());
        Ok(())
    } 

    pub fn read_routes(&mut self) -> Result<(), Box<dyn Error>> {
        let data_routes = "datasets/routes.csv";

        // Open the file
        let mut rdr = Reader::from_path(data_routes)?;
        // Iterate over records in the CSV file
        for res in rdr.records() {
            let record = res?;
            // println!("{:?}", record);
            
            if record[3].to_string() != "NA" && record[5].to_string() != "NA" {
                let airport_src: i32 = record[3].to_string().parse()?;
                let airport_dest: i32 = record[5].to_string().parse()?;

                self.connections.push((airport_src, airport_dest));
                // if self.airports.contains_key(&airport_id) {
                //     println!("{:?}", airport_id);
                // }

            }
        }
        Ok(())
    }

    pub fn distance(&mut self, src:i32, dst:i32) -> f64 {
        const EARTH_RADIUS: f64 = 6371.0; // Earth radius in kilometers

        let mut delta_latitude:f64;
        let mut delta_longitude:f64;
        let src_latitude:f64;
        let dst_latitude:f64;

        if let Some(dst_airport) = self.airports.get(&dst) {
            // Access the fields of the Person object
            // println!("{}: {}, {}", dst_airport.name, dst_airport.city, dst_airport.country);
            dst_latitude = dst_airport.latitude;
            delta_latitude = dst_airport.latitude;
            delta_longitude = dst_airport.longitude;
        } else {
            return -1.000;
        }

        if let Some(src_airport) = self.airports.get(&src) {
            // Access the fields of the Person object
            // println!("{}: {}, {}", src_airport.name, src_airport.city, src_airport.country);
            src_latitude = src_airport.latitude;
            delta_latitude = delta_latitude - src_airport.latitude;
            delta_longitude = delta_longitude - src_airport.longitude;
        } else {
            return -1.000;
        }
        
        let delta_latitude = delta_latitude.to_radians();
        let delta_longitude = delta_longitude.to_radians();

        // compute the haversine distance of two longitude distances
        let a:f64 = sin(delta_latitude / 2.0) * sin(delta_latitude / 2.0) +
                    cos(src_latitude.to_radians()) * cos(dst_latitude.to_radians()) *
                    sin(delta_longitude / 2.0) * sin(delta_longitude / 2.0);
            
        let c:f64 = 2.0 * atan2(sqrt(a), sqrt(1.0-a));

        return EARTH_RADIUS * c;
    }

    pub fn breadth_first_search(&mut self, src:i32, dst:i32) -> Vec<(i32,i32)> {
        // create a queue of airport ID's
        let mut queue:Vec<i32> = Vec::new();

        // create a visited hash
        let mut parent:HashMap<i32, i32> = HashMap::new();
        let mut visited:Vec<i32> = Vec::new();
        let mut route:Vec<(i32,i32)> = Vec::new();

        if let Some(_) = self.airports.get(&src) {
            queue.push(src);
            parent.insert(src, -1);
        }
        
        while queue.len() > 0 {
            let node:i32 = queue.remove(0);
            visited.push(node);

            if let Some(airport) = self.airports.get(&node) {
                for conn in &airport.connections {
                    if !visited.contains(conn) {
                        parent.insert(*conn, node);
                        queue.push(*conn);
                        if *conn == dst {
                            let mut t = *conn;
                            while t != -1 {
                                if let Some(value) = parent.get(&t) {
                                    if *value != -1 {
                                        route.push((t,*value));
                                        println!("ID:{} -> ID:{}", t, *value);
                                    }
                                    t = *value;
                                }
                            }
                            return route;
                        }
                    }
                }
            }    
        }
        return route;
    }
    
    pub fn get_flight_id(&mut self, src:i32, dst:i32) {
        let mut src_name:String = "".to_string();
        let mut dst_name:String = "".to_string(); 

        if let Some(value) = self.airports.get(&src) {
            src_name = value.name.to_string();
        } else {
            println!("No airport source avaliable");
            return;
        }

        if let Some(value) = self.airports.get(&dst) {
            dst_name = value.name.to_string();
        } else {
            println!("No airport destination avaliable");
            return;
        }

        if (src == dst) {
            println!("Cannot fly to same destination!");
            return;
        }

        let route:Vec<(i32,i32)> = self.breadth_first_search(src, dst);
        let mut distance:f64 = 0.0;
        for pair in &route {
            distance += self.distance(pair.0, pair.1);
        }

        if distance == 0.0 {
            println!("No flight from {} to {} avaliable", src_name, dst_name);
        } else {
            println!("Flight from {} to {} is {:.3} km", src_name, dst_name, distance);
        }
    }

    // using IACO identifiers to index into hashmap nameIDmap
    // calls get_flight_id to perform the actions needed to get the flight info
    pub fn get_flight(&mut self, src:&str, dst:&str) {
        println!("--------------------------");

        let mut src_id:i32 = -1;
        let mut dst_id:i32 = -1; 
        
        let src_s = String::from(src);
        let dst_s = String::from(dst);

        if let Some(value) = self.name_idmap.get(&src_s) {
            src_id = *value;
        }

        if let Some(value) = self.name_idmap.get(&dst_s) {
            dst_id = *value;
        }

        // print!("{} {}",src_id,dst_id);
        self.get_flight_id(src_id, dst_id);
        println!("--------------------------\n");
    }

}

pub struct Airport {
    id: i32,
    name: String,
    latitude: f64,
    longitude: f64,
    city: String,
    country: String,
    connections: Vec<i32>,
}

impl Airport {
    pub fn dummy() -> Airport {

        let id:i32 = 3448;
        let name = String::from("Logan International Airport");
        let latitude: f64 = 42.36429977;
        let longitude: f64 = -71.00520325;
        let city = String::from("Boston");
        let country = String::from("US");

        Airport {
            id,
            name,
            latitude,
            longitude,
            city,
            country,
            connections: Vec::new()
        }
    }

    pub fn display(&self) {
        println!("\n[{} airport id {}]", self.name, self.id);
        println!(" {}, {}:", self.city, self.country);
        println!(" coordinates({}, {}):", self.longitude, self.latitude);
    }
}