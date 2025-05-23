use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::{fs, io};

fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]

    {
        pub trait Person {
            fn name(&self) -> String;
        }

        pub trait Student: Person {
            fn id(&self) -> u32;
            fn field_of_study(&self) -> String;
        }

        pub struct Undergraduate {
            pub id: u32,
            pub name: String,
            pub field_of_study: String,
        }

        impl Person for Undergraduate {
            fn name(&self) -> String {
                self.name.clone()
            }
        }

        impl Student for Undergraduate {
            fn id(&self) -> u32 {
                self.id
            }

            fn field_of_study(&self) -> String {
                self.field_of_study.clone()
            }
        }
    }

    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}

pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    let len = slice.len();
    for i in indices {
        if *i < len {
            slice[*i] = value;
        }
    }
}

pub struct Person {
    // Define fields here
    // Read the description
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    person.age >= 18
}

// pub struct Rectangle(pub f32, pub f32); // 1. Finish the struct
//
// pub fn area(rect: &Rectangle) -> f32 {
//     rect.0 * rect.1
// }

pub struct Book {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: u32,
}

impl Book {
    // 2. Define the `new` associated function
    pub fn new(title: &str, author: &str, year: i32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            year,
            likes: 0,
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn log_message(message: &str) {
        println!("[LOG]: {}", message);
    }
}

pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }
}

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let mut vec = registry.entry(section.to_string()).or_insert(Vec::new());
    if !vec.contains(&animal.to_string()) {
        vec.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    if let Some(animals) = registry.get(section) {
        let mut vec = animals.clone();
        vec.sort();
        vec
    } else {
        Vec::new()
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut animals = Vec::new();
    for x in registry.values().into_iter() {
        animals.extend(x.clone());
    }
    animals.sort();
    animals
}

pub enum TrafficLight {
    // Define enum variants here
    Red,
    Yellow,
    Green,
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    // Your code here...
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}

pub enum Card {
    // Define the Card variants here
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    match card {
        Card::King => "King".to_string(),
        Card::Queen => "Queen".to_string(),
        Card::Jack => "Jack".to_string(),
        Card::Numbered(a, b) => {
            format!("{} of {}", a, b)
        }
    }
}

// pub enum Animal {
//     // Define the Animal variants here
//     Dog,
//     Cat(String),
//     Bird { species: String, can_fly: bool },
// }
//
// pub fn describe_animal(animal: &Animal) -> String {
//     match animal {
//         Animal::Dog => "A friendly dog.".to_string(),
//         Animal::Cat(name) => {
//             format!("A cat named {name}.")
//         }
//         Animal::Bird { species, can_fly } => {
//             if *can_fly {
//                 format!("A {species} that can fly.")
//             } else {
//                 format!("A {species} that cannot fly.")
//             }
//         }
//     }
// }

pub enum VehicleStatus {
    // Define the VehicleStatus variants here
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        match self {
            VehicleStatus::Parked | VehicleStatus::Driving { .. } => true,
            _ => false,
        }
    }

    pub fn description(&self) -> String {
        match self {
            VehicleStatus::Parked => "The vehicle is parked.".to_string(),
            VehicleStatus::Driving { speed } => {
                format!("The vehicle is driving at {speed} km/h.")
            }
            VehicleStatus::BrokenDown(reason) => {
                format!("The vehicle is broken down: {reason}.")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Cancelled(String),
}

// pub enum Message {
//     Text(String),
//     Number(i32),
//     Quit,
//     None,
// }
//
// pub fn process_text_message(message: &Message) -> String {
//     // Your code here...
//
//     if let Message::Text(content) = message {
//         format!("Processed Text: {content}")
//     } else {
//         String::from("Unhandled Message")
//     }
// }

pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32]) {
    // Your code here
    vec.extend(elements);
}

pub fn remove_element(vec: &mut Vec<i32>, index: usize) {
    // Your code here
    if index < vec.len() {
        vec.remove(index);
    }
}

pub fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).cloned()
}

