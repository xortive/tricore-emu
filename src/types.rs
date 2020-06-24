pub type Handler<Instruction, Context> = fn(Instruction, &mut Context);

pub trait CPU<Identifier, Instruction> {
  fn execute(&mut self, instruction: Instruction);
}

pub trait Registry<Identifier, Instruction, Context> {
  fn handler_for(&self, instruction: Instruction) -> Handler<Instruction, Context>;
  fn register(&mut self, id: Identifier, handler: Handler<Instruction, Context>);
}