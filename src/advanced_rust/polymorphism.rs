trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

struct Sword {
    pub name: String,
    pub damage: u16,
    pub swing_time_ms: u16,
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        (self.damage * 1000_u16) / self.swing_time_ms * 10_u16
    }

    fn description(&self) -> String {
        format!(
            "{}, damage: {}, swing time: {}ms",
            self.name, self.damage, self.swing_time_ms
        )
    }
}
struct Shield {
    pub name: String,
    pub armor: u16,
    pub block: u16,
}

impl Sellable for Shield {
    fn price(&self) -> u16 {
        self.armor + self.block
    }

    fn description(&self) -> String {
        format!(
            "{}, armor: {}, block: {}",
            self.name, self.armor, self.block
        )
    }
}

// Static dispatch or compile time polymorphism
fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("I offer you: {} [{}g]", item.description(), item.price())
}

fn vendor_text_dyanmic(item: &dyn Sellable) -> String {
    format!("I offer you: {} [{}g]", item.description(), item.price())
}

pub fn main() {
    let sword = Sword {
        name: "Sword of Cowardice".into(),
        damage: 10,
        swing_time_ms: 1500,
    };

    let shield = Shield {
        name: "Golden Barrier".into(),
        armor: 50,
        block: 35,
    };

    println!("{}", vendor_text_static(&sword));
    println!("{}", vendor_text_static(&shield));

    let sellables: Vec<&dyn Sellable> = vec![&sword, &shield];

    for s in sellables {
        println!("{}", vendor_text_dyanmic(s))
    }

    let owned_sellable: Vec<Box<dyn Sellable>> = vec![
        Box::new(Sword {
            name: "Blade on Heap".into(),
            damage: 55,
            swing_time_ms: 1400,
        }),
        Box::new(Shield {
            name: "Shield of Dynamic memory".into(),
            armor: 130,
            block: 55,
        }),
    ];

    for s in &owned_sellable{
         println!("{}", vendor_text_dyanmic(s.as_ref()))
    }
}