/// Inserts a key-value pair into the hashmap or updates the value if the key exists.
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String) {
    // Your code here...
    map.insert(key, value);
}

/// Retrieves the value associated with a key from the hashmap.
pub fn get_value(map: &HashMap<String, String>, key: String) -> Option<String> {
    // Your code here...
    map.get(&key).cloned()
}

pub struct Student {
    // 1. Define the fields
    pub name: String,
    pub grades: Vec<u8>,
}

impl Student {
    pub fn add_grade(&mut self, grade: u8) {
        self.grades.push(grade);
    }

    pub fn average_grade(&self) -> f64 {
        if self.grades.len() == 0 {
            0f64
        } else {
            self.grades.iter().sum::<u8>() as f64 / self.grades.len() as f64
        }
    }
}

pub struct StudentGrades {
    // 2. Define the fields
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str) {
        self.students.entry(name.to_string()).or_insert(Student {
            name: name.to_string(),
            grades: vec![],
        });
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_grade(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
        &self.students.get(name).unwrap().grades
    }
}

pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Your code here...
    numbers.iter().find(|&&x| x % 2 == 0).cloned()
}

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl Display for ParsePercentageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    let percentage = input
        .parse::<u8>()
        .map_err(|e| ParsePercentageError::InvalidInput)?;
    if percentage > 100 {
        Err(ParsePercentageError::OutOfRange)
    } else {
        Ok(percentage)
    }
}

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
    let f = File::open(file_path)?;
    let f = BufReader::new(f);
    let mut sum = 0;
    for line in f.lines() {
        if let Ok(l) = line {
            let num = l
                .parse::<i32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            sum += num;
        } else {
            return Err(io::Error::new(ErrorKind::Other, "oh no!"));
        }
    }

    Ok(sum)
}

pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) -> Option<i32> {
    // HINT: `numbers.get` returns a Option<i32> value

    let num1 = numbers.get(index1)?;
    let num2 = numbers.get(index2)?;
    Some(num1 * num2)
}

pub fn read_file(file_path: &str) -> Option<String> {
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let mut f = File::open(file_path).ok()?;
    let mut content = String::new();
    f.read_to_string(&mut content).ok()?;
    if content.eq("Cannot read this file.") {
        None
    } else {
        Some(content)
    }
}

pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // Finish the function
    let first_element = numbers.first().ok_or("Vector is empty")?; // <- Returns an Option<&i32>
    if *first_element >= min_value {
        Ok(*first_element)
    } else {
        Err("First element is below the minimum allowed value".to_string())
    }
}

pub fn get_database_url() -> String {
    // Your code here...
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.");
    if url.starts_with("postgresql://") {
        return url;
    }
    panic!("DATABASE_URL must start with 'postgresql://'");
}

pub fn read_file_to_string(path: &str) -> String {
    // 1. Implement the function
    let mut content = String::new();
    let err = format!("Failed to read file: \"{}\"", path);
    File::open(path)
        .expect(err.as_str())
        .read_to_string(&mut content)
        .expect(err.as_str());
    content
}

pub fn get_env_variable(key: &str) -> String {
    std::env::var(key).unwrap()
}

pub trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {}, Age: {}", self.name, self.age)
    }
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}, Author: {}", self.title, self.author)
    }
}

pub struct ItemContainer<T> {
    pub item: T,
}

impl<T> ItemContainer<T> {
    pub fn get_item(&self) -> &T {
        &self.item
    }
}

pub fn compare_and_display<T: Display + PartialOrd>(a: T, b: T) -> T {
    println!("{}", a);
    println!("{}", b);
    if a > b {
        a
    } else {
        b
    }
}

pub fn print_message<T: AsRef<str>>(a: T) {
    println!("{}", a.as_ref());
}

pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Self {
        Self(minutes.0 / 60)
    }
}

impl From<Hours> for Days {
    fn from(value: Hours) -> Self {
        Self(value.0 / 24)
    }
}

impl From<Minutes> for Days {
    fn from(minutes: Minutes) -> Self {
        let hour: Hours = minutes.into();
        hour.into()
    }
}

