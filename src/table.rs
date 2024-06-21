use core::fmt;

pub struct TableRow<'a> {
    serial_number: usize,
    cells: &'a [String],
}

impl<'a> TableRow<'a> {
    pub fn new(serial_number: usize, cells: &'a [String]) -> Self {
        TableRow {serial_number, cells}
    }
}

impl<'a> fmt::Display for TableRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[37;1m{:width$}\x1B[0m ", self.serial_number, width = 5)?;

        for cell in self.cells {
            write!(f, "\x1B[32m{:width$}\x1B[0m", cell, width = 10)?;
        }

        writeln!(f)
    }
}