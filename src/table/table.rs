use crate::table::Column;

const END_BORDER: &str = "| ";
const COL_DIVIDER: &str = " | ";

pub struct Table {
    columns: Vec<Column>,
    width: usize,
}

impl Table {
    /// Creates a new table. At least one column must be provided.
    pub fn new(columns: Vec<Column>) -> Table {
        let component_count = columns[0].data().len();
        let index_col =
            Column::from_strings("#", (1..=component_count).map(|x| x.to_string()).collect());
        let mut columns = columns;
        columns.insert(0, index_col);

        let mut table = Table { columns, width: 0 };
        table.width = table.columns.iter().map(|c| c.width()).sum::<usize>()
            + END_BORDER.len() * 2
            + COL_DIVIDER.len() * (table.columns.len() - 1);
        table
    }

    /// Returns a formatted string representation.
    pub fn draw(&self) -> String {
        let mut sb = String::new();

        self.draw_row_divider(&mut sb);
        self.draw_header(&mut sb);
        self.draw_row_divider(&mut sb);
        for i in 0..(self.columns[0].data().len()) {
            self.draw_row(&mut sb, i);
        }
        self.draw_row_divider(&mut sb);

        sb
    }

    fn draw_row_divider(&self, sb: &mut String) {
        sb.push_str(&"-".repeat(self.width));
        sb.push('\n');
    }

    fn draw_header(&self, sb: &mut String) {
        sb.push_str(END_BORDER);
        for col in self.columns.iter() {
            sb.push_str(&format!("{:>width$}", col.name(), width = col.width()));
            sb.push_str(COL_DIVIDER);
        }
        sb.push('\n');
    }

    fn draw_row(&self, sb: &mut String, row: usize) {
        sb.push_str(END_BORDER);
        for col in self.columns.iter() {
            sb.push_str(&format!("{:>width$}", col.data()[row], width = col.width()));
            sb.push_str(COL_DIVIDER);
        }
        sb.push('\n');
    }
}
