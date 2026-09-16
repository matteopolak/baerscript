use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPES: &'static str = r#"
export interface ExecutionResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  steps: number;
}
"#;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(typescript_type = "ExecutionResult")]
	pub type ExecutionResult;
}

/// Executes BaerScript synchronously with captured input and output.
///
/// Browser callers should invoke this inside a Web Worker and terminate the
/// worker to enforce a wall-clock timeout. `max_steps` provides a separate,
/// deterministic instruction budget.
#[wasm_bindgen]
pub fn execute(source: &str, input: &str, ascii: bool, max_steps: u32) -> ExecutionResult {
	let result = runtime::execute::execute(source, input, ascii, max_steps);
	let object = Object::new();

	set(&object, "ok", JsValue::from_bool(result.ok));
	set(&object, "stdout", JsValue::from_str(&result.stdout));
	set(&object, "stderr", JsValue::from_str(&result.stderr));
	set(&object, "steps", JsValue::from_f64(result.steps as f64));

	object.unchecked_into()
}

fn set(object: &Object, key: &str, value: JsValue) {
	Reflect::set(object, &JsValue::from_str(key), &value)
		.expect("setting a property on a fresh object cannot fail");
}

#[cfg(test)]
mod tests {
	use super::*;
	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn exposes_a_structured_result() {
		let result = execute("+v\n+v\n>v", "", false, 100);
		let result = result.as_ref();

		assert_eq!(
			Reflect::get(result, &"ok".into()).unwrap(),
			JsValue::from_bool(true)
		);
		assert_eq!(
			Reflect::get(result, &"stdout".into()).unwrap(),
			JsValue::from_str("2")
		);
		assert_eq!(
			Reflect::get(result, &"stderr".into()).unwrap(),
			JsValue::from_str("")
		);
		assert!(
			Reflect::get(result, &"steps".into())
				.unwrap()
				.as_f64()
				.unwrap() > 0.0
		);
	}
}
