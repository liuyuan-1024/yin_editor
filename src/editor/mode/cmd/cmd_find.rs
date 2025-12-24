pub enum Find {
    
}

impl Find {
    fn search_in_direction(&mut self, from: Location, direction: SearchDirection) {
        if let Some(location) = self.get_search_query().and_then(|query| {
            if query.is_empty() {
                None
            } else if direction == SearchDirection::Forward {
                self.buffer.search_forward(query, from)
            } else {
                self.buffer.search_backward(query, from)
            }
        }) {
            self.text_location = location;
            self.center_text_location();
        };
        self.set_needs_redraw(true);
    }

    // 在当前位置的左侧寻找下一个出现的位置
    pub fn search_next(&mut self) {
        let step_right = self
            .get_search_query()
            .map_or(1, |query| min(query.cells_count(), 1));
        let location = Location {
            line_idx: self.text_location.line_idx,

            cell_idx: self.text_location.cell_idx.saturating_add(step_right),
        };

        self.search_in_direction(location, SearchDirection::Forward);
    }

    pub fn search_prev(&mut self) {
        self.search_in_direction(self.text_location, SearchDirection::Backward);
    }


    /// 执行命令

}
