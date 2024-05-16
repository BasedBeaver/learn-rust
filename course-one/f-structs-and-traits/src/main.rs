// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    grapes_left: u32,
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        // Eat one grape.
        self.grapes_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
    item.bite();
    item.bite();
    item.bite();
}

fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    let mut grapes = Grapes { grapes_left: 100 };

    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}
