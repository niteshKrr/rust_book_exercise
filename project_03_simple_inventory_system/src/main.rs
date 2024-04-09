struct Item {
    name: String,
    quantity: usize,
    price: usize,
}

impl Item {
    fn new(name: &str , quantity: usize , price: usize) -> Item {
        Item {
            name: name.to_string(),
            quantity,
            price,
        }
    }
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {

    fn new() -> Inventory {
        Inventory {
            items: Vec::new(),
        }
    }

    fn add_item(&mut self , name: &str , quantity: usize , price: usize) {
        let item = Item::new(name, quantity, price);
        self.items.push(item);
    }

    fn remove_item(&mut self , index: usize) {
        if index <= self.items.len() {
            self.items.remove(index);
        }
        else {
            println!("please enter a valid index");
        }
    }

    fn update_quantity(&mut self , quantity: usize , index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            item.quantity = quantity;
        }
        else {
            println!("please enter a valid index to remove item from inventory");
        }
    }

    fn display_items(&self) {
        for item in self.items.iter() {
            print!("{} ",item.name);
            print!("{} ",item.quantity);
            print!("{} ",item.price);
            println!();
        }
    }
}

fn my_seperator() {
    let sep = String::from("-").repeat(50);
    println!("{}",sep);
}

fn main() {

    let mut my_inventory = Inventory::new();

    my_inventory.add_item("Book", 50, 100);
    my_inventory.add_item("Pen", 10, 10);
    my_inventory.add_item("Study_table", 40, 500);
    my_inventory.add_item("Laptop", 80, 80000);

    my_inventory.display_items();

    my_seperator();

    my_inventory.remove_item(2);
    my_inventory.display_items();

    my_seperator();

    my_inventory.update_quantity(200, 1);
    my_inventory.display_items();
    
}