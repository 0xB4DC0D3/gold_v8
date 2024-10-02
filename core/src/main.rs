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
        let isolate = v8::isolate::Isolate::new(isolate_create_params).unwrap();
        let handle_scope = v8::scope::HandleScope::new(isolate);
        let context = v8::context::Context::new(&handle_scope, None);
        let context_scope = v8::scope::ContextScope::new(context);

        let source = v8::string::String::new(&handle_scope, r#""hello, " + "world""#).to_local_checked();

        let mut script = v8::script::Script::compile(&context_scope, &source);

        let result = script.run(&context_scope);

        println!(
            "result = {}",
            result
                .to_string(&context_scope)
                .to_local_checked()
                .as_str(&handle_scope)
        );

        let mut test = v8::object::Object::new(&handle_scope);

        test.set(
            &context_scope,
            &v8::string::String::new(&handle_scope, "test").to_local_checked(),
            &v8::string::String::new(&handle_scope, "test").to_local_checked(),
            None,
        );

        println!(
            "{{ test: {} }}",
            test.get(
                &context_scope,
                &v8::string::String::new(&handle_scope, "test").to_local_checked(),
                None
            )
            .cast::<v8::string::String>()
            .to_local_checked()
            .as_str(&handle_scope)
        );
    }

    v8::v8::dispose();
    v8::v8::dispose_platform();

    println!("V8 succesfully finished");
}