impl From<Days> for Hours {
    fn from(value: Days) -> Self {
        Self(value.0 * 24)
    }
}

pub fn filter_starts_with<'a>(a: &'a [String], b: &'a str) -> impl Iterator<Item = &'a String> {
    a.iter().filter(move |s| s.starts_with(b))
}

pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        "Beep boop".to_string()
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => Box::new(Dog {
            name: "Bob".to_string(),
            breed: "Bob".to_string(),
        }),
        "robot" => {
            // Return a Robot instance here
            Box::new(Robot {
                model: "Robot".to_string(),
                purpose: "Robot".to_string(),
            })
        }
        _ => panic!("Unknown speaker type"),
    }
}

pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Renderable for Circle {
    fn render(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

impl Renderable for Rectangle {
    fn render(&self) -> String {
        format!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        )
    }
}

pub struct Canvas {
    shapes: Vec<Box<dyn Renderable>>,
}

impl Canvas {
    // Initializes an empty canvas.
    pub fn new() -> Self {
        Canvas { shapes: Vec::new() }
    }

    // Adds a shape to the canvas.
    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        self.shapes.push(shape);
    }

    // Returns a vector of strings, each representing a rendered shape.
    pub fn render_all(&self) -> Vec<String> {
        self.shapes.iter().map(|shape| shape.render()).collect()
    }
}

pub trait KeyValueStore {
    type Key;
    type Value;

    // Method to add a key-value pair
    fn set(&mut self, key: Self::Key, value: Self::Value);

    // Method to get a value by key
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

// Define the struct InMemoryStore with public fields
pub struct InMemoryStore<K, V> {
    pub storage: HashMap<K, V>,
}

impl<K, V> InMemoryStore<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    // Constructor to create a new InMemoryStore
    pub fn new() -> Self {
        InMemoryStore {
            storage: HashMap::new(),
        }
    }
}

// Implement the KeyValueStore trait for InMemoryStore
impl<K, V> KeyValueStore for InMemoryStore<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    type Key = K;
    type Value = V;

    fn set(&mut self, key: Self::Key, value: Self::Value) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.storage.get(key)
    }
}

pub trait Plugin {
    // 1. Finish the trait
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    pub plugins: Vec<Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name();
        let has_same_name = self.plugins.iter().any(|p| p.name() == name);
        if has_same_name {
            panic!("Plugin with name '{}' already exists", name);
        }
        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, plugin_name: &str) -> Option<Box<dyn Plugin>> {
        let find = self.plugins.iter().position(|p| p.name() == plugin_name);
        if let Some(index) = find {
            Some(self.plugins.remove(index))
        } else {
            None
        }
    }

    pub fn execute_all(&self) {
        for plugin in self.plugins.iter() {
            plugin.execute();
        }
    }
}

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.chars().count() < b.chars().count() {
        b
    } else {
        a
    }
}

// 1. Define the struct
pub struct TextFinder<'a>(&'a str);

impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        TextFinder(text)
    }

    pub fn find_first(&self, text: &str) -> Option<&'a str> {
        let lines = self.0.split('\n').collect::<Vec<&str>>();
        lines
            .iter()
            .find(|line| line.contains(text))
            .map(|line| *line)
    }

    pub fn find_many(&self, text: &str) -> Vec<&'a str> {
        let lines = self.0.split('\n').collect::<Vec<&str>>();
        lines
            .iter()
            .filter(|line| line.contains(text))
            .map(|line| *line)
            .collect()
    }
}

// 1. Finish the struct definition
pub struct MutableTextFinder<'a>(&'a mut String);

