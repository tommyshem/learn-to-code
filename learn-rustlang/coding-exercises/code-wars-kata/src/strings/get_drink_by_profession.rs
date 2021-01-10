// codewars Get drink by profession https://www.codewars.com/kata/568dc014440f03b13900001d/train/rust
use colored::*;

pub fn solution(){
    println!("{} Run test suit {} {}","Get drink by profession".green(),"cargo test get_drink_by_profession_basic_test".yellow(),"Not Finished".red().bold());
    let _solution = get_drink_by_profession("jabrOni");
}

// solution for the problem
fn get_drink_by_profession(param: &str) -> &'static str {
    // code me!
    ""
}

    #[test]
    fn get_drink_by_profession_basic_test() {
        assert_eq!(get_drink_by_profession("jabrOni"), "Patron Tequila", "'Jabroni' should map to 'Patron Tequila'");
        assert_eq!(get_drink_by_profession("scHOOl counselor"), "Anything with Alcohol", "'School Counselor' should map to 'Anything with alcohol'");
        assert_eq!(get_drink_by_profession("prOgramMer"), "Hipster Craft Beer", "'Programmer' should map to 'Hipster Craft Beer'");
        assert_eq!(get_drink_by_profession("bike ganG member"), "Moonshine", "'Bike Gang Member' should map to 'Moonshine'");
        assert_eq!(get_drink_by_profession("poLiTiCian"), "Your tax dollars", "'Politician' should map to 'Your tax dollars'");
        assert_eq!(get_drink_by_profession("rapper"), "Cristal", "'Rapper' should map to 'Cristal'");
        assert_eq!(get_drink_by_profession("pundit"), "Beer", "'Pundit' should map to 'Beer'");
        assert_eq!(get_drink_by_profession("Pug"), "Beer", "'Pug' should map to 'Beer'");
    }

