fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut msg = String::from(input);
    msg.push_str(" world!");
    msg
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let msg: String = input.replace("cars", "balloons");
    msg
}

fn main() {
    // You can optionally experiment here.
    println!("trim1 : {:?}", trim_me("Hello!     "));
    println!("trim2 : {:?}", trim_me("  What's up!"));
    println!("trim3 : {:?}", trim_me("   Hola!  "));
    println!("trim4 : {:?}", trim_me("Hi!"));
    println!("compose1 : {:?}", compose_me("Hello"));
    println!("compose2 : {:?}", compose_me("Goodbye"));
    println!("replace1 : {:?}", replace_me("I think cars are cool"));
    println!("replace2 : {:?}", replace_me("I love to look at cars"));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