impl<'a> MutableTextFinder<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self(text)
    }

    pub fn get_text(&self) -> &String {
        self.0
    }

    pub fn find_first(&self, text: &str) -> Option<&str> {
        let lines = self.0.split('\n').collect::<Vec<&str>>();
        lines
            .iter()
            .find(|line| line.contains(text))
            .map(move |line| *line)
    }

    pub fn replace_lines(&mut self, text: &str, replacement: &str) {
        // Split the original string into lines
        for x in self.0.lines() {}
        let lines: Vec<&str> = self.0.lines().collect();

        // Iterate over the lines and replace lines containing the keyword
        let replaced_lines: Vec<String> = lines
            .iter()
            .map(|line| {
                if line.contains(text) {
                    replacement.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        self.0.clear();
        self.0.push_str(&replaced_lines.join("\n"));

        // Join the lines back together
        // let mut a = replaced_lines.join("\n");
        // self.0 = &a;
    }

    pub fn replace_all_occurrence(&mut self, text: &str, replacement: &str) {
        let index_diff = (replacement.len() - text.len()) as i32;
        let all_match = self
            .0
            .match_indices(text)
            .map(|(x, _)| x)
            .collect::<Vec<usize>>();
        let mut diff = 0i32;
        for x in all_match {
            self.0.replace_range(
                ((x as i32 + diff) as usize)..((x as i32 + diff) as usize + text.len()),
                replacement,
            );
            diff += index_diff;
        }
    }
}

mod test {
    use crate::MutableTextFinder;

    #[test]
    fn test_mutable_text_finder() {
        let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
        let mut finder = MutableTextFinder::new(&mut text);

        let first = finder.find_first("Rust");
        println!("{:?}", first); // Should print: Some("Rust is awesome")

        finder.replace_lines("Rust", "Programming in Rust");
        println!("{}", finder.get_text()); // Should print the modified text
    }

    #[test]
    fn test_get_text_after_modification() {
        let mut text = String::from("Rust is awesome\nReplace this line\nFinal line");
        let mut finder = MutableTextFinder::new(&mut text);

        finder.replace_lines("Replace", "This line has been replaced");
        assert_eq!(
            finder.get_text(),
            "Rust is awesome\nThis line has been replaced\nFinal line"
        );
    }
}

pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| {
        // Step 1: Implement here
        a + b
    };

    // Step 2:
    // Create the `subtract_closure` closure that subtracts two `i32` values.

    let subtract_closure = |a, b| {
        // Step 1: Implement here
        a - b
    };
    // Step 3:
    // Create the `multiply_closure` closure that multiplies two `i32` values.

    let multiply_closure = |a, b| {
        // Step 1: Implement here
        a * b
    };

    (add_closure, subtract_closure, multiply_closure)
}

// 1. Based on the `main` function below,
// Find out the types of the closures and define them
pub fn create_typed_closures() -> (
    impl Fn(f32, f32) -> f32,
    impl FnMut(&mut f32, f32),
    impl FnOnce(String) -> String,
) {
    // 2. Implement calculate_total closure here
    let calculate_total = |price: f32, tax_rate: f32| price + price * tax_rate;

    // 3. Implement apply_discount closure here
    let apply_discount = |price: &mut f32, tax_rate: f32| {
        *price -= tax_rate;
    };

    // 4. Implement checkout_cart closure here
    let checkout_cart = |content: String| format!("Checkout complete: {}", content);

    (calculate_total, apply_discount, checkout_cart)
}

mod test_closure {
    use crate::create_typed_closures;

    #[test]
    fn test() {
        let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

        // Example tests
        assert_eq!(calculate_total(100.0, 0.2), 120.0);

        let mut total_price = 120.0;
        apply_discount(&mut total_price, 20.0);
        assert_eq!(total_price, 100.0);

        let cart_details = String::from("Items: Apple, Banana, Orange");
        assert_eq!(
            checkout_cart(cart_details),
            "Checkout complete: Items: Apple, Banana, Orange"
        );
    }
}

pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    iter.filter(|&n| n % 2 != 0).collect()
}

// 2. Finish the function here
pub fn uppercase_strings<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<String> {
    iter.map(|x| x.to_uppercase()).collect()
}

