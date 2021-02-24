#[derive(Debug, PartialEq, Eq)]
pub struct Problem {}

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {}

pub fn solve(_problem: Problem) -> Solution {
    todo!()
}

#[cfg(test)]
mod test {

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;
    #[test]
    pub fn example_is_correct() -> Result {
        Ok(())
    }

    #[test]
    pub fn example_works() -> Result {
        Ok(())
    }
}
