use super::datastructure::{Cons, EnvItem, EnvItemFunctionWrapped, Environment, Item};
use super::stdlib;
use std::collections::VecDeque;
use std::rc::Rc;

mod error;

pub use error::VMError;

#[derive(Clone, Debug)]
enum Instruction {
    CondEval(Item),
    ElseEval(Item),
    Eval(Item),
    EvalFunction(Item),
    EvalRegister(usize),
    Pop,
    Call,
    CrashIfNotFunction,
}

struct StackFrame {
    register: VecDeque<EnvItem>,
    instructions: VecDeque<Instruction>,
    ret: (usize, usize),
    condition_success: bool,
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
            condition_success: false,
        }
    }
}

pub struct Machine {
    // For future reference:
    // The environment stack could be moved to the stackframe
    env: Environment,
    stack: Vec<StackFrame>,
}

type VMOutput<T = ()> = Result<T, error::VMError>;

impl Machine {
    pub fn eval(&mut self, item: Item) -> VMOutput<EnvItem> {
        self.clear_stack()?;
        self.push_instruction(Instruction::Eval(item))?;
        while match self.stack.first() {
            Some(sf) => match sf.instructions.front() {
                Some(Instruction::Pop) => false,
                Some(_) => true,
                None => false,
            },
            None => Err(VMError::no_stack_frame())?,
        } || self.stack.len() > 1
        {
            #[cfg(feature = "vm-debug")]
            self.print_machine()?;

            self.run_instruction()?;
        }

        let mut reg = self.get_current_register()?;
        if reg.is_empty() || reg.len() > 1 {
            Err(VMError::error(format!(
                "Incorrect amount of values returned. Expected 1, found: {}",
                reg.len()
            )))
        } else {
            Ok(reg.remove(0))
        }
    }

