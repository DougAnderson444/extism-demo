//! Extism Demo

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use extism::{Context, CurrentPlugin, Function, Plugin, UserData, Val, ValType};

    fn hello_world(
        _plugin: &mut CurrentPlugin,
        inputs: &[Val],
        outputs: &mut [Val],
        _user_data: UserData,
    ) -> Result<(), Error> {
        outputs[0] = inputs[0].clone();
        Ok(())
    }

    #[test]
    fn it_works() {
        // works
        let wasm = include_bytes!("code-functions.wasm");

        // fails
        // let wasm = include_bytes!("../target/wasm32-unknown-unknown/release/extism_plugin.wasm");

        let context = Context::new();

        let f = Function::new(
            "hello_world",
            [ValType::I64],
            [ValType::I64],
            None,
            hello_world,
        )
        .with_namespace("env");

        let mut plugin = Plugin::new(&context, wasm, [f], true).unwrap();

        let input = "this is a test";
        let data = plugin.call("count_vowels", input).unwrap();

        assert_eq!(data, b"{\"count\": 4}");
    }
}
