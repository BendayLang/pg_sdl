#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HAlign {
	Left,
	Center,
	Right,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VAlign {
	Top,
	Center,
	Bottom,
}

#[derive(Clone, Copy)]
pub enum Align {
	TopLeft,
	Top,
	TopRight,
	Left,
	Center,
	Right,
	BottomLeft,
	Bottom,
	BottomRight,
}
impl Align {
	pub fn from_align(h_align: HAlign, v_align: VAlign) -> Self {
		match (h_align, v_align) {
			(HAlign::Left, VAlign::Top) => Self::TopLeft,
			(HAlign::Left, VAlign::Center) => Self::Left,
			(HAlign::Left, VAlign::Bottom) => Self::BottomLeft,
			(HAlign::Center, VAlign::Top) => Self::Top,
			(HAlign::Center, VAlign::Center) => Self::Center,
			(HAlign::Center, VAlign::Bottom) => Self::Bottom,
			(HAlign::Right, VAlign::Top) => Self::TopRight,
			(HAlign::Right, VAlign::Center) => Self::Right,
			(HAlign::Right, VAlign::Bottom) => Self::BottomRight,
		}
	}
	pub fn opposite(&self) -> Self {
		match self {
			Self::TopLeft => Self::BottomRight,
			Self::Top => Self::Bottom,
			Self::TopRight => Self::BottomLeft,
			Self::Left => Self::Right,
			Self::Center => Self::Center,
			Self::Right => Self::Left,
			Self::BottomLeft => Self::TopRight,
			Self::Bottom => Self::Top,
			Self::BottomRight => Self::TopLeft,
		}
	}
}
