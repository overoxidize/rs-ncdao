use crate::io_sys::{NCInit, NCInitServices, NCInitUrls};
use crate::types::{ NCCreateDao, NCGetDaoWhiteList, 
    NCCreateDaoProposal, NCCreateDaoUserWhitelistProposal, NCCreateDaoStakeProposal,
    NCApproveDaoProposal, NCExecuteDaoProposal, NCGetVotes, ActionGenerator,
    NCGetDaoProposals, NCDaoProposalVote, NCDaoWithdrawVoteDeposit, NCReturnTxs, TransactResult,
    GetTableRowsPayload, DAOPayload, ProposalPayload, TopPoolPayload, SlicePayload, VotePayload,

    RewardPayload, WhiteListPayload};

use crate::chain_api::{ChainApi};

struct NCDaosAPI {
    debug: bool,
    services: NCInitServices,
    urls: NCInitUrls,
    chain_api: ChainApi,
    dao_ag: ActionGenerator,
    // submitter: NCSubmitApi
}