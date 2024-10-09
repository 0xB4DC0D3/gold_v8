use v8::{local::MaybeLocal, value::traits::Value};

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

        let source = v8::string::String::new(
            handle_scope,
            r#"
            import { test } from "test-module" with { type: "attrval" };

            test("hello, world!");
            "#,
        )
        .to_local_checked();

        let script_origin = v8::script_origin::ScriptOrigin::new_default(
            v8::string::String::new(handle_scope, "main")
                .to_local_checked()
                .cast_ref(),
            true,
        );

        let source = &mut v8::script_compiler::Source::new(&source, &script_origin);
        let mut main_module =
            v8::script_compiler::ScriptCompiler::compile_module(handle_scope, source)
                .to_local_checked();

        main_module.instantiate_module(
            context_scope,
            |mut context, specifier, _import_attributes, _referrer| {
                let isolate = context.get_isolate().unwrap();
                let specifier_name = specifier.as_str(isolate);

                match specifier_name {
                    "test-module" => v8::local::MaybeLocal::from_local({
                        let synthetic_module_name =
                            v8::string::String::new(isolate, "test-module").to_local_checked();

                        let synthetic_module_exports =
                            vec![v8::string::String::new(isolate, "test").to_local_checked()];

                        _import_attributes
                            .iter(&isolate.get_current_context())
                            .for_each(|attribute| {
                                let attribute_string = attribute.cast::<v8::string::String>();

                                if attribute_string.is_string() {
                                    println!("attribute = {}", attribute_string.as_str(isolate));
                                }
                            });

                        v8::module::Module::create_synthetic_module(
                            isolate,
                            &synthetic_module_name,
                            synthetic_module_exports,
                            |mut context, mut module| {
                                let isolate = context.get_isolate().unwrap();

                                let export_test = (
                                    v8::string::String::new(isolate, "test").to_local_checked(),
                                    v8::function::Function::new_default(
                                        &isolate.get_current_context(),
                                        |pci| {
                                            let isolate = pci.get_isolate().unwrap();

                                            println!(
                                                "printed value = {}",
                                                pci.at(0)
                                                    .to_string(&isolate.get_current_context())
                                                    .to_local_checked()
                                                    .as_str(isolate)
                                            );
                                        },
                                    )
                                    .to_local_checked(),
                                );

                                module.set_synthetic_module_export(
                                    isolate,
                                    export_test.0.cast_ref(),
                                    export_test.1.cast_ref(),
                                );

                                MaybeLocal::from_local(
                                    v8::boolean::Boolean::new(isolate, true).cast(),
                                )
                            },
                        )
                    }),
                    _ => v8::local::MaybeLocal::empty(),
                }
            },
        );

        let result = main_module.evaluate(context_scope).to_local_checked();

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
