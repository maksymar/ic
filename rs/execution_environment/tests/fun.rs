use ic_registry_subnet_type::SubnetType;
use ic_state_machine_tests::{Cycles, StateMachineBuilder};
use ic_test_utilities_execution_environment::{wat_canister, wat_fn};

const NUM_CREATOR_CANISTERS: usize = 10;
const NUM_CANISTERS_PER_CREATOR_CANISTER: usize = 100;
const STEPS: usize = 7;

fn bytes_to_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/*
v4
$ bazel test //rs/execution_environment:execution_environment_misc_integration_tests/fun \
  --test_output=streamed \
  --test_arg=--nocapture \
  --test_arg=test_fun_spinup
===
100,1010,0.123,12.3,14.3
200,1510,0.146,26.9,34.4
300,1610,0.124,39.3,53.9
400,1710,0.121,51.4,74.0
500,2010,0.141,65.5,96.7
600,2010,0.119,77.4,118.0
700,2010,0.126,90.1,141.1
800,2110,0.135,103.6,164.9
900,2110,0.128,116.4,188.5
1000,2510,0.156,132.1,214.9
1100,2510,0.130,145.1,238.9
1200,2510,0.136,158.7,265.1
1300,2510,0.132,171.8,290.2
1400,2610,0.153,187.1,318.7
1500,2610,0.135,200.6,345.0
1600,2610,0.140,214.6,372.7
1700,3010,0.169,231.5,402.9
1800,3010,0.141,245.6,431.2
1900,3010,0.136,259.2,459.1
2000,3010,0.137,272.9,488.1
2100,3010,0.143,287.2,518.7
2200,3110,0.150,302.2,549.5
2300,3110,0.142,316.4,579.9
2400,3310,0.155,331.9,611.3
2500,3510,0.158,347.7,643.1
2600,3510,0.143,362.0,673.8
2700,3510,0.147,376.7,705.4
2800,3510,0.142,390.9,737.2
2900,3510,0.144,405.2,769.1

*/
#[test]
fn test_fun_spinup() {
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

    let total_canisters_num = NUM_CREATOR_CANISTERS * NUM_CANISTERS_PER_CREATOR_CANISTER * STEPS;
    println!(
        "\nABC creators {} x canisters per creator {} x steps {} = {}",
        NUM_CREATOR_CANISTERS, NUM_CANISTERS_PER_CREATOR_CANISTER, STEPS, total_canisters_num
    );
    //println!("\nrounds,canisters,time");
    for _i in 0..STEPS {
        let wasm_module = wat_canister()
            .heartbeat(
                wat_fn()
                    .debug_print(b"hi!")
                    .wait(5_000)
                    .debug_print(b"bye!"),
            )
            .build_wasm();
        let arg: Vec<u8> = vec![];

        // let timer = std::time::Instant::now();
        for canister_id in creator_canister_ids.iter() {
            env.execute_ingress(
                *canister_id,
                "spinup_canisters",
                format!(
                    r#"[{},[{}],[{}]]"#,
                    NUM_CANISTERS_PER_CREATOR_CANISTER,
                    bytes_to_str(&wasm_module),
                    bytes_to_str(&arg)
                )
                .as_bytes()
                .to_vec(),
            )
            .expect("Failed to execute 'spinup_canisters' ingress");
        }
        // println!(
        //     "{},{:>0.1}",
        //     env.num_running_canisters() - NUM_CREATOR_CANISTERS as u64,
        //     timer.elapsed().as_secs_f64()
        // );
    }

    println!(
        "\nABC test done: {},{},{},{:>0.1}",
        NUM_CREATOR_CANISTERS,
        NUM_CANISTERS_PER_CREATOR_CANISTER,
        STEPS,
        total_timer.elapsed().as_secs_f64()
    );
}

/*
v1
$ bazel test //rs/execution_environment:execution_environment_misc_integration_tests/fun \
  --test_output=streamed \
  --test_arg=--nocapture \
  --test_arg=test_fun_create_install_steps
*/
#[test]
fn test_fun_create_install_steps() {
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
