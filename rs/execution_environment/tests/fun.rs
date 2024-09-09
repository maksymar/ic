use ic_registry_subnet_type::SubnetType;
use ic_state_machine_tests::{Cycles, StateMachineBuilder};

const NUM_CREATOR_CANISTERS: usize = 10;
const NUM_CANISTERS_PER_CREATOR_CANISTER: usize = 50;

/*
$ bazel test //rs/execution_environment:execution_environment_misc_integration_tests/fun \
  --test_output=streamed \
  --test_arg=--nocapture \
  --test_arg=test_fun
*/
#[test]
fn test_fun() {
    let env = StateMachineBuilder::new()
        .with_subnet_type(SubnetType::Application)
        .with_checkpoints_enabled(false)
        .build();

    // Install creator canisters.
    let features = [];
    let creator_wasm =
        canister_test::Project::cargo_bin_maybe_from_env("canister_creator_canister", &features);
    let mut canister_ids = vec![];
    for _ in 0..NUM_CREATOR_CANISTERS {
        let canister_id = env
            .install_canister_with_cycles(
                creator_wasm.clone().bytes(),
                vec![],
                None,
                Cycles::new(1 << 64),
            )
            .unwrap();
        canister_ids.push(canister_id);
    }

    // Create secondary canisters.
    for canister_id in canister_ids.into_iter() {
        env.execute_ingress(
            canister_id,
            "create_canisters",
            format!("{}", NUM_CANISTERS_PER_CREATOR_CANISTER)
                .as_bytes()
                .to_vec(),
        )
        .unwrap();
    }
}
