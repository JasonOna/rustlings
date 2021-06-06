// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 5. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object

impl FromStr for Person {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
      if s.len() == 0 {
        return Err(Box::new(MyError::new("Boom")))
      }

      let mut attributes: Vec<&str> = s.split(',').collect();
      if attributes.len() != 2 {
        return Err(Box::new(MyError::new("Boom")))
      }
      if attributes[0].len() == 0 {
        return Err(Box::new(MyError::new("Boom")))
      }
      if attributes[1].len() == 0 {
        return Err(Box::new(MyError::new("Boom")))
      }

      let age: usize;
      let name: String;

      match attributes.pop() {
        Some(a) => {
          match a.parse::<usize>() {
              Ok(x) => {
                age = x;
              },
              Err(e) => {
                return Err(Box::new(MyError::new("Boom")))
              },
          };
        },
        None => {
          return Err(Box::new(MyError::new("Boom")))
        }
      }

      // 4. If the name is empty, then return the default of Person
      match attributes.pop() {
        Some(n) => {
          if n.len() == 0 {
            return Err(Box::new(MyError::new("Boom")))
          } else {
            name = n.to_string();
          }
        },
        None => {
          return Err(Box::new(MyError::new("Boom")))
        }
      }
      // 5. Extract the other element from the split operation and parse it into a `usize` as the age
      // If while parsing the age, something goes wrong, then return the default of Person
      // Otherwise, then return an instantiated Person object with the results
      Ok(Person {
        name: name,
        age: age,
      })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!("John,".parse::<Person>().is_err());
    }

    #[test]
    fn invalid_age() {
        assert!("John,twenty".parse::<Person>().is_err());
    }

    #[test]
    fn missing_comma_and_age() {
        assert!("John".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name() {
        assert!(",1".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_age() {
        assert!(",".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(",one".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma() {
        assert!("John,32,".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert!("John,32,man".parse::<Person>().is_err());
    }
}
