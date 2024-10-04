use v8::{object::traits::Object, value::traits::Value};

fn main() {
    let platform = v8::platform::Platform::new(
        16,
        v8::platform::IdleTaskSupport::Disable,
        v8::platform::InProcessStackDumping::Disable,
        v8::platform::PriorityMode::DontApply,
    )
    .unwrap();

    v8::v8::initialize_platform(platform);
    v8::v8::initialize();

    {
        let isolate_create_params = &v8::isolate::IsolateCreateParams::new(
            v8::array_buffer::ArrayBufferAllocator::new().unwrap(),
        );
        let isolate = &mut v8::isolate::Isolate::new(isolate_create_params);
        let handle_scope = &mut v8::scope::HandleScope::new(isolate);
        let context = v8::context::Context::new(handle_scope, None);
        let context_scope = &mut v8::scope::ContextScope::new(context);

        let mut test = v8::object::Object::new(handle_scope);

        test.set(
            &context_scope,
            &v8::string::String::new(handle_scope, "test").to_local_checked(),
            &v8::string::String::new(handle_scope, "test").to_local_checked(),
            None,
        );

        println!(
            "{{ test: {} }}",
            test.get(
                &context_scope,
                &v8::string::String::new(handle_scope, "test").to_local_checked(),
                None
            )
            .cast::<v8::string::String>()
            .to_local_checked()
            .as_str(handle_scope)
        );

        let synthetic_module_name =
            v8::string::String::new(isolate, "test-module").to_local_checked();

        let synthetic_module_exports =
            vec![v8::string::String::new(isolate, "test").to_local_checked()];

        let test_module = v8::module::Module::create_synthetic_module(
            handle_scope,
            &synthetic_module_name,
            synthetic_module_exports,
            |mut context, mut module| {
                let isolate = context.get_isolate().unwrap();

                let export_test = (
                    v8::string::String::new(isolate, "test").to_local_checked(),
                    v8::string::String::new(isolate, "value of test").to_local_checked(),
                );

                module.set_synthetic_module_export(
                    isolate,
                    export_test.0.cast_ref(),
                    export_test.1.cast_ref(),
                );

                v8::local::MaybeLocal::empty()
            },
        );

        println!(
            "test_module.is_synthetic_module() = {}",
            test_module.is_synthetic_module()
        );

        let source = v8::string::String::new(
            handle_scope,
            r#"
            import { test } from "test-module";

            test
            "#,
        )
        .to_local_checked();

        let mut script = v8::script::Script::compile(&context_scope, &source).to_local_checked();

        let result = script.run(&context_scope).to_local_checked();

        println!(
            "result = {}",
            result
                .to_string(context_scope)
                .to_local_checked()
                .as_str(handle_scope)
        );
    }

    v8::v8::dispose();
    v8::v8::dispose_platform();

    println!("V8 succesfully finished");
}
