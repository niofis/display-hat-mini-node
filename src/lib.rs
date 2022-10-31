#[macro_use]
extern crate lazy_static;
use display_hat_mini_driver::DisplayHATMini;
use neon::{prelude::*, types::buffer::TypedArray};
use std::sync::Mutex;

lazy_static! {
    static ref DISPLAY: Mutex<DisplayHATMini> = Mutex::new(DisplayHATMini::new().unwrap());
}

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    DISPLAY.lock().unwrap().init().unwrap();
    Ok(cx.undefined())
}

fn display(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut data = cx.argument::<JsBuffer>(0)?;
    let buffer = data.as_mut_slice(&mut cx);
    DISPLAY.lock().unwrap().display_rgb(buffer).unwrap();
    Ok(cx.undefined())
}

fn width(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let width = DISPLAY.lock().unwrap().width;
    Ok(cx.number(width))
}

fn height(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let height = DISPLAY.lock().unwrap().height;
    Ok(cx.number(height))
}

fn set_vsync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let use_vsync = cx.argument::<JsBoolean>(0)?;
    DISPLAY.lock().unwrap().set_vsync(use_vsync.value(&mut cx));
    Ok(cx.undefined())
}

fn read_buttons(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let buttons = DISPLAY.lock().unwrap().read_buttons().unwrap();
    Ok(cx.number(buttons))
}

fn set_led(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let red = cx.argument::<JsNumber>(0)?;
    let green = cx.argument::<JsNumber>(1)?;
    let blue = cx.argument::<JsNumber>(2)?;
    DISPLAY.lock().unwrap().set_led(
        red.value(&mut cx) as u8,
        green.value(&mut cx) as u8,
        blue.value(&mut cx) as u8)
        .unwrap();
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("display", display)?;
    cx.export_function("width", width)?;
    cx.export_function("height", height)?;
    cx.export_function("setVSync", set_vsync)?;
    cx.export_function("readButtons", read_buttons)?;
    cx.export_function("setLED", set_led)?;
    Ok(())
}
