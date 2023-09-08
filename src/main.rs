use std::*;

const DATABASE: &str = "/var/lib/tracker/tracker.json";

struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

impl Color {
    fn new() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }
    }

    fn joke() {
        println!("{}", "king of the castle");
    }

    fn normy(&self) {
        Self::joke();
    }

    fn yell(&self) -> Vec<f64> {
        vec![self.red, self.green, self.blue, self.alpha]
    }
}

#[derive(Debug)]
struct SiEvent<'a> {
    title: &'a str,
    dur: f64,
}

impl SiEvent<'_> {
    fn new() -> Self {
        Self {
            title: "asdfa",
            dur: 42.0,
        }
    }
}

impl fmt::Display for SiEvent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {})", self.title, self.dur);
    }
}

fn main() {
    if false {
        println!("Hello, world!");
        haus();

        let haus = Haus {
            tuer: 3.19,
            fenster: 14.304,
        };

        println!(
            "fenster {:.2}\ttuer {:.2}",
            haus.get_fenser(),
            haus.get_tuer()
        );

        haus.open_tuer_and_door();

        let color = Color::new();

        println!("{}", color.red);
        println!("{}", color.yell()[0]);
        Color::joke();
        color.normy();
        for i in 0..=10 {
            println!("{}", i);
        }
        println!("{}", SiEvent::new());
    }

    let sievent = SiEvent::new();
    assert_eq!(format!("{sievent}"), "(asdfa, 42)");

    println!("{:?}", sievent);
}

fn haus() {
    println!("{}", DATABASE);
}

impl Haus {
    fn get_tuer(&self) -> f32 {
        self.tuer
    }

    fn get_fenser(&self) -> f32 {
        self.fenster
    }

    fn open_tuer_and_door(&self) {
        println!("{}", "Öffne Tür und TOOOOR!");
    }
}

struct Haus {
    tuer: f32,
    fenster: f32,
}
