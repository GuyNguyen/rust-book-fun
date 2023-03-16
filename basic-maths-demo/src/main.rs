use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum GenericVar {
    Float32 { value: f32},
    Float64 { value: f64},
    Int { value: u32}
}

impl Display for GenericVar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Float32 { value } => write!(f, "The value is: {}", value),
            Self::Float64 { value } => write!(f, "The value is: {}", value),
            Self::Int { value } => write!(f, "The value is: {}", value),
        }
    }
}

fn main() {
    let x = GenericVar::Float64 { value: 2.0 };
    println!("{}", x);

    let y = GenericVar::Float32 { value: 3.0 };
    println!("{}", y);

    let sum = GenericVar::Int { value: 5 + 10 };
    println!("{}", sum);
    // output.push(GenericVar::Int(sum));

    let difference = GenericVar::Float64 { value: 95.5 - 4.3 };
    println!("{}", difference);
    // output.push(GenericVar::Float64(difference));

    // let product: i32 = 4 * 30;
    // output.push(GenericVar::Int(product));

    // let quotient: f64 = 56.7 / 32.2;
    // output.push(GenericVar::Float64(quotient));

    // let floored: i32 = 2 / 3;
    // output.push(GenericVar::Int(floored));

    // let remaider: u32 = 43 % 5;
    // output.push(GenericVar::UInt(remaider));

    // println!("The value is: {:?}", output);
}
