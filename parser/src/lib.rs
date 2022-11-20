#[derive(Debug)]
pub struct Address([bool; 15]);
#[derive(Debug)]
pub struct Comp([bool; 15]);
#[derive(Debug)]
pub struct Dest([bool; 15]);
#[derive(Debug)]
pub struct Jump([bool; 15]);
#[derive(Debug)]
pub enum Instruction {
    A(Address),
    C { comp: Comp, dest: Dest, jump: Jump },
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
