//! Smoke tests.

use crate::contract::{execute, instantiate};
use crate::msg::{Deposit, ExecuteMsg, InstantiateMsg};
use crate::ContractError;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

/// Test instantiating the contract, creating a pool, adding liquidity and making a trade.
#[test]
fn happy_path() -> Result<(), ContractError> {
    let mut deps = mock_dependencies();
    let mut env = mock_env();

    let info = mock_info("admin0000", &[]);
    let _ = instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        InstantiateMsg {
            retry_delay: 60,
            job_id: "test_job".to_string(),
        },
    )?;

    let r = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::PutWithdraw {
            deposits: vec![
                Deposit {
                    deposit_id: 0u32,
                    profit_taking_or_stop_loss: true,
                },
                Deposit {
                    deposit_id: 1u32,
                    profit_taking_or_stop_loss: false,
                },
                Deposit {
                    deposit_id: 2u32,
                    profit_taking_or_stop_loss: true,
                },
            ],
        },
    )?;

    assert_eq!(r.messages.len(), 1);
    let r = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::PutWithdraw {
            deposits: vec![
                Deposit {
                    deposit_id: 0u32,
                    profit_taking_or_stop_loss: true,
                },
                Deposit {
                    deposit_id: 1u32,
                    profit_taking_or_stop_loss: false,
                },
                Deposit {
                    deposit_id: 2u32,
                    profit_taking_or_stop_loss: true,
                },
            ],
        },
    )
    .is_err();
    assert!(r);

    env.block.time = env.block.time.plus_seconds(100u64);

    let r = execute(
        deps.as_mut(),
        env,
        info.clone(),
        ExecuteMsg::PutWithdraw {
            deposits: vec![
                Deposit {
                    deposit_id: 0u32,
                    profit_taking_or_stop_loss: true,
                },
                Deposit {
                    deposit_id: 1u32,
                    profit_taking_or_stop_loss: false,
                },
                Deposit {
                    deposit_id: 2u32,
                    profit_taking_or_stop_loss: true,
                },
            ],
        },
    )?;
    assert_eq!(r.messages.len(), 1);

    Ok(())
}
