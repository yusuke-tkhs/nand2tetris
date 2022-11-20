mod pre_processor;
mod syntax;

pub use syntax::*;

pub fn parse(input: String) -> Vec<Command> {
    let pre_processsed: Vec<String> = pre_processor::pre_process(input);

    unimplemented!()
}
#[cfg(test)]
mod tests {
    use super::*;
}
