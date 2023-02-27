mod front_of_house;

use front_of_house::hosting::add_to_waitlist as wait_list;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        wait_list();
    }
}
