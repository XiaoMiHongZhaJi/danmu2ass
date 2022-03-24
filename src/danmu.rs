//! 一个弹幕实例，但是没有位置信息

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DanmuType {
    Float,
    Top,
    Bottom,
    Reverse,
}
impl Default for DanmuType {
    fn default() -> Self {
        DanmuType::Float
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Danmu {
    pub timeline_s: f64,
    pub content: String,
    pub r#type: DanmuType,
    pub fontsize: u32,
    pub rgb: (u8, u8, u8),
}

impl Danmu {
    /// 像素长度
    pub fn length(&self) -> u32 {
        // 汉字算一个全宽，英文算2/3宽
        self.fontsize
            * self
                .content
                .chars()
                .map(|ch| if ch.is_ascii() { 2 } else { 3 })
                .sum::<u32>()
            / 3
    }
}
