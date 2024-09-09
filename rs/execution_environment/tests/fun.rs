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
100,1010,0.122,12.2,14.1
200,1510,0.144,26.6,33.7
300,1610,0.116,38.3,51.9
400,1710,0.120,50.2,71.6
500,2010,0.143,64.6,94.6
600,2010,0.127,77.3,117.1
700,2010,0.126,89.9,140.0
800,2110,0.140,103.9,165.0
900,2110,0.129,116.8,188.1
1000,2510,0.164,133.2,215.3
1100,2510,0.134,146.6,239.6
1200,2510,0.134,160.0,265.1
1300,2510,0.139,173.9,291.8
1400,2610,0.150,188.8,320.0
1500,2610,0.139,202.7,347.2
1600,2610,0.140,216.8,374.5
1700,3010,0.172,234.0,404.8
1800,3010,0.138,247.8,432.1
1900,3010,0.136,261.4,459.6
2000,3010,0.139,275.3,489.2
2100,3010,0.139,289.2,517.8
2200,3110,0.148,304.0,547.8
2300,3110,0.145,318.5,578.4
2400,3310,0.157,334.1,609.5
2500,3510,0.161,350.2,641.2
2600,3510,0.145,364.7,671.9
2700,3510,0.143,379.0,702.5
2800,3510,0.143,393.3,733.9
2900,3510,0.145,407.8,766.6
3000,3610,0.148,422.6,798.5
3100,3610,0.145,437.0,830.5
3200,3610,0.145,451.5,862.7
3300,3810,0.153,466.8,895.7

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
