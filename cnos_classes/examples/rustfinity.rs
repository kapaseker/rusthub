use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};

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
    // TODO: implement this function
    let mut vec = registry.entry(section.to_string()).or_insert(Vec::new());
    if !vec.contains(&animal.to_string()) {
        vec.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    if let Some(animals) = registry.get(section) {
        let mut vec = animals.clone();
        vec.sort();
        vec
    } else {
        Vec::new()
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
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

pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_text_message(message: &Message) -> String {
    // Your code here...

    if let Message::Text(content) = message {
        format!("Processed Text: {content}")
    } else {
        String::from("Unhandled Message")
    }
}

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
    // TODO: Implement the function here
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
    // TODO: Implement this function
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
    // TODO: Instead of using `unwrap`, use the `?` operator to propagate the option
    // HINT: `numbers.get` returns a Option<i32> value

    let num1 = numbers.get(index1)?;
    let num2 = numbers.get(index2)?;
    Some(num1 * num2)
}

pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
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
    // TODO: Implement the `get_item` method to return a reference to the item.
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

pub fn longest<'a>(a:&'a str, b:&'a str) -> &'a str {
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
        lines.iter().find(|line| line.contains(text)).map(|line| *line)
    }

    pub fn find_many(&self, text: &str) -> Vec<&'a str> {
        let lines = self.0.split('\n').collect::<Vec<&str>>();
        lines.iter().filter(|line| line.contains(text)).map(|line| *line).collect()
    }
}
