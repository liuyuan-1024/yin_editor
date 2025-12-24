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

    pub fn cell_width(&self) -> usize {
        self.cell_width
    }

    /// 保存文档时，将图元转为对应的字符串
    pub fn to_grapheme(&self) -> &str {
        match self.content.as_str() {
            "␣" => " ",
            "␣␣␣␣" => "\t",
            _ => &self.content,
        }
    }

    pub fn char_to_cell(char: char) -> Cell {
        Self::from_grapheme(&char.to_string())
    }

    // 将字符串转为图元向量, 还是有一些表情不支持。
    pub fn str_to_cells(str: &str) -> Vec<Cell> {
        let mut result = Vec::new();

        str.graphemes(true).for_each(|grapheme| {
            // 对每一个 grapheme，都要替换成我们的 Cell 实例
            result.push(Self::from_grapheme(grapheme));
        });

        return result;
    }

    fn from_grapheme(grapheme: &str) -> Cell {
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
