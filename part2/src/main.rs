struct City {
    description: String,
    residents: u64,
    // 💡 HINT: this will cause other compiler errors.
    //    They will tell you what other changes need to happen!
    is_coastal: bool
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal: true
        }
    } else {
        City {
            description: format!("a *non-coastal* city of approximately {} residents", residents),
            residents,
            is_coastal: false
        }
    }
}

fn main() {
    let rustville: City = new_city(1000, false);

    println!("This city can be described as:");
    println!("{}", rustville.description);
}