pub fn unique_items<I, K>(iterator: I) -> Vec<String>
where
    K: AsRef<str>,
    I: Iterator<Item = K>,
{
    let mut seen = HashSet::new();
    let mut result: Vec<String> = iterator
        .filter_map(|item| {
            let trimmed = item.as_ref().trim().to_string();
            if !trimmed.is_empty() && seen.insert(trimmed.clone()) {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect();

    result.sort();
    result
}

mod test_unique_items {
    use crate::unique_items;

    #[test]
    pub fn test() {
        let product_ids = vec![
            "abc123".to_string(),
            "  ".to_string(),
            "def456".to_string(),
            "abc123".to_string(),
            "ghi789".to_string(),
            "ghi789".to_string(),
            "   def456".to_string(),
        ];

        let unique_ids = unique_items(product_ids.into_iter());
        assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
    }
}

pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    Box::new(Animal {
        name: name.to_string(),
        age,
    })
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    // Your code here
    (animal.name, animal.age)
}

use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    // 1. Define the `new` associated function
    pub fn new(path: impl AsRef<str>) -> Result<TempFile, io::Error> {
        File::create(path.as_ref())?;

        Ok(TempFile {
            path: PathBuf::from(path.as_ref()),
        })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete temporary file: {}", e);
        }
    }
}

use std::rc::Rc;

pub fn use_shared_data<T: Display>(data: Rc<Vec<T>>) {
    // 1. Loop over each item in the vector and print it using `println!`
    data.iter().for_each(|x| {
        println!("{}", x);
    })
}

pub fn share_data_to_other_functions<F>(mut take_item: F, items: Vec<String>)
where
    F: FnMut(Rc<Vec<String>>),
{
    let items = Rc::new(items);
    // 2. Implement the function
    // Share the data as a reference-counted pointer 3 times with the closure
    take_item(items.clone());
    take_item(items.clone());
    take_item(items.clone());
}

use std::cell::RefCell;
use std::ops::{Add, DerefMut};

pub fn push<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    // 1. Finish the function
    data.borrow_mut().push(element);
}

pub fn iterate_and_print_shared_data<T: Display>(data: Rc<RefCell<Vec<T>>>) {
    // 2. Borrow the data and print each item
    data.borrow().iter().for_each(|element| {
        println!("{}", element);
    })
}

pub fn plus_one(data: Rc<RefCell<i32>>) {
    // 3. Finish the function
    let mut cell = data.borrow_mut();
    *cell += 1;
}

use std::thread;

pub fn concurrent_add<T: Add<Output = T> + Send + Copy + 'static>(
    items: Vec<T>,
    num: T,
) -> Vec<thread::JoinHandle<T>> {
    // Implement the function here
    items
        .into_iter()
        .map(|item| thread::spawn(move || item.add(num)))
        .collect::<Vec<thread::JoinHandle<T>>>()
}

mod test_concurrent_add {
    use crate::concurrent_add;

    #[test]
    pub fn test() {
        {
            let mut list = vec![1, 2, 3, 4, 5];

            let handles = concurrent_add(list.clone(), 3);

            for handle in handles {
                let result = handle.join().unwrap();
                let original = list.remove(0);

                assert_eq!(result, original + 3);
            }
        }

        {
            let mut list = vec![10., 20., 30., 40., 50.];

            let handles = concurrent_add(list.clone(), 5.);

            for handle in handles {
                let result = handle.join().unwrap();
                let original = list.remove(0);

                assert_eq!(result, original + 5.);
            }
        }
    }
}

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

