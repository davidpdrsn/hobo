pub type F32 = ordered_float::NotNan<f32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Unit {
	Px(F32),
	Em(F32),
	Rem(F32),
	Vw(F32),
	Vh(F32),
	Vmin(F32),
	Vmax(F32),
	Percent(F32),
	// TODO: calc?
}

impl ToString for Unit {
	fn to_string(&self) -> String {
		match self {
			Self::Px(x)      => format!("{}px", x),
			Self::Em(x)      => format!("{}em", x),
			Self::Rem(x)     => format!("{}rem", x),
			Self::Vw(x)      => format!("{}vw", x),
			Self::Vh(x)      => format!("{}vh", x),
			Self::Vmin(x)    => format!("{}vmin", x),
			Self::Vmax(x)    => format!("{}vmax", x),
			Self::Percent(x) => format!("{}%", x),
		}
	}
}

macro_rules! generate_units {
	($($small:ident, $big:ident);+$(;)*) => {
		$(
			#[macro_export]
			macro_rules! $small {
				($e:expr) => { $crate::Unit::$big(unsafe { $crate::units::F32::unchecked_new($e as _) }) };
			}
		)+
	};
}

generate_units!{
	px, Px;
	em, Em;
	rem, Rem;
	vw, Vw;
	vh, Vh;
	vmin, Vmin;
	vmax, Vmax;
	pct, Percent;
}

#[macro_export]
macro_rules! unit {
	(expr = ($($e:tt)+))   => { $crate::Unit::Px(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) px)   => { $crate::Unit::Px(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) em)   => { $crate::Unit::Em(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) rem)  => { $crate::Unit::Rem(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vw)   => { $crate::Unit::Vw(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vh)   => { $crate::Unit::Vh(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vmin) => { $crate::Unit::Vmin(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vmax) => { $crate::Unit::Vmax(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) %)    => { $crate::Unit::Percent(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)*) $tt:tt $($rest:tt)*) => { $crate::unit!(expr = ($($e)* $tt) $($rest)*) };
	($head:tt $($rest:tt)*) => { $crate::unit!(expr = ($head) $($rest)*) };
}