use key_val_tree::KeyValMap;

fn main() {
    let mut key_val = KeyValMap::<String, String>::new();

    println!("{:?}", key_val.insert("Foo".to_string(), "Bar".to_string()));
    println!("{:?}", key_val.insert("Foo".to_string(), "Bar".to_string()));
    println!("{:?}", key_val.insert("Baz".to_string(), "Bar".to_string()));
    println!("{:?}", key_val.insert("Foo".to_string(), "Baz".to_string()));
    println!("{:?}", key_val.insert("Baz".to_string(), "Baz".to_string()));

    println!();
    
    println!("{:?}", key_val.get_by_key(&"Foo".to_string()));
    println!("{:?}", key_val.get_by_val(&"Bar".to_string()));
    
    println!();

    println!("{:?}", key_val.get_by_key(&"Baz".to_string()));
    println!("{:?}", key_val.get_by_val(&"Baz".to_string()));
}