#[derive(Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    channel()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    // - Updates the priority based on content
    // - Sends the updated message through the channel
    thread::spawn(move || {
        messages.iter().for_each(|m| {
            let mut msg = Message {
                priority: Priority::Low,
                content: m.content.clone(),
                sender_id: m.sender_id,
            };
            if m.content.starts_with("ERROR") {
                msg.priority = Priority::Critical;
            } else if m.content.starts_with("WARNING") {
                msg.priority = Priority::High;
            } else if m.content.starts_with("DEBUG") {
                msg.priority = Priority::Medium;
            }
            tx.send(msg).unwrap();
        });
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    // - Receives messages from the channel
    // - Formats them as "[PRIORITY|SENDER_ID] CONTENT"
    // - Returns a vector of formatted messages
    thread::spawn(move || {
        let mut vec: Vec<String> = vec![];
        rx.iter().for_each(|m| {
            vec.push(format!(
                "[{}|{}] {}",
                match m.priority {
                    Priority::Low => "LOW",
                    Priority::Medium => "MED",
                    Priority::High => "HIGH",
                    Priority::Critical => "CRIT",
                },
                m.sender_id,
                m.content
            ));
        });
        vec
    })
}

use std::sync::{Arc, Mutex};

pub fn create_shared_data<T>(initial: T) -> Arc<Mutex<T>> {
    // 1. Initialize and return a new Arc<Mutex<T>> with the initial value
    Arc::new(Mutex::new(initial))
}

pub fn increment_counter(
    counter: Arc<Mutex<i32>>,
    threads: usize,
    increments: usize,
) -> Vec<JoinHandle<()>> {
    // 2. Increment the counter by the given increments using the given number of threads
    (0..threads)
        .map(|thread_id| {
            thread::spawn({
                let mut counter = counter.clone();
                move || {
                    let mut counter = counter.lock().unwrap();
                    *counter = *counter + increments as i32;
                }
            })
        })
        .collect::<Vec<JoinHandle<()>>>()
}

pub fn modify_shared_data<T: Send + 'static>(
    data: Arc<Mutex<T>>,
    modifier: fn(&mut T),
) -> JoinHandle<()> {
    // 3. Use a new thread to modify the shared data

    thread::spawn({
        let mut a = data.clone();
        move || {
            modifier(a.lock().unwrap().deref_mut());
        }
    })
}

mod test_modify_shared_data {
    use crate::{create_shared_data, increment_counter, modify_shared_data};
    use std::sync::Arc;

    #[test]
    pub fn test() {
        let counter = create_shared_data(0);
        let handles = increment_counter(Arc::clone(&counter), 5, 10);
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Counter value: {}", *counter.lock().unwrap());

        let shared_string = create_shared_data(String::from("Hello"));
        let handle = modify_shared_data(shared_string.clone(), |s| s.push_str(" World"));
        handle.join().unwrap();
        println!("Modified string: {}", *shared_string.lock().unwrap());
    }
}

#[derive(Debug, Clone)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

impl BookItem {
    pub fn check_validity(&self) -> bool {
        match self {
            BookItem::Book { pages, discount } => {
                let page_valid = *pages > 0i32;
                if page_valid {
                    if let Some(discount) = discount {
                        *discount >= 0 && *discount <= 50
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            BookItem::EBook(title, (a, b)) => !title.is_empty() && *b > 0i32,
            BookItem::Collection(items) => {
                !items.is_empty() && items.iter().all(|i| i.check_validity())
            }
            BookItem::OutOfPrint => false,
        }
    }
}

mod test_book_item {
    use crate::BookItem;

    #[test]
    pub fn test() {
        let book_a = BookItem::Book {
            pages: 42,
            discount: Some(100),
        };
        let ebook_b = BookItem::EBook("hello".to_string(), (1, 2));
        let collection_c = BookItem::Collection(vec![book_a.clone(), BookItem::OutOfPrint]);

        assert!(
            !book_a.check_validity(),
            "Book with discount > 50 should be invalid"
        );
        assert!(
            ebook_b.check_validity(),
            "EBook with valid title and tuple should be valid"
        );
        assert!(
            !collection_c.check_validity(),
            "Collection containing invalid items should be invalid"
        );
        assert!(
            !BookItem::OutOfPrint.check_validity(),
            "OutOfPrint should always be invalid"
        );
    }
}

pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
        Self(rhs.0 * 1000 + self.0)
    }
}

#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
    pub auto_save: bool,
    pub cache_size_mb: u32,
    pub log_level: String,
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "Light".to_string(),
            notifications_enabled: true,
            max_users: 100u32,
            auto_save: true,
            cache_size_mb: 512,
            log_level: "INFO".to_string(),
            retry_attempts: 3,
            timeout_seconds: 30,
        }
    }
}

