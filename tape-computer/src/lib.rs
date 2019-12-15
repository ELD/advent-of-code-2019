#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save(Mode),
    Output(Mode),
    JIT(Mode, Mode),
    JIF(Mode, Mode),
    LT(Mode, Mode, Mode),
    EQ(Mode, Mode, Mode),
    AdjustRelative(Mode),
    Halt,
}

impl From<i64> for Opcode {
    fn from(input: i64) -> Self {
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
            9 => Opcode::AdjustRelative(param1_mode),
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
            Opcode::AdjustRelative(_) => 2,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Mode {
    fn from(input: i64) -> Self {
        match input {
            0 => Mode::Position,
            2 => Mode::Relative,
            _ => Mode::Immediate,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Amp {
    pc: usize,
    relative: i64,
    output: Option<i64>,
    mem: Vec<i64>,
}

impl Amp {
    pub fn new(mem: Vec<i64>) -> Self {
        Self {
            pc: 0,
            relative: 0,
            output: None,
            mem,
        }
    }

    pub fn run(&mut self, mut input: Option<i64>, signal: i64) -> Option<i64> {
        loop {
            let opcode = Opcode::from(self.mem[self.pc]);

            match opcode {
                Opcode::Add(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1, self.relative),
                        get_value(&self.mem, self.pc + 2, m2, self.relative),
                    );

                    set_value(
                        &mut self.mem,
                        self.pc + 3,
                        m3,
                        param1 + param2,
                        self.relative,
                    );
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Mul(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1, self.relative),
                        get_value(&self.mem, self.pc + 2, m2, self.relative),
                    );

                    set_value(
                        &mut self.mem,
                        self.pc + 3,
                        m3,
                        param1 * param2,
                        self.relative,
                    );
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Save(m1) => {
                    if let Some(input) = input.take() {
                        set_value(&mut self.mem, &self.pc + 1, m1, input, self.relative);
                    } else {
                        set_value(&mut self.mem, &self.pc + 1, m1, signal, self.relative);
                    }
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Output(m1) => {
                    let param1 = get_value(&self.mem, self.pc + 1, m1, self.relative);
                    self.output = Some(param1);
                    self.pc += Into::<usize>::into(opcode);
                    break;
                }
                Opcode::JIT(m1, m2) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1, self.relative),
                        get_value(&self.mem, self.pc + 2, m2, self.relative),
                    );

                    if param1 != 0 {
                        self.pc = param2 as usize;
                    } else {
                        self.pc += Into::<usize>::into(opcode);
                    }
                }
                Opcode::JIF(m1, m2) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1, self.relative),
                        get_value(&self.mem, self.pc + 2, m2, self.relative),
                    );

                    if param1 == 0 {
                        self.pc = param2 as usize;
                    } else {
                        self.pc += Into::<usize>::into(opcode);
                    }
                }
                Opcode::LT(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1, self.relative),
                        get_value(&self.mem, self.pc + 2, m2, self.relative),
                    );

                    if param1 < param2 {
                        set_value(&mut self.mem, self.pc + 3, m3, 1, self.relative);
                    } else {
                        set_value(&mut self.mem, self.pc + 3, m3, 0, self.relative);
                    }

                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::EQ(m1, m2, m3) => {
                    let (param1, param2) = (
                        get_value(&self.mem, self.pc + 1, m1, self.relative),
                        get_value(&self.mem, self.pc + 2, m2, self.relative),
                    );

                    if param1 == param2 {
                        set_value(&mut self.mem, self.pc + 3, m3, 1, self.relative);
                    } else {
                        set_value(&mut self.mem, self.pc + 3, m3, 0, self.relative);
                    }
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::AdjustRelative(m1) => {
                    let param1 = get_value(&self.mem, self.pc + 1, m1, self.relative);

                    self.relative += param1;
                    self.pc += Into::<usize>::into(opcode);
                }
                Opcode::Halt => {
                    self.output = None;
                    break;
                }
            }
        }

        self.output
    }
}

pub fn get_value(mem: &[i64], addr: usize, mode: Mode, relative: i64) -> i64 {
    match mode {
        Mode::Immediate => mem[addr],
        Mode::Position => mem[mem[addr] as usize],
        Mode::Relative => mem[(mem[addr] + relative) as usize],
    }
}

pub fn set_value(mem: &mut [i64], addr: usize, mode: Mode, value: i64, relative: i64) {
    match mode {
        Mode::Immediate => mem[addr] = value,
        Mode::Position => mem[mem[addr] as usize] = value,
        Mode::Relative => mem[(mem[addr] + relative) as usize] = value,
    };
}
