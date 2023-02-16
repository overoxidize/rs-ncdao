use crate::io_sys::io::{NCInit, NCInitServices, NCInitUrlsDev};
use crate::types::{
    ActionGenerator, DAOPayload, GetTableRowsPayload, NCApproveDaoProposal, NCCreateDao,
    NCCreateDaoProposal, NCCreateDaoStakeProposal, NCCreateDaoUserWhitelistProposal,
    NCDaoProposalVote, NCDaoWithdrawVoteDeposit, NCExecuteDaoProposal, NCGetDaoProposals,
    NCGetDaoWhiteList, NCGetVotes, NCReturnTxs, ProposalPayload, RewardPayload, SlicePayload,
    TopPoolPayload, VotePayload, WhiteListPayload,
};

use crate::c_api::chain_api::ChainApi;

struct NCDaosAPI {
    debug: bool,
    services: NCInitServices,
    urls: NCInitUrlsDev,
    chain_api: ChainApi,
    dao_ag: ActionGenerator,
    // submitter: NCSubmitApi
}
