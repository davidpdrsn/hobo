#![feature(proc_macro_hygiene, specialization)]

pub mod prelude;
pub mod web_str;
pub mod css;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
pub use hobo_derive::*;
pub use web_sys;
pub use paste;

thread_local! {
	static CONTEXT: Context = Default::default();
}

struct StyleStorage {
	element: web_sys::Element,
	map: RefCell<HashMap<css::Style, u64>>,
}

impl Default for StyleStorage {
	fn default() -> Self {
		let dom = web_sys::window().unwrap().document().unwrap();
		let element = dom.create_element(web_str::style()).unwrap();
		dom.head().unwrap().append_child(&element).unwrap();
		Self { element, map: RefCell::new(HashMap::new()) }
	}
}

impl StyleStorage {
	fn fetch(&self, style: &css::Style) -> String {
		if let Some(id) = self.map.borrow().get(style) { return format!("s{}", id) }
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		style.hash(&mut hasher);
		let id = hasher.finish();
		self.map.borrow_mut().insert(style.clone(), id);
		let class = format!("s{}", id);
		let mut style = style.clone();
		for rule in style.0.iter_mut() {
			for selector_component in (rule.0).0.iter_mut() {
				if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
					*selector_component = css::selector::SelectorComponent::Class(class.clone());
				}
			}
		}
		self.element.append_with_str_1(&style.to_string()).unwrap();
		class
	}
}

#[derive(Default)]
pub struct Context {
	style_storage: StyleStorage,
	// classes: RefCell<HashMap<u64, String>>,
}

macro_rules! generate_events {
	($($event_kind:path, $name:ident, $trait:ident, $f:ident);+$(;)*) => {paste::item!{
		pub trait EventTarget: Element {
			fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>>;
			$(
				fn $f(&self, f: impl FnMut($event_kind) + 'static) where Self: Sized {
					use event_raw_exts::*;

					let handler = self.element().$f(f);
					self.event_handlers().push(handler);
				}

				#[allow(clippy::missing_safety_doc)]
				unsafe fn [<unsafe_ $f>]<'a>(&'a self, f: impl FnMut($event_kind) + 'a) where Self: Sized {
					use event_raw_exts::*;

					let handler = self.element().[<unsafe_ $f>](f);
					self.event_handlers().push(handler);
				}
			)+
		}

		pub mod event_raw_exts {
			use super::*;

			$(
				#[extend::ext(pub, name = [<Raw $trait>])]
				impl web_sys::EventTarget {
					fn $f(&self, f: impl FnMut($event_kind) + 'static) -> EventHandler where Self: Sized {
						let fbox: Box<dyn FnMut($event_kind) + 'static> = Box::new(f);
						let handler = Closure::wrap(fbox);
						self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
						EventHandler(Box::new(handler))
					}

					#[allow(clippy::missing_safety_doc)]
					unsafe fn [<unsafe_ $f>]<'a>(&'a self, f: impl FnMut($event_kind) + 'a) -> EventHandler where Self: Sized {
						let fbox: Box<dyn FnMut($event_kind) + 'a> = Box::new(f);
						let long_fbox: Box<dyn FnMut($event_kind) + 'static> = unsafe { std::mem::transmute(fbox) };
						let handler = Closure::wrap(long_fbox);
						self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
						EventHandler(Box::new(handler))
					}
				}
			)+
		}
	}};
}

pub struct EventHandler(Box<dyn std::any::Any>);
pub type EventHandlers = RefCell<Vec<EventHandler>>;

generate_events!{
	web_sys::MouseEvent,    click,       OnClick,       on_click;
	web_sys::MouseEvent,    contextmenu, OnContextMenu, on_context_menu;
	web_sys::MouseEvent,    dblclick,    OnDblClick,    on_dbl_click;
	web_sys::MouseEvent,    mousedown,   OnMouseDown,   on_mouse_down;
	web_sys::MouseEvent,    mouseenter,  OnMouseEnter,  on_mouse_enter;
	web_sys::MouseEvent,    mouseleave,  OnMouseLeave,  on_mouse_leave;
	web_sys::MouseEvent,    mousemove,   OnMouseMove,   on_mouse_move;
	web_sys::MouseEvent,    mouseover,   OnMouseOver,   on_mouse_over;
	web_sys::MouseEvent,    mouseout,    OnMouseOut,    on_mouse_out;
	web_sys::MouseEvent,    mouseup,     OnMouseUp,     on_mouse_up;
	web_sys::KeyboardEvent, keydown,     OnKeyDown,     on_key_down;
	web_sys::KeyboardEvent, keyup,       OnKeyUp,       on_key_up;
	web_sys::Event,         change,      OnChange,      on_change;
}

pub trait Element: Drop {
	fn element(&self) -> &web_sys::Element;
	fn class() -> String where Self: Sized + 'static {
		std::any::TypeId::of::<Self>().to_class_string("t")
	}
	fn append<C: Element>(&self, child: impl std::ops::Deref<Target = C>) -> &Self where Self: Sized {
		self.element().append_child(child.element()).expect("Can't append child");
		self
	}
	fn set_class(&self, style: &css::Style) -> &Self where Self: Sized + 'static {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(style);
			self.element().set_attribute(web_str::class(), &format!("{} {}", Self::class(), element_class)).unwrap();
			// TODO:
			// ctx.classes.borrow_mut().insert(0, element_class);
			self
		})
	}
}

pub struct BasicElement<T: AsRef<web_sys::Element>> {
	pub element: T,
}

impl<T: AsRef<web_sys::Element>> Drop for BasicElement<T> {
	fn drop(&mut self) {
		self.element.as_ref().remove();
	}
}

impl<T: AsRef<web_sys::Element>> Element for BasicElement<T> {
	fn element(&self) -> &web_sys::Element { &self.element.as_ref() }
}

#[extend::ext(name = RawSetClass)]
impl web_sys::Element {
	fn set_class(self, style: &css::Style) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(style);
			self.set_attribute(web_str::class(), &element_class).unwrap();
		})
	}
}

macro_rules! html {
	($($name:ident, $t:ident),+$(,)*) => {
		pub mod create {
			fn dom() -> web_sys::Document {
				web_sys::window().unwrap().document().unwrap()
			}

			$(
				pub fn $name() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element(crate::web_str::$name()).unwrap())) }
			)+
		}

		pub mod web_sys_element_exts {
			use super::*;

			$(
				#[extend::ext(pub)]
				impl web_sys::$t {
					fn to_element(self) -> BasicElement<web_sys::$t> {
						BasicElement { element: self }
					}
				}

				impl Default for BasicElement<web_sys::$t> {
					fn default() -> Self {
						BasicElement { element: create::$name() }
					}
				}
			)+
		}
	};
}

html![
	div, HtmlDivElement,
	span, HtmlSpanElement,
	input, HtmlInputElement,
	a, HtmlAnchorElement,
	img, HtmlImageElement,
	textarea, HtmlTextAreaElement,
];

#[extend::ext]
impl<T: Hash> T {
	fn to_class_string(&self, prefix: &str) -> String {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);
		let id = hasher.finish();
		format!("{}{}", prefix, id)
	}
}
