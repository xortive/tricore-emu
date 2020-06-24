use crate::{
  types, 
  types::Registry 
};
use crate::cpu::TricoreContext;

type Handler = types::Handler<u32, TricoreContext>;

pub struct TricoreInstructionRegistry {
  handlers: [Option<Handler>; 256],
}

impl Registry<u8, u32, TricoreContext> for TricoreInstructionRegistry {
  fn handler_for(&self, instruction: u32) -> Handler {
      let op1 = (instruction & 0xFF) as usize;
      match self.handlers[op1] {
        Some(handler) => handler,
        None => unimplemented!("Op1 0x{:X?} not implemented!", op1)
      }
  }

  fn register(&mut self, id: u8, handler: Handler) {
    self.handlers[id as usize] = Some(handler);
  }
}

impl TricoreInstructionRegistry {
  pub fn new() -> Self {
    let handlers = {
      // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
      // safe because the type we are claiming to have initialized here is a
      // bunch of `MaybeUninit`s, which do not require initialization.
      let mut handlers: [std::mem::MaybeUninit<Option<Handler>>; 256] = unsafe {
          std::mem::MaybeUninit::uninit().assume_init()
      };
  
      // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
      // assignment instead of `ptr::write` does not cause the old
      // uninitialized value to be dropped. Also if there is a panic during
      // this loop, we have a memory leak, but there is no memory safety
      // issue.
      for handler in &mut handlers[..] {
          unsafe { std::ptr::write(handler.as_mut_ptr(), None); }
      }
  
      unsafe { std::mem::transmute::<_, [Option<Handler>; 256]>(handlers) }
    };

    return Self {
      handlers
    }
  }
}