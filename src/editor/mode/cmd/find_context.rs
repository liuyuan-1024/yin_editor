use crate::prelude::DocumentCoordinate;

/// 查找模式的上下文
pub struct FindContext {
    // 目标字符串
    query: String,
    // 结果字符串的文档位置向量
    results: Vec<DocumentCoordinate>,
}

impl FindContext {
    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }
}

impl Default for FindContext {
    fn default() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
        }
    }
}
