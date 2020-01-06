use super::datastructure::{Cons, EnvItem, EnvItemFunctionWrapped, Environment, Item};
use std::collections::VecDeque;
//use std::ops::{Deref, DerefMut};

mod error;

use error::VMError;

#[derive(Clone)]
enum Instruction {
    Eval(Item),
    EvalFunction(Item),
    Pop,
    Call,
    CrashIfNotFunction,
}

struct StackFrame {
    register: VecDeque<EnvItem>,
    instructions: VecDeque<Instruction>,
    ret: (usize, usize),
}

impl StackFrame {
    fn new(stack_pointer: usize, register_pointer: usize) -> StackFrame {
        StackFrame {
            ret: (stack_pointer, register_pointer),
            ..Default::default()
        }
    }
}

impl Default for StackFrame {
    fn default() -> Self {
        StackFrame {
            register: VecDeque::new(),
            instructions: VecDeque::new(),
            ret: (0, 0),
        }
    }
}

//impl Deref for StackFrame {
//    type Target = Vec<Item>;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//
//impl DerefMut for StackFrame {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.0
//    }
//}

pub struct Machine {
    // For future reference:
    // The environment stack could be moved to the stackframe
    env: Environment,
    stack: Vec<StackFrame>,
}

type VMOutput<T = ()> = Result<T, error::VMError>;

impl Machine {
    #[allow(dead_code)]
    fn run_instruction(&mut self) -> VMOutput {
        match self.stack.last_mut() {
            Some(sf) => match sf.instructions.pop_front() {
                Some(i) => match i {
                    Instruction::Eval(i) => self.eval(i),
                    Instruction::EvalFunction(i) => self.eval_function(i),
                    Instruction::Pop => {
                        self.pop_stack_frame()?;
                        Ok(())
                    }
                    Instruction::Call => self.call_function(),
                    Instruction::CrashIfNotFunction => self.crash_if_not_function(),
                },
                None => Err(VMError::error("No more instructions")),
            },
            None => Err(VMError::error("No more stack frames")),
        }
    }

    fn push_instruction(&mut self, instruction: Instruction) -> VMOutput {
        match self.stack.last_mut() {
            Some(sf) => {
                sf.instructions.push_back(instruction);
                Ok(())
            }
            None => Err(VMError::error("No stack frame exists")),
        }
    }

    fn push_register(&mut self, item: EnvItem) -> VMOutput {
        match self.stack.last_mut() {
            Some(sf) => {
                sf.register.push_back(item);
                Ok(())
            }
            None => Err(VMError::error("No stack frame exists")),
        }
    }

    fn pop_register(&mut self) -> VMOutput<EnvItem> {
        match self.stack.last_mut() {
            Some(sf) => match sf.register.pop_front() {
                Some(r) => Ok(r),
                None => Ok(EnvItem::None),
            },
            None => Err(VMError::error("No stack frame exists")),
        }
    }

    fn get_current_stack_frame(&self) -> VMOutput<&StackFrame> {
        match self.stack.last() {
            Some(sf) => Ok(sf),
            None => Err(VMError::no_stack_frame()),
        }
    }

    #[allow(dead_code)]
    fn get_current_stack_frame_mut(&mut self) -> VMOutput<&mut StackFrame> {
        match self.stack.last_mut() {
            Some(sf) => Ok(sf),
            None => Err(VMError::no_stack_frame()),
        }
    }

    fn push_stack_frame(&mut self) -> VMOutput {
        let sp = self.stack.len() - 1;
        let rp = self.get_current_stack_frame()?.register.len();
        self.push_register(EnvItem::None)?;
        self.stack.push(StackFrame::new(sp, rp));
        self.env.push();
        Ok(())
    }

    fn pop_stack_frame(&mut self) -> VMOutput<Option<StackFrame>> {
        self.env.pop();
        let res = self.stack.pop();
        if self.stack.is_empty() {
            self.push_stack_frame()?;
        }
        Ok(res)
    }

    fn get_current_register(&mut self) -> VMOutput<Vec<EnvItem>> {
        let mut sf = match self.pop_stack_frame()? {
            Some(sf) => sf,
            None => Err(VMError::no_stack_frame())?,
        };
        let register = sf.register.into();
        sf.register = VecDeque::new();
        self.stack.push(sf);
        Ok(register)
    }

    fn eval(&mut self, item: Item) -> VMOutput {
        match item {
            Item::Cons(list) => self.eval_list(list),
            Item::Name(n) => self.eval_name(n),
            item => self.push_register(EnvItem::Data(item)),
        }
    }

    fn eval_list(&mut self, list: Cons) -> VMOutput {
        self.push_stack_frame()?;
        self.push_instruction(Instruction::EvalFunction(list.car().clone()))?;
        self.push_instruction(Instruction::CrashIfNotFunction)?;
        for item in list.iter().skip(1) {
            self.push_instruction(Instruction::Eval(item.clone()))?
        }
        self.push_instruction(Instruction::Call)?;
        self.push_instruction(Instruction::Pop)?;
        Ok(())
    }

    fn eval_name(&mut self, name: String) -> VMOutput {
        match self.env.lookup(&name) {
            EnvItem::Data(item) => self.push_register(EnvItem::Data(item)),
            EnvItem::Function(func) => self.push_register(EnvItem::Function(func)),
            EnvItem::None => Err(VMError::error(format!("'{}' not found", name))),
        }
    }

    fn eval_function(&mut self, item: Item) -> VMOutput {
        match item {
            Item::Cons(list) => self.eval_list(list),
            Item::Name(n) => self.eval_function_name(n),
            item => Err(VMError::error(format!("'{}' is not a function", item))),
        }
    }

    fn eval_function_name(&mut self, name: String) -> VMOutput {
        match self.env.lookup(&name) {
            EnvItem::Function(func) => self.push_register(EnvItem::Function(func)),
            EnvItem::Data(_) => Err(VMError::error(format!(
                "'{}' is not bound to a function",
                name
            ))),
            EnvItem::None => Err(VMError::error(format!("'{}' not found", name))),
        }
    }

    fn crash_if_not_function(&self) -> VMOutput {
        match self.stack.last() {
            Some(sf) => match sf.register.front() {
                Some(EnvItem::Function(_)) => Ok(()),
                _ => Err(VMError::not_a_function()),
            },
            None => Err(VMError::no_stack_frame()),
        }
    }

    fn call_function(&mut self) -> VMOutput {
        let func = match self.pop_register()? {
            EnvItem::Function(f) => f,
            _ => Err(VMError::not_a_function())?,
        };
        let args = self.get_current_register()?;

        match func(self, args) {
            Ok(output) => {
                let sf = self.get_current_stack_frame_mut()?;
                let (sp, rp) = sf.ret;
                self.stack[sp].register[rp] = output;
                Ok(())
            }
            Err(e) => Err(VMError::eval_error(e)),
        }
    }

    pub fn call(&mut self, func: EnvItemFunctionWrapped) -> VMOutput {
        self.push_stack_frame()?;
        self.push_register(EnvItem::Function(func))?;
        Ok(())
    }
}

impl Default for Machine {
    fn default() -> Machine {
        Machine {
            env: Environment::default(),
            stack: vec![StackFrame::default()],
        }
    }
}
