mod types;
mod handlers;
mod cpu;
use types::*;
use handlers::TricoreInstructionRegistry;
use cpu::TricoreContext;

struct Tricore {
    cur_context: TricoreContext,
    registry: TricoreInstructionRegistry
}

impl CPU<u8, u32> for Tricore {
    fn execute(&mut self, instruction: u32) {
        self.registry.handler_for(instruction)(instruction, &mut self.cur_context);
    }
}

impl Tricore {
    pub fn new() -> Self {
        let mut registry = TricoreInstructionRegistry::new();
        let cur_context = TricoreContext {};

        registry.register(0x69, Self::test69);

        Self {
            registry, cur_context
        }
    }

    fn test69(instruction: u32, context: &mut TricoreContext) {
        print!("I am the test69 handler");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tricore = Tricore::new();
        tricore.execute(0xDEAD0069);
    }
}
