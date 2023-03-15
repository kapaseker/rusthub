mod front_of_house;

use front_of_house::hosting::add_to_waitlist as wait_list;
use front_of_house::serving::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        wait_list();
    }

    #[test]
    fn tryServing() {
        take_order();
        serve_order();
    }
}
