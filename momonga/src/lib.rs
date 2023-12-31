mod ast;
mod data;
mod env;
mod error;
mod eval;
mod parser;

use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::CustomEvent;

use crate::env::Env;
use crate::eval::eval;
use crate::parser::parse;

pub fn interpret(src: &str) -> Option<String> {
    match parse(src) {
        Ok(ast) => match eval(&ast, Rc::new(RefCell::new(Env::new_with_builtins()))) {
            Ok(val) => val.map(|val| (*val.borrow()).to_string()),
            Err(eval_err) => Some(eval_err.to_string()),
        },
        Err(parse_err) => Some(parse_err.to_string()),
    }
}

#[wasm_bindgen]
pub enum OutputEvent {
    Stdout,
    Stderr,
}

#[wasm_bindgen]
pub fn emit_output_event(output_event: OutputEvent, data: &str) {
    let window = web_sys::window().unwrap();
    let type_ = match output_event {
        OutputEvent::Stdout => "stdout",
        OutputEvent::Stderr => "stderr",
    };
    let event = CustomEvent::new(type_).unwrap();
    event.init_custom_event_with_can_bubble_and_cancelable_and_detail(
        type_,
        true,
        true,
        &JsValue::from_str(data),
    );
    window.dispatch_event(&event).unwrap();
}

#[wasm_bindgen]
pub fn momonga_run(source: &str) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    match parse(source) {
        Ok(ast) => match eval(&ast, Rc::new(RefCell::new(Env::new_with_builtins()))) {
            Ok(_) => (),
            Err(eval_err) => emit_output_event(OutputEvent::Stderr, &eval_err.to_string()),
        },
        Err(parse_err) => emit_output_event(OutputEvent::Stderr, &parse_err.to_string()),
    }
}

#[wasm_bindgen]
pub fn is_momonga_parse_error(source: &str) -> bool {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    match parse(source) {
        Ok(_ast) => false,
        Err(_parse_err) => true,
    }
}
