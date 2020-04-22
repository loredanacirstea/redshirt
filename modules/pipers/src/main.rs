fn main() {
    redshirt_log_interface::log(redshirt_log_interface::Level::Info, &"pipers!");
    redshirt_syscalls::block_on(async_main())
}

async fn async_main() {
    // redshirt_time_interface::monotonic_wait(Duration::from_secs(5)).await;
    //
    // redshirt_interface_interface::register_interface(redshirt_loader_interface::ffi::INTERFACE)
    //     .await
    //     .unwrap();
    //
    // let rp = redshirt_loader_interface::ffi::LoadResponse { result: Ok(data) };
    // redshirt_syscalls::emit_answer(user_data, &rp);

    let wasm_file_content: Vec<u8> = vec!(0, 97, 115, 109, 1, 0, 0, 0, 1, 7, 1, 96, 2, 127, 127, 1, 127, 3, 2, 1, 0, 7, 7, 1, 3, 97, 100, 100, 0, 0, 10, 9, 1, 7, 0, 32, 0, 32, 1, 106, 11);

    let result = redshirt_pipeline_interface::run(
        redshirt_pipeline_interface::ffi::PipelineMessage {
            module: wasm_file_content,
            funcname: "add".as_bytes().to_vec(), // &"sum_u32",
            inputs: vec!(8,3),
        }
    ).await.unwrap();
    println!("Result {:?}", result);


    // let user_data = redshirt_syscalls::MessageId::from(9);
    // // let data = include_bytes!("/Users/loredana/wasm/pipeliners/tests/libs/sums/target/wasm32-unknown-unknown/release/sums.wasm");
    //  let data = include_bytes!("/Users/loredana/wasm/redshirt/modules/target/wasm32-wasi/release/hello-world.wasm");

    // let rp = redshirt_loader_interface::ffi::LoadResponse { result: Ok(data.to_vec()) };
    // redshirt_syscalls::emit_answer(user_data, &rp);
    // redshirt_log_interface::log(redshirt_log_interface::Level::Info, &"pipers222!");


    // redshirt_syscalls::emit_message_with_response(
    //     redshirt_loader_interface::ffi::INTERFACE,
    //
    // )
    //
    // let message_id = self.core.emit_interface_message_answer(
    //     self.load_source_virtual_pid,
    //     redshirt_loader_interface::ffi::INTERFACE,
    //     redshirt_loader_interface::ffi::LoaderMessage::Load(From::from(hash)),
    // );
}
