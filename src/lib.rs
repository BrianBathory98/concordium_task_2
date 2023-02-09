//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

type Greeting = String;
/// Your smart contract state.
#[derive(Serialize, SchemaType,Clone)]
pub struct State {
    // Your state
    description: Greeting,
}

#[derive(Serialize,SchemaType)]
struct InitParameter {
    description: Greeting,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    ContractSetter
}

/// Init function that creates a new smart contract.
#[init(contract = "my_concordium_project",parameter="InitParameter")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitParameter = _ctx.parameter_cursor().get()?;

    Ok(State {
        description: param.description
    })
}


/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "my_concordium_project",
    name = "set_greeting",
    parameter = "Greeting",
    error = "ContractError",
    mutable
)]
fn set_greeting<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), ContractError> {
    // ensure that the sender is an account (not a contract)
    match ctx.sender(){
        Address::Account(_) => true, 
        Address::Contract(_) => return Err(ContractError::ContractSetter),
    };

    let greeting: Greeting = ctx.parameter_cursor().get()?;

    // update greeting
    _host.state_mut().description = greeting;

    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "my_concordium_project", name = "view", return_value = "State")]
fn view<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<State> {
    let state = host.state();
    Ok(State {
         description:state.description.clone(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_infrastructure::*;

    const ACC:AccountAddress = AccountAddress([0u8;32]);
    #[test]
    fn set_greet(){
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Account(ACC));
        let greeting = "Hello World!";
        let parameter = to_bytes(&greeting);
        ctx.set_parameter(&parameter);

        let state = State {description: "Set New Greeting".to_string()};
        let mut host = TestHost::new(state,TestStateBuilder::new());
        let result = set_greeting(&ctx,&mut host);

        assert!(result.is_ok());
        assert_eq!(host.state().description,"Hello World!")
    }
}