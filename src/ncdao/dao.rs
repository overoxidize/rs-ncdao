use crate::io_sys::io::{NCInit, NCInitServices, NCInitUrlsDev};
use crate::types::{ NCCreateDao, NCGetDaoWhiteList, 
    NCCreateDaoProposal, NCCreateDaoUserWhitelistProposal, NCCreateDaoStakeProposal,
    NCApproveDaoProposal, NCExecuteDaoProposal, NCGetVotes, ActionGenerator,
    NCGetDaoProposals, NCDaoProposalVote, NCDaoWithdrawVoteDeposit, NCReturnTxs,
    GetTableRowsPayload, DAOPayload, ProposalPayload, TopPoolPayload, SlicePayload, VotePayload,
    RewardPayload, WhiteListPayload};

use crate::c_api::chain_api::{ChainApi};

struct NCDaosAPI {
    debug: bool,
    services: NCInitServices,
    urls: NCInitUrlsDev,
    chain_api: ChainApi,
    dao_ag: ActionGenerator,
    // submitter: NCSubmitApi
}