pub fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    // Your code here...
    let mut sum = 0;
    for x in start..=end {
        if x % 2 == 0 {
            sum += x;
        }
    }
    sum
}

pub fn countdown(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];
    let mut n = n;
    while n > 0u32 {
        vec.push(n);
        n -= 1;
    }
    vec.push(0u32);
    vec
}

pub fn weekday_from_number(day: u8) -> &'static str {
    // TODO: Implement the function here
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day number",
    }
}

pub fn fizz_buzz(num: u32) -> String {
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else {
        num.to_string()
    }
}

pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn is_prime(n: u32) -> bool {
    // Implement your code here
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn describe_number(n: i32) -> String {
    // TODO: Implement the function here
    if n == 0 {
        "Zero".to_string()
    } else if n > 0 {
        if n % 2 == 0 {
            "Positive even".to_string()
        } else {
            "Positive odd".to_string()
        }
    } else {
        if n % 2 == 0 {
            "Negative even".to_string()
        } else {
            "Negative odd".to_string()
        }
    }
}

fn factorial(n: u32) -> u128 {
    (1..=n).fold(1u128, |acc, x| acc * x as u128)
}

pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    match (from_unit, to_unit) {
        ("C", "F") => Ok(value * 9.0 / 5.0 + 32.0),
        ("F", "C") => Ok((value - 32.0) * 5.0 / 9.0),
        ("C", "K") => Ok(value + 273.15),
        ("K", "C") => Ok(value - 273.15),
        ("F", "K") => {
            let celsius = (value - 32.0) * 5.0 / 9.0;
            Ok(celsius + 273.15)
        }
        ("K", "F") => {
            let celsius = value - 273.15;
            Ok(celsius * 9.0 / 5.0 + 32.0)
        }
        ("C", "C") | ("F", "F") | ("K", "K") => Ok(value), // Handle same unit conversion
        _ => Err(String::from("Invalid unit")),
    }
}

pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    // Implement here
    // Check if age is valid
    if age < 0 || age > 120 {
        return Err(String::from("Invalid age"));
    }

    // Check if email contains '@' symbol
    if !email.contains('@') {
        return Err(String::from("Invalid email"));
    }

    // If both checks pass, return Ok(())
    Ok(())
}

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    if numbers.is_empty() {
        return 0.0;
    }

    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid] as f32 + numbers[mid - 1] as f32) / 2.0
    } else {
        numbers[mid] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    // TODO: Implement logic here to return the mode of the list
    let mut occurrences = HashMap::new();
    let mut order = Vec::new();

    // Count the occurrences of each number and keep track of their order
    for &num in numbers {
        let counter = occurrences.entry(num).or_insert(0);
        *counter += 1;
        if *counter == 1 {
            order.push(num);
        }
    }

    // Determine the highest frequency
    let max_count = occurrences.values().cloned().max().unwrap_or(0);

    // Collect all numbers with the highest frequency, preserving the original order
    order
        .into_iter()
        .filter(|&num| occurrences[&num] == max_count)
        .collect()
}

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    if start <= end {
        for num in start..=end {
            if is_palindrome(num) {
                return Some(num);
            }
        }
    } else {
        for num in end..=start {
            if is_palindrome(num) {
                return Some(num);
            }
        }
    }
    None
}

#[derive(Debug)]
enum State {
    Start,
    SeenA,
    SeenB,
    SeenC,
    Invalid,
}

fn transition(state: State, input: char) -> State {
    match state {
        State::Start => match input {
            'a' => State::SeenA,
            _ => State::Invalid,
        },
        State::SeenA => match input {
            'b' => State::SeenB,
            'c' => State::SeenC,
            _ => State::Invalid,
        },
        State::SeenB => match input {
            'b' => State::SeenB,
            'c' => State::SeenC,
            _ => State::Invalid,
        },
        State::SeenC => State::Invalid,
        State::Invalid => State::Invalid,
    }
}

fn recognize_pattern(input: &str) -> bool {
    let mut state = State::Start;

    for c in input.chars() {
        state = transition(state, c);
        if let State::Invalid = state {
            return false;
        }
    }

    matches!(state, State::SeenC)
}

