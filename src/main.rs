use std::f64::consts::PI;
use std::io;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        (PI * self.radius).powi(2)
    }
    fn perimeter(&self) -> f64 {
        2.0 * (PI * self.radius)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}

// Hauptprogramm
fn main() {
    loop {
        // Menü anzeigen
        println!("Wählen Sie eine Form:");
        println!("1. Circle");
        println!("2. Rectangle");
        println!("3. Exit");

        // Benutzerwahl einlesen
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Fehler beim Lesen der Eingabe");
        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) => {
                // Kreis erstellen
                println!("Geben Sie den Radius des Kreises ein:");
                let radius = read_input();
                if radius <= 0.0 {
                    println!("Fehler: Der Radius muss größer als 0 sein.\n");
                    continue;
                }

                let circle = Circle { radius };
                println!("Sie haben einen Kreis mit Radius {:.2} gewählt.", circle.radius);
                println!("Fläche: {:.2}", circle.area());
                println!("Umfang: {:.2}\n", circle.perimeter());
            }
            Ok(2) => {
                // Rechteck erstellen
                println!("Geben Sie die Breite des Rechtecks ein:");
                let width = read_input();
                println!("Geben Sie die Höhe des Rechtecks ein:");
                let height = read_input();

                if width <= 0.0 || height <= 0.0 {
                    println!("Fehler: Breite und Höhe müssen größer als 0 sein.\n");
                    continue;
                }

                let rectangle = Rectangle { width, height };
                println!(
                    "Sie haben ein Rechteck mit Breite {:.2} und Höhe {:.2} gewählt.",
                    rectangle.width, rectangle.height
                );
                println!("Fläche: {:.2}", rectangle.area());
                println!("Umfang: {:.2}\n", rectangle.perimeter());
            }
            Ok(3) => {
                println!("Programm beendet.");
                break;
            }
            _ => {
                println!("Ungültige Eingabe. Bitte wählen Sie 1, 2 oder 3.\n");
            }
        }
    }
}



// Hilfsfunktion zum Einlesen von Gleitkommazahlen
fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fehler beim Lesen der Eingabe");
    input.trim().parse::<f64>().unwrap_or(-1.0)
}

