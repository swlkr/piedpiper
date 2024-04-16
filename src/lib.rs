pub use piedpiper_macros::pp;

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn it_works() {
        fn append(s: impl Display, append: impl Display) -> String {
            format!("{}{}", s, append)
        }

        let result = pp! {
          "1"
          |> append(_, "2")
          |> append(_, "3")
        };

        assert_eq!(result, "123");

        fn add(x: u64, y: u64) -> u64 {
            x + y
        }

        let result = pp! {
            1
            |> add(_, 2)
            |> add(_, 3)
        };

        assert_eq!(result, 6);
    }
}
