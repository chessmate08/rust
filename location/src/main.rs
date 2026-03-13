enum Location {
    Unknown,
    Anonymous, 
    Known(f64, f64)
}
impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Location is not known"),
            Location::Anonymous => println!("Location is Anymous"),
            Location::Known(latitude, longitude) => {
                println!("Location is known: Latidue {:.4} by Longitude {:.4}", latitude, longitude)
            }
        }
    }
}
use std::fmt::Display;
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Location::Unknown => write!(f, "Unknown"),
            Location::Anonymous => write!(f, "Anymous"),
            Location::Known(latitude, longitude) => {
                write!(f, "{:.4}, {:.4}", latitude, longitude)
            }
        }
    }
}
fn main () {
    let address = Location::Known(20.645, -507.794);
    address.display();
    println!("{}", address);
}