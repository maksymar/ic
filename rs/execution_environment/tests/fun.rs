use ic_registry_subnet_type::SubnetType;
use ic_state_machine_tests::{Cycles, StateMachineBuilder};
use ic_test_utilities_execution_environment::{wat_canister, wat_fn};

const NUM_CREATOR_CANISTERS: usize = 10;
const NUM_CANISTERS_PER_CREATOR_CANISTER: usize = 100;
const STEPS: usize = 5;

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
ABC creators 10 x canisters per creator 50, steps 5
50,310,0.056,2.784
100,810,0.131,9.356
150,1160,0.142,16.474
200,1260,0.123,22.618
250,1260,0.108,28.040
300,1510,0.135,34.814
350,1610,0.127,41.176
400,1660,0.119,47.140
450,1660,0.125,53.369
500,1660,0.122,59.448
550,1660,0.117,65.285
600,1760,0.138,72.164
650,1760,0.114,77.862
700,1760,0.105,83.136
750,1760,0.107,88.470
800,1760,0.115,94.216
850,1810,0.112,99.826
900,1810,0.116,105.638
950,1810,0.117,111.467
1000,1810,0.127,117.807
1050,2010,0.163,125.956
1100,2010,0.125,132.192
1150,2010,0.129,138.636
1200,2010,0.127,144.965
1250,2010,0.125,151.234
1300,2010,0.130,157.758
1350,2060,0.140,164.754
1400,2060,0.132,171.330
1450,2060,0.133,177.977
1500,2060,0.130,184.475
1550,2210,0.149,191.905
1600,2260,0.143,199.038
1650,2260,0.130,205.516
1700,2260,0.126,211.833
1750,2260,0.128,218.238
1800,2260,0.138,225.127
1850,2260,0.123,231.264
1900,2310,0.130,237.777
1950,2310,0.116,243.596
2000,2310,0.128,250.014
2050,2310,0.132,256.627
2100,2310,0.133,263.301

ABC test done: 10,50,5,459.9

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
