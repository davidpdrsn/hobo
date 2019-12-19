css_macros::easy_enum!{border-collapse separate collapse}
css_macros::easy_enum!{box-decoration-break slice clone unset}
css_macros::easy_enum!{outline-width medium thin thick @}
css_macros::easy_enum!{outline-style none hidden dotted dashed solid double groove ridge inset outset}
css_macros::easy_enum!{outline-offset @}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum OutlineColor {
	Invert,
	Rgba(u8, u8, u8, u8),
	Initial,
	Inherit,
}

impl ToString for OutlineColor {
	fn to_string(&self) -> String {
		match self {
			Self::Invert           => "outline-color:invert;".to_owned(),
			Self::Rgba(r, g, b, a) => format!("outline-color:#{:02x}{:02x}{:02x}{:02x};", r, g, b, a),
			Self::Initial          => "outline-color:initial;".to_owned(),
			Self::Inherit          => "outline-color:inherit;".to_owned(),
		}
	}
}

#[macro_export]
macro_rules! outline_color {
	(invert)                  => {$crate::Property::OutlineColor($crate::OutlineColor::Invert)};
	(initial)                 => {$crate::Property::OutlineColor($crate::OutlineColor::Initial)};
	(inherit)                 => {$crate::Property::OutlineColor($crate::OutlineColor::Inherit)};
	($rgb:tt)                 => {$crate::Property::OutlineColor($crate::OutlineColor::Rgba(($rgb, $rgb, $rgb, 255)))};
	($r:tt $g:tt $b:tt $a:tt) => {$crate::Property::OutlineColor($crate::OutlineColor::Rgba(($r, $g, $b, $a)))};
	($r:tt $g:tt $b:tt)       => {$crate::Property::OutlineColor($crate::OutlineColor::Rgba(($r, $g, $b, 255)))};
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BorderColor {
	Transparent,
	Rgba(u8, u8, u8, u8),
	Initial,
	Inherit,
}

impl ToString for BorderColor {
	fn to_string(&self) -> String {
		match self {
			Self::Transparent        => "transparent".to_owned(),
			Self::Rgba(r, g, b, a)   => format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
			Self::Initial            => "initial".to_owned(),
			Self::Inherit            => "inherit".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BorderStyle {
	None,
	Hidden,
	Dotted,
	Dashed,
	Solid,
	Double,
	Groove,
	Ridge,
	Inset,
	Outset,
	Initial,
	Inherit,
}

impl ToString for BorderStyle {
	fn to_string(&self) -> String {
		match self {
			Self::None    => "none".to_owned(),
			Self::Hidden  => "hidden".to_owned(),
			Self::Dotted  => "dotted".to_owned(),
			Self::Dashed  => "dashed".to_owned(),
			Self::Solid   => "solid".to_owned(),
			Self::Double  => "double".to_owned(),
			Self::Groove  => "groove".to_owned(),
			Self::Ridge   => "ridge".to_owned(),
			Self::Inset   => "inset".to_owned(),
			Self::Outset  => "outset".to_owned(),
			Self::Initial => "initial".to_owned(),
			Self::Inherit => "inherit".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BorderWidth {
	Medium,
	Thin,
	Thick,
	Zero,
	Some(crate::Unit),
	Initial,
	Inherit,
}

impl ToString for BorderWidth {
	fn to_string(&self) -> String {
		match self {
			Self::Medium  => "medium".to_owned(),
			Self::Thin    => "thin".to_owned(),
			Self::Thick   => "thick".to_owned(),
			Self::Zero    => "0".to_owned(),
			Self::Some(x) => x.to_string(),
			Self::Initial => "initial".to_owned(),
			Self::Inherit => "inherit".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BorderRadius {
	Zero,
	Some(crate::Unit),
	Initial,
	Inherit,
}

impl ToString for BorderRadius {
	fn to_string(&self) -> String {
		match self {
			Self::Zero    => "0".to_owned(),
			Self::Some(x) => x.to_string(),
			Self::Initial => "initial".to_owned(),
			Self::Inherit => "inherit".to_owned(),
		}
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __border_width {
	($side:ident, medium)      => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Medium)}};
	($side:ident, thin)        => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Thin)}};
	($side:ident, thick)       => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Thick)}};
	($side:ident, initial)     => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Initial)}};
	($side:ident, inherit)     => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Inherit)}};
	($side:ident, 0)           => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Zero)}};
	($side:ident, $($val:tt)+) => {$crate::paste::expr!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Some($crate::unit!($($val)+)))}};
}

#[macro_export] macro_rules! border_left_width {($($tt:tt)+)   => {$crate::__border_width!(Left, $($tt)+)}}
#[macro_export] macro_rules! border_right_width {($($tt:tt)+)  => {$crate::__border_width!(Right, $($tt)+)}}
#[macro_export] macro_rules! border_top_width {($($tt:tt)+)    => {$crate::__border_width!(Top, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_width {($($tt:tt)+) => {$crate::__border_width!(Bottom, $($tt)+)}}

