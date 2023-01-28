struct Item(String, i64);

fn main() {
    //let banana = Item("바나나".to_string(), 300);
    let banana = Item{0:"바나나".to_string(), 1:300};
    let apple = Item("사과".to_string(), 200);

    let items = vec![banana,apple];

    let total = print_and_sum_items(&items);
    println!("sum = {}", total);
}

fn print_tuple(item: &Item) {
    println!("{}, {}", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total 
}