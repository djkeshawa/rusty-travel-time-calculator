
use std::f32;

fn main() {
    let cor1  = Coordinates{ lat: 40.7128, lon: -74.0060 };
    let cor2 = Coordinates{ lat:40.7128, lon:-74.0070 };
    println!("haversine distance {:?}", haversine_distance(&cor1, &cor2));
}


struct Coordinates {
    lat: f32,
    lon: f32
}

// struct Stop {
//     coordinates:Coordinates,
//     stop_number:i32,
//     expected_arrival_time: String,
//     expected_departure: String,
//     actual_arrival:String,
//     actual_departure:String,
//     wait_time: i32,
//     skipped:bool
// }

fn haversine_distance(coordinates1: &Coordinates, coordinates2: &Coordinates) -> f32{
    const R: f32 = 6371.0;

    let lat1_rad = coordinates1.lat.to_radians();
    let lon1_rad = coordinates1.lon.to_radians();
    let lat2_rad =coordinates2.lat.to_radians();
    let lon2_rad =coordinates2.lon.to_radians();

    let d_lat =  lat2_rad -lat1_rad;
    let d_lon = lon2_rad -lon1_rad;

    let a = f32::sin(d_lat/2.0).powi(2) +
        lat1_rad.cos() * lat2_rad.cos() * f32::sin(d_lon/2.0).powi(2);
    let c = 2.0 * f32::atan2(a.sqrt(),(1.0-a).sqrt());

     R * c
}