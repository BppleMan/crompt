pub struct Term {
    pub width: usize,
}

pub enum Block<const N: usize> {
    Paragraph(String),
    Table(Table<N>),
    Blank,
    Divider,
}

pub struct Table<const N: usize> {
    pub rows: Vec<Row<N>>,
}

pub struct Row<const N: usize> {
    pub cells: [String; N],
}