#[macro_export]
#[doc(hidden)]
macro_rules! __border_style {
	($side:ident, none)    => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::None)}};
	($side:ident, hidden)  => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Hidden)}};
	($side:ident, dotted)  => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Dotted)}};
	($side:ident, dashed)  => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Dashed)}};
	($side:ident, solid)   => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Solid)}};
	($side:ident, double)  => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Double)}};
	($side:ident, groove)  => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Groove)}};
	($side:ident, ridge)   => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Ridge)}};
	($side:ident, inset)   => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Inset)}};
	($side:ident, outset)  => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Outset)}};
	($side:ident, initial) => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Initial)}};
	($side:ident, inherit) => {$crate::paste::expr!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Inherit)}};
}

#[macro_export] macro_rules! border_left_style {($($tt:tt)+)   => {$crate::__border_style!(Left, $($tt)+)}}
#[macro_export] macro_rules! border_right_style {($($tt:tt)+)  => {$crate::__border_style!(Right, $($tt)+)}}
#[macro_export] macro_rules! border_top_style {($($tt:tt)+)    => {$crate::__border_style!(Top, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_style {($($tt:tt)+) => {$crate::__border_style!(Bottom, $($tt)+)}}

#[macro_export]
#[doc(hidden)]
macro_rules! __border_color {
	($side:ident, transparent)             => {$crate::paste::expr!{$crate::Property::[<Border $side Color>]($crate::BorderColor::Transparent)}};
	($side:ident, initial)                 => {$crate::paste::expr!{$crate::Property::[<Border $side Color>]($crate::BorderColor::Initial)}};
	($side:ident, inherit)                 => {$crate::paste::expr!{$crate::Property::[<Border $side Color>]($crate::BorderColor::Inherit)}};
	($side:ident, $r:tt $g:tt $b:tt $a:tt) => {$crate::paste::expr!{$crate::Property::[<Border $side Color>]($crate::BorderColor::Rgba($r, $g, $b, $a))}};
	($side:ident, $r:tt $g:tt $b:tt)       => {$crate::paste::expr!{$crate::Property::[<Border $side Color>]($crate::BorderColor::Rgba($r, $g, $b, 255))}};
	($side:ident, $rgb:tt)                 => {$crate::paste::expr!{$crate::Property::[<Border $side Color>]($crate::BorderColor::Rgba($rgb, $rgb, $rgb, 255))}};
}

#[macro_export] macro_rules! border_left_color {($($tt:tt)+)   => {$crate::__border_color!(Left, $($tt)+)}}
#[macro_export] macro_rules! border_right_color {($($tt:tt)+)  => {$crate::__border_color!(Right, $($tt)+)}}
#[macro_export] macro_rules! border_top_color {($($tt:tt)+)    => {$crate::__border_color!(Top, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_color {($($tt:tt)+) => {$crate::__border_color!(Bottom, $($tt)+)}}

#[macro_export]
#[doc(hidden)]
macro_rules! __border_radius {
	($side:ident, initial)     => {$crate::paste::expr!{$crate::Property::[<Border $side Radius>]($crate::BorderRadius::Initial)}};
	($side:ident, inherit)     => {$crate::paste::expr!{$crate::Property::[<Border $side Radius>]($crate::BorderRadius::Inherit)}};
	($side:ident, 0)           => {$crate::paste::expr!{$crate::Property::[<Border $side Radius>]($crate::BorderRadius::Zero)}};
	($side:ident, $($val:tt)+) => {$crate::paste::expr!{$crate::Property::[<Border $side Radius>]($crate::BorderRadius::Some($crate::unit!($($val)+)))}};
}

#[macro_export] macro_rules! border_top_left_radius {($($tt:tt)+)     => {$crate::__border_radius!(TopLeft, $($tt)+)}}
#[macro_export] macro_rules! border_top_right_radius {($($tt:tt)+)    => {$crate::__border_radius!(TopRight, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_left_radius {($($tt:tt)+)  => {$crate::__border_radius!(BottomLeft, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_right_radius {($($tt:tt)+) => {$crate::__border_radius!(BottomRight, $($tt)+)}}

#[macro_export] macro_rules! border_radius {($($tt:tt)+) => {
	vec![
		$crate::border_top_left_radius!($($tt)+),
		$crate::border_top_right_radius!($($tt)+),
		$crate::border_bottom_left_radius!($($tt)+),
		$crate::border_bottom_right_radius!($($tt)+),
	]
}}

#[macro_export] macro_rules! border_width {($($tt:tt)+) => {
	vec![
		$crate::border_left_width!($($tt)+),
		$crate::border_right_width!($($tt)+),
		$crate::border_top_width!($($tt)+),
		$crate::border_bottom_width!($($tt)+),
	]
}}

#[macro_export] macro_rules! border_style {($($tt:tt)+) => {
	vec![
		$crate::border_left_style!($($tt)+),
		$crate::border_right_style!($($tt)+),
		$crate::border_top_style!($($tt)+),
		$crate::border_bottom_style!($($tt)+),
	]
}}

#[macro_export] macro_rules! border_color {($($tt:tt)+) => {
	vec![
		$crate::border_left_color!($($tt)+),
		$crate::border_right_color!($($tt)+),
		$crate::border_top_color!($($tt)+),
		$crate::border_bottom_color!($($tt)+),
	]
}}