use std::collections::VecDeque;

fn neighbors(pos: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x < max_x - 1 {
        neighbors.push((x + 1, y));
    }
    if y < max_y - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut parent = vec![vec![None; maze[0].len()]; maze.len()];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some(current) = queue.pop_front() {
        if current == end {
            let mut path = Vec::new();
            let mut step = Some(current);
            while let Some(pos) = step {
                path.push(pos);
                step = parent[pos.0][pos.1];
            }
            path.reverse();
            return path;
        }

        for neighbor in neighbors(current, maze.len(), maze[0].len()) {
            if !visited[neighbor.0][neighbor.1] && maze[neighbor.0][neighbor.1] != '#' {
                queue.push_back(neighbor);
                visited[neighbor.0][neighbor.1] = true;
                parent[neighbor.0][neighbor.1] = Some(current);
            }
        }
    }

    vec![]
}

macro_rules! x_vec {
    () => {
        Vec::new()
    };
    (ept) => {
        Vec::new()
    };
    {$x:expr} => {
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };
    {$x:ident, $v:expr} => {
        // let $x = {
            let mut $x = Vec::new();
            $x.push($v);
        // };
    };
    ($x:ident, $($v:expr),+) => {
        let mut $x = Vec::new();
        $(
            $x.push($v);
        )+
    };
    [$($x:expr),*] => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    }
}

mod test_macro {
    #[test]
    pub fn test() {
        let a: Vec<i32> = x_vec!();
        println!("{:?}", a.len());
        let b: Vec<i32> = x_vec!(ept);
        println!("{:?}", b.len());
        let c = x_vec!(10i32);
        println!("{:?}", c.len());

        x_vec!(abc, 123);
        println!("{:?}", abc.len());

        x_vec!(abc, 1, 2, 3, 4, 5);
        println!("{:?}", abc.len());

        let abc = vec![1, 2, 3, 4, 5];
        println!("{:?}", abc.len());
    }
}

#[macro_export]
macro_rules! math_operations {
    ($a:expr, $b:literal, $c:expr) => {{
        let d = match $b {
            "+" => $a + $c,
            "-" => $a - $c,
            "*" => $a * $c,
            "/" => {
                if $c == 0 {
                    panic!("Division by zero");
                }
                $a / $c
            }
            _ => {
                panic!("Unsupported operator: {}", $b);
            }
        };
        format!("{} {} {} = {}", $a, $b, $c, d)
    }};
}

mod test_math_operations {
    #[test]
    pub fn test() {
        assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
        assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7");
        assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24");
        assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5");
    }
}

pub trait ConfigDefault {
    fn get_default() -> Self;
}

// These types need default values
pub struct ConnectionTimeout(pub u64);
pub struct MaxConnections(pub u32);
pub struct RetryAttempts(pub u8);
pub struct PostgresPort(pub u16);
pub struct MySQLPort(pub u16);
pub struct MongoPort(pub u16);
pub struct RedisPort(pub u16);

#[macro_export]
macro_rules! config_default_impl {
    ($x:ident, $v:expr) => {
        impl ConfigDefault for $x {
            fn get_default() -> Self {
                Self($v)
            }
        }
    };
}

config_default_impl!(ConnectionTimeout, 30);
config_default_impl!(MaxConnections, 100);
config_default_impl!(RetryAttempts, 3);
config_default_impl!(PostgresPort, 5432);
config_default_impl!(MySQLPort, 3306);
config_default_impl!(MongoPort, 27017);
config_default_impl!(RedisPort, 6379);

mod test_config_default_impl {
    use crate::ConfigDefault;

    #[test]
    pub fn test() {
        // let's say we have a new struct
        struct CustomPort(pub u16);

        // we implement the ConfigDefault trait for CustomPort
        config_default_impl!(CustomPort, 8080);

        // when running the `get_default` method, it should return the default value
        assert_eq!(<CustomPort as ConfigDefault>::get_default().0, 8080);
    }
}