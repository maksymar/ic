use ic_registry_subnet_type::SubnetType;
use ic_state_machine_tests::{Cycles, StateMachineBuilder};
use ic_test_utilities_execution_environment::{wat_canister, wat_fn};

const NUM_CREATOR_CANISTERS: usize = 10;
const NUM_CANISTERS_PER_CREATOR_CANISTER: usize = 40;
const STEPS: usize = 5;

fn bytes_to_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/*
$ bazel test //rs/execution_environment:execution_environment_misc_integration_tests/fun \
  --test_output=streamed \
  --test_arg=--nocapture \
  --test_arg=test_fun
*/
#[test]
fn test_fun() {
    let total_timer = std::time::Instant::now();

    let env = StateMachineBuilder::new()
        .with_subnet_type(SubnetType::Application)
        .with_checkpoints_enabled(false)
        .build();

    // Install creator canisters.
    println!("ABC test: install creator canisters");
    let features = [];
    let creator_wasm =
        canister_test::Project::cargo_bin_maybe_from_env("canister_creator_canister", &features);
    let mut creator_canister_ids = vec![];
    for _ in 0..NUM_CREATOR_CANISTERS {
        let canister_id = env
            .install_canister_with_cycles(
                creator_wasm.clone().bytes(),
                vec![],
                None,
                Cycles::new(1 << 64),
            )
            .unwrap();
        creator_canister_ids.push(canister_id);
    }

    println!(
        "\nABC creators {} x canisters per creator {}, steps {}",
        NUM_CREATOR_CANISTERS, NUM_CANISTERS_PER_CREATOR_CANISTER, STEPS
    );
    println!("\ncanisters,time");
    for _i in 0..STEPS {
        // Create secondary canisters.
        //println!("ABC test: create secondary canisters");
        for canister_id in creator_canister_ids.iter() {
            env.execute_ingress(
                *canister_id,
                "create_canisters",
                format!("{}", NUM_CANISTERS_PER_CREATOR_CANISTER)
                    .as_bytes()
                    .to_vec(),
            )
            .expect("Failed to execute 'create_canisters' ingress");
        }

        // Install code on secondary canisters.
        //println!("ABC test: install code on secondary canisters");
        let wasm_module = wat_canister()
            .heartbeat(
                wat_fn()
                    .debug_print(b"hi!")
                    .wait(5_000)
                    .debug_print(b"bye!"),
            )
            .build_wasm();
        let arg: Vec<u8> = vec![];
        for canister_id in creator_canister_ids.iter() {
            env.execute_ingress(
                *canister_id,
                "install_code",
                format!(
                    r#"[[{}],[{}]]"#,
                    bytes_to_str(&wasm_module),
                    bytes_to_str(&arg)
                )
                .as_bytes()
                .to_vec(),
            )
            .expect("Failed to execute 'install_code' ingress");
        }

        let timer = std::time::Instant::now();
        env.tick();
        println!(
            "{},{:>0.3}",
            env.num_running_canisters() - NUM_CREATOR_CANISTERS as u64,
            timer.elapsed().as_secs_f64()
        );
    }

    println!(
        "\nABC test done: {},{},{},{:>0.1}",
        NUM_CREATOR_CANISTERS,
        NUM_CANISTERS_PER_CREATOR_CANISTER,
        STEPS,
        total_timer.elapsed().as_secs_f64()
    );
}