    fn run_instruction(&mut self) -> VMOutput {
        match self.stack.last_mut() {
            Some(sf) => match sf.instructions.pop_front() {
                Some(i) => match i {
                    Instruction::CondEval(expression) => self.conditional_eval(expression),
                    Instruction::ElseEval(expression) => self.else_eval(expression),
                    Instruction::Eval(i) => self.eval_item(i),
                    Instruction::EvalFunction(i) => self.eval_function(i),
                    Instruction::EvalRegister(r) => self.eval_register(r),
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

    #[allow(dead_code)]
    fn push_register_front(&mut self, item: EnvItem) -> VMOutput {
        match self.stack.last_mut() {
            Some(sf) => {
                sf.register.push_front(item);
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

    fn pop_register_back(&mut self) -> VMOutput<EnvItem> {
        match self.stack.last_mut() {
            Some(sf) => match sf.register.pop_back() {
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

    fn get_current_stack_frame_mut(&mut self) -> VMOutput<&mut StackFrame> {
        match self.stack.last_mut() {
            Some(sf) => Ok(sf),
            None => Err(VMError::no_stack_frame()),
        }
    }

    fn push_stack_frame(&mut self) -> VMOutput {
        if !self.stack.is_empty() {
            //println!("{}", self.stack.len());
            let sp = self.stack.len() - 1;
            let rp = self.get_current_stack_frame()?.register.len();
            self.push_register(EnvItem::None)?;
            self.stack.push(StackFrame::new(sp, rp));
        } else {
            self.stack.push(StackFrame::default());
        }
        self.env.push();
        Ok(())
    }

    fn pop_stack_frame(&mut self) -> VMOutput<Option<StackFrame>> {
        //println!("{}", self.stack.len());
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

    fn get_register_item(&mut self, register: usize) -> VMOutput<Item> {
        match self
            .get_current_stack_frame_mut()?
            .register
            .remove(register)
        {
            Some(EnvItem::Data(d)) => Ok(d),
            _ => Err(VMError::error(format!(
                "Could not get the item in register: {}",
                register
            ))),
        }
    }

    fn conditional_eval(&mut self, expression: Item) -> VMOutput {
        let reg = self.pop_register_back()?;
        println!("{:?}", reg);
        let test = match reg {
            EnvItem::Data(d) => d,
            _ => Err(VMError::error(
                "Test of the conditional can not be converted to a bool",
            ))?,
        };

        if test.into() {
            self.get_current_stack_frame_mut()?.condition_success = true;
            self.eval_item(expression)
        } else {
            Ok(())
        }
    }

    fn else_eval(&mut self, expression: Item) -> VMOutput {
        if !self.get_current_stack_frame()?.condition_success {
            self.eval_item(expression)
        } else {
            Ok(())
        }
    }

    fn eval_register(&mut self, register: usize) -> VMOutput {
        let item = self.get_register_item(register)?;
        self.eval_item(item)
    }

    fn eval_item(&mut self, item: Item) -> VMOutput {
        match item {
            Item::Cons(list) => self.eval_list(list),
            Item::Name(n) => self.eval_name(n),
            item => self.push_register(EnvItem::Data(item)),
        }
    }

    fn eval_special_function(&mut self, name: &str, list: Cons) -> VMOutput<usize> {
        let mut skip = 1;

        match name {
            "def" | "define" => {
                self.push_register(EnvItem::Function(Rc::new(stdlib::def)))?;
                self.push_register(EnvItem::Data(list.cadr().clone()))?;
                skip = 2;
            }
            "quote" => {
                if list.len() != 2 {
                    return Err(VMError::error("Too many arguments to function"));
                }
                self.push_register(EnvItem::Function(Rc::new(stdlib::quote)))?;
                self.push_register(EnvItem::Data(list.cadr().clone()))?;
                skip = 2;
            }
            "if" => {
                if list.len() != 4 {
                    return Err(VMError::error(
                        "Incorrect amount of arguments for the 'if' function",
                    ));
                }
                self.push_register(EnvItem::Function(Rc::new(stdlib::quote)))?;
                self.push_instruction(Instruction::Eval(list.cadr().clone()))?;
                self.push_instruction(Instruction::CondEval(list.caddr().clone()))?;
                self.push_instruction(Instruction::ElseEval(list.cadddr().clone()))?;
            }
            "eval" => {
                self.push_register(EnvItem::Function(Rc::new(stdlib::quote)))?;
                self.push_instruction(Instruction::EvalRegister(1))?;
                self.push_instruction(Instruction::Call)?;
                self.push_instruction(Instruction::Pop)?;
                self.push_stack_frame()?;
                self.push_register(EnvItem::Function(Rc::new(stdlib::quote)))?;
            }
            _ => {
                self.push_instruction(Instruction::EvalFunction(Item::Name(name.into())))?;
            }
        };

        Ok(skip)
    }

    fn eval_list(&mut self, list: Cons) -> VMOutput {
        self.push_stack_frame()?;

        let skip;

        if let Item::Name(name) = list.car() {
            skip = self.eval_special_function(name.as_ref(), list.clone())?;
        } else {
            self.push_instruction(Instruction::EvalFunction(list.car().clone()))?;
            skip = 1;
        }

        self.push_instruction(Instruction::CrashIfNotFunction)?;
        for item in list.iter().skip(skip) {
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

    fn clear_stack(&mut self) -> VMOutput {
        self.stack = vec![StackFrame::default()];
        Ok(())
    }

    pub fn define<T>(&mut self, key: T, value: EnvItem) -> VMOutput
    where
        T: Into<String>,
    {
        self.env.define(key, value);
        Ok(())
    }

    #[cfg(feature = "vm-debug")]
    fn print_machine(&self) -> VMOutput {
        println!("Registers: {:?}", self.get_current_stack_frame()?.register);
        println!(
            "Instructions: {:?}",
            self.get_current_stack_frame()?.instructions
        );
        println!("Stack size: {}", self.stack.len());
        Ok(())
    }
}

impl Default for Machine {
    fn default() -> Machine {
        Machine {
            env: stdlib::stdlib(),
            stack: vec![StackFrame::default()],
        }
    }
}

#[allow(dead_code)]
fn negate(item: Item) -> Item {
    Item::Cons(Cons::new(Item::Name("not".into()), item))
}
//const SPECIAL_FUNCTIONS: [&str; 4] = ["def", "if", "quote", "lambda"];
