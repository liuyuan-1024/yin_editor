use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

// Cell 细胞，我称为图元，在视觉上是一个字符，但其宽度是不固定的。在unicode-segmentation 中称为 grapheme 字素。
#[derive(Clone)]
pub struct Cell {
    // 图元的内容，终端将其渲染为视觉上的一个字符。
    content: String,
    // 图元的渲染时宽度，用于终端渲染时确定宽度。
    cell_width: usize,
}

impl Cell {
    pub fn to_string(&self) -> &String {
        &self.content
    }

    pub fn get_cell_width(&self) -> usize {
        self.cell_width
    }

    pub fn char_to_cell(char: char) -> Cell {
        Self::grapheme_to_cell(&char.to_string())
    }

    // 将字符串转为图元向量, 还是有一些表情不支持。
    pub fn str_to_cells(str: &str) -> Vec<Cell> {
        let mut result = Vec::new();

        str.graphemes(true).for_each(|grapheme| {
            // 对每一个 grapheme，都要替换成我们的 Cell 实例
            result.push(Self::grapheme_to_cell(grapheme));
        });

        return result;
    }

    fn grapheme_to_cell(grapheme: &str) -> Cell {
        match grapheme {
            " " => Cell {
                content: "␣".to_string(),
                cell_width: 1,
            },
            "\t" => Cell {
                content: "␣␣␣␣".to_string(),
                cell_width: 4,
            },
            _ => Cell {
                content: grapheme.to_string(),
                cell_width: grapheme.width(),
            },
        }
    }
}
