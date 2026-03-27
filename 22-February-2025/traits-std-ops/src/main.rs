use std::ops::{Add, Sub};

// check these:
// overflowing_ (sub, add)
// checked_ (sub, add)
// wrapping_ (sub, add)

fn main() {
    let numbers = 1 + 1;

    println!("{}", numbers);

    let khalid = Houses {
        name: "Khalid".to_string(),
        hse_no: 5,
    };

    let linet = Streets {
        name: "Linet".to_string(),
        hse_no: 2,
    };

    let house_nos = khalid.clone() + linet.clone();
    println!("numbers: {}", house_nos);

    // let house_nos = khalid - linet;

    // if let Some(house_) = house_nos {
    //     println!("House numbers: {}", house_);
    // } else {
    //     println!("The server rejected this request",);
    // }
}

#[derive(Debug, Clone)]
pub struct Houses {
    name: String,
    hse_no: u8,
}

impl Add for Houses {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        self.hse_no as u16 + rhs.hse_no as u16
    }
}

impl Sub for Houses {
    type Output = Option<u16>;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.hse_no as u16).checked_sub(rhs.hse_no as u16)
    }
}

#[derive(Debug, Clone)]
pub struct Streets {
    name: String,
    hse_no: u8,
}

impl Add for Streets {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        self.hse_no as u16 + rhs.hse_no as u16
    }
}
