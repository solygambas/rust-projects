use std::collections::HashMap;

pub fn run() {
    // creating
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // zip creates an iterator of tuples > collect turns that into a hash map

    // ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name & field_value are no longer valid at this point

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Some(&10)

    // iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value); // Blue: 10, Yellow: 50
    }

    // updating
    // overwriting
    scores.insert(String::from("Green"), 30);
    scores.insert(String::from("Green"), 25);

    // inserting if the key has no value
    scores.entry(String::from("Red")).or_insert(50);

    // updating based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"wonderful": 1, "world": 2, "hello": 1}
}
