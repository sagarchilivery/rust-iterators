use std::any::type_name;

pub fn iterator_test() {
    let fruit_list: Vec<&str> = vec![
        "Watermelon",
        "Banana",
        "Mango",
        "Gauva",
        "Orange",
        "Apple",
        "Pineapple",
        "Cherry",
    ];

    println!("\n\nFruit List : {:?} \n", fruit_list);

    let fruit_iterator = fruit_list.iter();

    println!("Fruit Iterator : {:?} \n", fruit_iterator);

    let nut_list: Vec<&str> = vec![
        "Hazelnut",
        "Almond",
        "Cashew",
        "Walnut",
        "Pecan",
        "Pistachio",
        "Peanut",
        "Pine nut",
    ];

    println!("Nut List : {:?} \n", nut_list);

    let nut_iterator = nut_list.iter();

    println!("Nut Iterator : {:?} \n", nut_iterator);
 
    let all_items = fruit_iterator.chain(nut_iterator);

    let all_food: Vec<&&str> = all_items.clone().collect();

    println!("{all_food:?}");
    for item in all_items {
        // for item in all_food {
        println!("Eating : {:?}", item);
    }
}
