#![no_std]

use ft_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <FungibleTokenMetadata as Metadata>::State;

    pub fn name(state: State) -> String {
        state.1.name
    }

    pub fn symbol(state: State) -> String {
        state.1.symbol
    }

    pub fn decimals(state: State) -> u8 {
        state.1.decimals
    }

    pub fn total_supply(state: State) -> u128 {
        state.1.total_supply
    }

    pub fn balances_of(state: State, account: ActorId) -> u128 {
        match state
            .1
            .balances
            .iter()
            .find(|(id, _balance)| account.eq(id))
        {
            Some((_id, balance)) => *balance,
            None => panic!("Balance for account ID {account:?} not found",),
        }
    }
}
