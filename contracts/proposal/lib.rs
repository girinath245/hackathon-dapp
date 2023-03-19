#![cfg_attr(not(feature = "std"), no_std)]

pub use self::proposal::{Proposal, ProposalRef};

#[ink::contract]
pub mod proposal {

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;

    use org::{Designation, OrgRef};
    use project::{ProjectRef,Error,Result};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum ProposalType {
        AddProject,
        FundProject,
        ChangeQuorum,
        ChangeProposalSpan,
    }
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum Status {
        Open,
        Rejected,
        Passed,
        Executed,
    }

    #[derive(Debug, Copy, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum Vote {
        Yes,
        No,
        Abstain,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct Votes {
        yes: u64,
        no: u64,
        abstain: u64,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProposalSpan {
        start_time: Timestamp,
        end_time: Timestamp,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct VoteInfo {
        voter: AccountId,
        vote: Vote,
    }
    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct VotingStatus {
        proposal_id: u64,
        voters: Vec<VoteInfo>,
        votes: Votes,
    }

    impl VotingStatus {
        pub fn new(id: u64) -> Self {
            let votes = Votes {
                yes: 0,
                no: 0,
                abstain: 0,
            };
            Self {
                proposal_id: id,
                voters: Vec::new(),
                votes: votes,
            }
        }
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProjectInfo {
        fund_asked: u128,
        strength: u64,
    }
    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct FundProjectInfo {
        project_id: u32,
        fund_asked: u128,
    }
    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct QuorumInfo {
        quorum: u64,
    }
    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct SpanInfo {
        duration: u64,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProposalInfo {
        proposal_type: ProposalType,
        proposal_id: u64,
        proposer: AccountId,
        proposal_span: ProposalSpan,
        voter_type: Designation,
        title: String,
        details: Vec<String>,
        status: Status,
    }

    #[ink(storage)]
    pub struct Proposal {
        next_proposal_id: u64,
        /// proposal id -> proposal infos
        proposals: Mapping<u64, ProposalInfo>,
        /// open proposals on which voting is still going on
        // open_proposals_id: Vec<u64>,                     // add this feature later
        /// proposal id -> voting status of that proposal
        voting_result: Mapping<u64, VotingStatus>,
        /// default_proposal_duration
        proposal_duration: Timestamp,
        // net voting should be checked. If voting is less than this quorum number then reject proposal after expiratiion period(it is a percentage value)
        quorum: u64, // stored as actual * 1_000_000
        // store org_contract address to execute operation based on that organistion
        org_address: AccountId,
        // store project created id (proposal id -> project id)
        // project_id: Mapping<u64, u32>,
        project_info: Mapping<u64, ProjectInfo>,
        //proposal id -> id of project created (if any)
        project_ids: Mapping<u64, u32>,
        fund_project_info: Mapping<u64, FundProjectInfo>,
        quorum_info: Mapping<u64, QuorumInfo>,
        span_info: Mapping<u64, SpanInfo>,
    }

    impl Proposal {
        // checkout accountid(you will be providing string address)
        #[ink(constructor)]
        pub fn new(organisation: AccountId) -> Self {
            let default_duration: u64 = 172800000; //2 days in millisecond
            let default_quorum: u64 = 10 * 1_000_000; //10 percent for upto 6 decimal places

            Self {
                next_proposal_id: 1,
                proposals: Mapping::new(),
                // open_proposals_id: Vec::new(),
                voting_result: Mapping::new(),
                proposal_duration: default_duration,
                quorum: default_quorum,
                org_address: organisation,
                // project_id: Mapping::new(),
                project_info: Mapping::new(),
                project_ids: Mapping::new(),
                fund_project_info: Mapping::new(),
                quorum_info: Mapping::new(),
                span_info: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            title: String,
            details: Vec<String>,
            proposal_type: ProposalType,
            add_project: Option<ProjectInfo>,
            fund_project: Option<FundProjectInfo>,
            change_quorum: Option<QuorumInfo>,
            change_proposal_span: Option<SpanInfo>,
        ) -> Result<u64> {
            let id = self.next_proposal_id;
            let caller = self.env().caller();

            let proposal_span = ProposalSpan {
                start_time: self.env().block_timestamp(),
                end_time: self.env().block_timestamp() + self.proposal_duration,
            };

            let org_instance: OrgRef =
                ink::env::call::FromAccountId::from_account_id(self.org_address);
            let member_status = org_instance.check_member(caller, false);
            match member_status {
                Ok(_) => {}
                Err(_er) => return Err(Error::MemberNotFound),
            }

            match proposal_type {
                ProposalType::AddProject => {
                    if let Some(project_info) = add_project {
                        let proposal_info = ProposalInfo {
                            proposal_type: ProposalType::AddProject,
                            proposal_id: id,
                            proposer: caller,
                            proposal_span,
                            voter_type: Designation::Secretariate,
                            title,
                            details,
                            status: Status::Open,
                        };
                        self.project_info.insert(id, &project_info);
                        self.proposals.insert(id, &proposal_info);
                        // self.open_proposals_id.push(id);

                        let voting_status = VotingStatus::new(id);
                        self.voting_result.insert(id, &voting_status);
                        self.next_proposal_id += 1;
                    } else {
                        return Err(Error::ProjectInfoNotFound);
                    }
                }
                ProposalType::FundProject => {
                    if let Some(fund_info) = fund_project {
                        let proposal_info = ProposalInfo {
                            proposal_type: ProposalType::FundProject,
                            proposal_id: id,
                            proposer: caller,
                            proposal_span,
                            voter_type: Designation::Secretariate,
                            title,
                            details,
                            status: Status::Open,
                        };
                        let project_address = org_instance.get_project_address(fund_info.project_id);
                        if project_address.is_err() {
                            return Err(Error::InvalidProjectInfo)
                        }
                        self.fund_project_info.insert(id, &fund_info);
                        self.proposals.insert(id, &proposal_info);
                        // self.open_proposals_id.push(id);

                        let voting_status = VotingStatus::new(id);
                        self.voting_result.insert(id, &voting_status);
                        self.next_proposal_id += 1;
                    } else {
                        return Err(Error::FundInfoNotFound);
                    }
                }
                ProposalType::ChangeQuorum => {
                    if let Some(quorum_info) = change_quorum {
                        let proposal_info = ProposalInfo {
                            proposal_type: ProposalType::ChangeQuorum,
                            proposal_id: id,
                            proposer: caller,
                            proposal_span,
                            voter_type: Designation::Secretariate,
                            title,
                            details,
                            status: Status::Open,
                        };
                        self.quorum_info.insert(id, &quorum_info);
                        self.proposals.insert(id, &proposal_info);
                        // self.open_proposals_id.push(id);

                        let voting_status = VotingStatus::new(id);
                        self.voting_result.insert(id, &voting_status);
                        self.next_proposal_id += 1;
                    } else {
                        return Err(Error::QuorumInfoNotFound);
                    }
                } // _ => return Err(Error::IncorrectProposalType)
                ProposalType::ChangeProposalSpan => {
                    if let Some(span_info) = change_proposal_span {
                        let proposal_info = ProposalInfo {
                            proposal_type: ProposalType::ChangeProposalSpan,
                            proposal_id: id,
                            proposer: caller,
                            proposal_span,
                            voter_type: Designation::Secretariate,
                            title,
                            details,
                            status: Status::Open,
                        };
                        self.span_info.insert(id, &span_info);
                        self.proposals.insert(id, &proposal_info);
                        // self.open_proposals_id.push(id);

                        let voting_status = VotingStatus::new(id);
                        self.voting_result.insert(id, &voting_status);
                        self.next_proposal_id += 1;
                    } else {
                        return Err(Error::SpanInfoNotFound);
                    }
                }
            }
            Ok(id)
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u64, vote: Vote) -> Result<()> {
            let caller = self.env().caller();

            if self.proposals.get(proposal_id).is_none() {
                return Err(Error::ProposalNotFound);
            }

            self.update_proposal_open_status(proposal_id);
            let proposal = self.proposals.get(proposal_id).unwrap();

            let org_address = self.org_address;

            let org_instance: OrgRef = ink::env::call::FromAccountId::from_account_id(org_address);

            match proposal.status {
                Status::Open => {
                    // for different voters
                    match proposal.voter_type {
                        Designation::Secretariate => {
                            let voter_status = org_instance.check_member(caller, true);
                            match voter_status {
                                Ok(is_true) => {
                                    if is_true {
                                        let update_status =
                                            self.update_vote_status(caller, vote, proposal_id);
                                        if update_status.is_err() {
                                            return update_status;
                                        }
                                    } else {
                                        return Err(Error::UnAuthorized);
                                    }
                                }
                                Err(_) => return Err(Error::MemberNotFound),
                            }
                        }
                        Designation::Member => {
                            let voter_status = org_instance.check_member(caller, false);
                            match voter_status {
                                Ok(is_true) => {
                                    if is_true {
                                        let update_status =
                                            self.update_vote_status(caller, vote, proposal_id);
                                        if update_status.is_err() {
                                            return update_status;
                                        }
                                    } else {
                                        return Err(Error::UnAuthorized);
                                    }
                                }
                                Err(_) => return Err(Error::MemberNotFound),
                            }
                        }
                        _ => {
                            return Err(Error::UnAuthorized);
                        }
                    }
                }
                Status::Rejected => return Err(Error::ProposalRejected),
                Status::Passed => {
                    let _resp = self.execute_proposal(proposal.proposal_id);
                    return Err(Error::ProposalClosed);
                }
                Status::Executed => return Err(Error::ProposalAlreadyExecuted),
            }

            Ok(())
        }

        fn update_vote_status(
            &mut self,
            caller: AccountId,
            vote: Vote,
            proposal_id: u64,
        ) -> Result<()> {
            let mut curr_voting_result = self.voting_result.get(proposal_id).unwrap();
            let voterinfo = curr_voting_result
                .voters
                .iter()
                .find(|voteinfo| voteinfo.voter == caller);

            if let Some(_) = voterinfo {
                return Err(Error::AlreadyVoted);
            } else {
                let voter_info = VoteInfo {
                    voter: caller,
                    vote: vote.clone(),
                };
                curr_voting_result.voters.push(voter_info);

                match vote.clone() {
                    Vote::Yes => {
                        curr_voting_result.votes.yes += 1;
                    }
                    Vote::No => {
                        curr_voting_result.votes.no += 1;
                    }
                    Vote::Abstain => {
                        curr_voting_result.votes.abstain += 1;
                    }
                }
            }
            self.voting_result.insert(proposal_id, &curr_voting_result);

            Ok(())
        }

        // passed proposal will get executed
        // can be called by anyone
        // call respective functions to execute the proposal
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u64) -> Result<()> {

            if self.proposals.get(proposal_id).is_none() {
                return Err(Error::ProposalNotFound);
            }
            
            self.update_proposal_open_status(proposal_id);
            let mut proposal = self.proposals.get(proposal_id).unwrap();
            let org_address = self.org_address;
            let mut org_instance: OrgRef = ink::env::call::FromAccountId::from_account_id(org_address);

            match proposal.status {
                Status::Open => {
                    return Err(Error::ProposalRunning);
                }
                Status::Rejected => {
                    return Err(Error::ProposalRejected);
                }
                Status::Passed => {
                    match proposal.proposal_type {
                        ProposalType::AddProject => {
                            let project_info = self.project_info.get(proposal_id).unwrap();
                            
                            let project_id = org_instance
                                .create_project(
                                    proposal.title.clone(),
                                    proposal.details.clone(),
                                    project_info.fund_asked,
                                    project_info.strength,
                                    proposal.proposer,
                                )
                                .unwrap();
                            proposal.status = Status::Executed;
                            self.project_ids.insert(proposal.proposal_id, &project_id);
                            self.proposals.insert(proposal.proposal_id, &proposal);
                        }
                        ProposalType::FundProject => {
                            let fund_project_info = self.fund_project_info.get(proposal_id).unwrap();
                            
                            let fund_transfer_status = org_instance
                                .transfer_fund(
                                    fund_project_info.project_id,
                                    fund_project_info.fund_asked
                                );
                            if fund_transfer_status.is_err() {
                                return fund_transfer_status
                            }
                            let project_address = org_instance.get_project_address(fund_project_info.project_id).unwrap();
                            let mut project_instance: ProjectRef = ink::env::call::FromAccountId::from_account_id(project_address);
                            project_instance.update_fund_allocated(fund_project_info.fund_asked);

                            proposal.status = Status::Executed;
                            self.proposals.insert(proposal.proposal_id, &proposal);
                        }
                        ProposalType::ChangeQuorum => {
                            let quorum_info = self.quorum_info.get(proposal_id).unwrap();
                            
                            self.quorum = quorum_info.quorum;
                            proposal.status = Status::Executed;
                            self.proposals.insert(proposal.proposal_id, &proposal);
                        }
                        ProposalType::ChangeProposalSpan => {
                            let span_info = self.span_info.get(proposal_id).unwrap();
                            
                            self.proposal_duration = span_info.duration;
                            proposal.status = Status::Executed;
                            self.proposals.insert(proposal.proposal_id, &proposal);
                        }
                    }
                }
                Status::Executed => {
                    return Err(Error::ProposalAlreadyExecuted);
                }
            }

            Ok(())
        }

        // voting is only done by secretaries of organisation
        fn update_proposal_open_status(&mut self, proposal_id: u64) {
            let mut proposal = self.proposals.get(proposal_id).unwrap();
            let curr_voting_result = self.voting_result.get(proposal_id).unwrap();
            let org_address = self.org_address;

            let org_instance: OrgRef = ink::env::call::FromAccountId::from_account_id(org_address);

            if self.env().block_timestamp() > proposal.proposal_span.end_time {
                
                let mut total_voters = 0;
                match proposal.voter_type {
                    Designation::Secretariate => {
                        total_voters = org_instance.total_secretariate();
                    },
                    Designation::Member => {
                        total_voters = org_instance.total_members();
                    },
                    _ => {} 
                }
                match proposal.status {
                    Status::Open => {
                        // Till 6 digit after decimal
                        let factor: u64 = 1_000_000;
                        let voted_quorum: u64 =
                            (curr_voting_result.voters.len() as u64 * factor) / total_voters;
                        if curr_voting_result.votes.no <= curr_voting_result.votes.yes
                            && (self.quorum) <= voted_quorum
                        {
                            proposal.status = Status::Passed;
                        } else {
                            proposal.status = Status::Rejected;
                        }
                        self.proposals.insert(proposal.proposal_id, &proposal);
                    }
                    _ => {}
                }
            }
        }

        // QUERIES  QUERIES QUERIES QUERIES QUERIES QUERIES  QUERIES QUERIES QUERIES QUERIES
        // QUERIES  QUERIES QUERIES QUERIES QUERIES QUERIES  QUERIES QUERIES QUERIES QUERIES
        // QUERIES  QUERIES QUERIES QUERIES QUERIES QUERIES  QUERIES QUERIES QUERIES QUERIES

        #[ink(message)]
        pub fn get_voting_status(&self, proposal_id: u64) -> Result<VotingStatus> {
            let voting_status = self.voting_result.get(proposal_id);
            if let Some(status) = voting_status {
                return Ok(status);
            } else {
                return Err(Error::ProposalNotFound);
            }
        }
        #[ink(message)]
        pub fn get_proposal_info(&self, proposal_id: u64) -> Result<ProposalInfo> {
            let proposal_info = self.proposals.get(proposal_id);
            if let Some(info) = proposal_info {
                return Ok(info);
            } else {
                return Err(Error::ProposalNotFound);
            }
        }
        #[ink(message)]
        pub fn get_span_info(&self, proposal_id: u64) -> Result<SpanInfo> {
            let span_info = self.span_info.get(proposal_id);
            if let Some(info) = span_info {
                return Ok(info);
            } else {
                return Err(Error::SpanInfoNotFound);
            }
        }
        #[ink(message)]
        pub fn get_quorum_info(&self, proposal_id: u64) -> Result<QuorumInfo> {
            let quorum_info = self.quorum_info.get(proposal_id);
            if let Some(info) = quorum_info {
                return Ok(info);
            } else {
                return Err(Error::QuorumInfoNotFound);
            }
        }
        #[ink(message)]
        pub fn get_project_fund_info(&self, proposal_id: u64) -> Result<FundProjectInfo> {
            let fund_info = self.fund_project_info.get(proposal_id);
            if let Some(info) = fund_info {
                return Ok(info);
            } else {
                return Err(Error::FundInfoNotFound);
            }
        }
        #[ink(message)]
        pub fn get_project_id(&self, proposal_id: u64) -> Result<u32> {
            let project_id = self.project_ids.get(proposal_id);
            if let Some(info) = project_id {
                return Ok(info);
            } else {
                return Err(Error::ProjectIdNotFound);
            }
        }
    }
}
