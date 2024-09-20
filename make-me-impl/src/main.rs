use std::fmt;

struct Vechicle {
    vechicle_type: VechicleType,
    year: i16,
    make: String,
}

enum VechicleType {
    Car,
    Truck,
    Van,
    Motorcycle,
}

impl fmt::Display for VechicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vechicle_type_str: &str = match self {
            VechicleType::Car => "Car",
            VechicleType::Truck => "Truck",
            VechicleType::Van => "Van",
            VechicleType::Motorcycle => "Motorcycle",
        };

        write!(f, "{}", vechicle_type_str)
    }
}

impl fmt::Display for Vechicle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.vechicle_type, self.year, self.make)
    }
}

fn main() {
    let vechicle1: Vechicle = Vechicle {
        vechicle_type: VechicleType::Car,
        year: 2019,
        make: String::from("Toyota"),
    };

    let vechicle2: Vechicle = Vechicle {
        vechicle_type: VechicleType::Truck,
        year: 2018,
        make: String::from("Ford"),
    };

    let vechicle3: Vechicle = Vechicle {
        vechicle_type: VechicleType::Van,
        year: 2017,
        make: String::from("Chevrolet"),
    };

    let vechicle4: Vechicle = Vechicle {
        vechicle_type: VechicleType::Motorcycle,
        year: 2016,
        make: String::from("Honda"),
    };

    println!("{}", vechicle1);
    println!("{}", vechicle2);
    println!("{}", vechicle3);
    println!("{}", vechicle4);
}
