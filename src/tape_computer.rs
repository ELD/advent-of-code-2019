#[derive(Copy, Clone, Debug)]
pub(crate) enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save(Mode),
    Output(Mode),
    JIT(Mode, Mode),
    JIF(Mode, Mode),
    LT(Mode, Mode, Mode),
    EQ(Mode, Mode, Mode),
    Halt,
}

impl From<i32> for Opcode {
    fn from(input: i32) -> Self {
        let param1_mode = Mode::from((input / 100) % 10);
        let param2_mode = Mode::from((input / 1000) % 10);
        let param3_mode = Mode::from((input / 10000) % 10);

        match input % 100 {
            1 => Opcode::Add(param1_mode, param2_mode, param3_mode),
            2 => Opcode::Mul(param1_mode, param2_mode, param3_mode),
            3 => Opcode::Save(param1_mode),
            4 => Opcode::Output(param1_mode),
            5 => Opcode::JIT(param1_mode, param2_mode),
            6 => Opcode::JIF(param1_mode, param2_mode),
            7 => Opcode::LT(param1_mode, param2_mode, param3_mode),
            8 => Opcode::EQ(param1_mode, param2_mode, param3_mode),
            _ => Opcode::Halt,
        }
    }
}

impl Into<usize> for Opcode {
    fn into(self) -> usize {
        match self {
            Opcode::Add(_, _, _) => 4,
            Opcode::Mul(_, _, _) => 4,
            Opcode::Save(_) => 2,
            Opcode::Output(_) => 2,
            Opcode::JIT(_, _) => 3,
            Opcode::JIF(_, _) => 3,
            Opcode::LT(_, _, _) => 4,
            Opcode::EQ(_, _, _) => 4,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(input: i32) -> Self {
        match input {
            0 => Mode::Position,
            _ => Mode::Immediate,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Amp {
    pc: usize,
    mem: Vec<i32>,
}

impl Amp {
    pub(crate) fn new(mem: Vec<i32>) -> Self {
        Self { pc: 0, mem }
    }

    pub(crate) fn run(&mut self, mut input: Option<i32>, signal: i32) -> Option<i32> {
        loop {
            let opcode = Opcode::from(self.mem[self.pc]);

            match opcode {
                Opcode::Add(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1),
                        get_value(&self.mem, self.pc + 2, m2),
                    );

                    set_value(&mut self.mem, self.pc + 3, m3, param1 + param2);
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Mul(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1),
                        get_value(&self.mem, self.pc + 2, m2),
                    );

                    set_value(&mut self.mem, self.pc + 3, m3, param1 * param2);
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Save(m1) => {
                    if let Some(input) = input.take() {
                        set_value(&mut self.mem, &self.pc + 1, m1, input);
                    } else {
                        set_value(&mut self.mem, &self.pc + 1, m1, signal);
                    }
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Output(m1) => {
                    let param1 = get_value(&self.mem, self.pc + 1, m1);
                    self.pc += Into::<usize>::into(opcode);
                    return Some(param1);
                }
                Opcode::JIT(m1, m2) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1),
                        get_value(&self.mem, self.pc + 2, m2),
                    );

                    if param1 != 0 {
                        self.pc = param2 as usize;
                    } else {
                        self.pc += Into::<usize>::into(opcode);
                    }
                }
                Opcode::JIF(m1, m2) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1),
                        get_value(&self.mem, self.pc + 2, m2),
                    );

                    if param1 == 0 {
                        self.pc = param2 as usize;
                    } else {
                        self.pc += Into::<usize>::into(opcode);
                    }
                }
                Opcode::LT(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1),
                        get_value(&self.mem, self.pc + 2, m2),
                    );

                    if param1 < param2 {
                        set_value(&mut self.mem, self.pc + 3, m3, 1);
                    } else {
                        set_value(&mut self.mem, self.pc + 3, m3, 0);
                    }

                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::EQ(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1),
                        get_value(&self.mem, self.pc + 2, m2),
                    );

                    if param1 == param2 {
                        set_value(&mut self.mem, self.pc + 3, m3, 1);
                    } else {
                        set_value(&mut self.mem, self.pc + 3, m3, 0);
                    }
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Halt => break,
            }
        }

        None
    }
}

pub(crate) fn get_value(mem: &[i32], addr: usize, mode: Mode) -> i32 {
    match mode {
        Mode::Immediate => mem[addr],
        Mode::Position => mem[mem[addr] as usize],
    }
}

pub(crate) fn set_value(mem: &mut [i32], addr: usize, mode: Mode, value: i32) {
    match mode {
        Mode::Immediate => mem[addr] = value,
        Mode::Position => mem[mem[addr] as usize] = value,
    };
}
