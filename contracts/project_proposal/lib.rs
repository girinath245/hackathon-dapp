#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pproposal::{Pproposal, PproposalRef};

#[ink::contract]
pub mod pproposal {

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;
    use project::{AcceptedApproach, Error, ProjectRef, Result, Task};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum ProposalType {
        ProposeApproach,
        ExtendDeadLine,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum Status {
        /// open for voting.
        Open,
        /// rejected.
        Rejected,
        /// proposal passed but not executed. Check on every voting
        Passed,
        /// passed and executed.
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
        /// The address that voted.
        voter: AccountId,
        /// vote status.
        vote: Vote,
    }
    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct VotingStatus {
        proposal_id: u64,
        voters: Vec<VoteInfo>,
        votes: Votes,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct DeadLineExtensionInfo {
        task_id: u16,
        deadline: Timestamp,
        details: Vec<String>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProposalInfo {
        /// contains all the type that will be executed on passing quorum
        proposal_type: ProposalType,
        proposal_id: u64,
        proposer: AccountId,
        proposal_span: ProposalSpan,
        title: String,
        status: Status,
    }

    #[ink(storage)]
    pub struct Pproposal {
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
        quorum: u64,
        // if proposal type is ProposeApproach then store the approach here after sorting out graph complecations
        approach_info: Mapping<u64, AcceptedApproach>,
        deadline_extension_info: Mapping<u64, DeadLineExtensionInfo>,

        project_address: AccountId,
    }

    impl Pproposal {
        // checkout accountid(you will be providing string address)
        #[ink(constructor)]
        pub fn new(project_address: AccountId) -> Self {
            // only owner of organisation should instantiate this contract
            let default_duration: u64 = 172800000; //2 days in millisecond
            let default_quorum: u64 = 10 * 1_000_000; //10 percent for upto 6 decimal places
            Self {
                next_proposal_id: 1,
                proposals: Mapping::new(),
                // open_proposals_id: Vec::new(),
                voting_result: Mapping::new(),
                proposal_duration: default_duration,
                quorum: default_quorum,
                project_address,
                approach_info: Mapping::new(),
                deadline_extension_info: Mapping::new(),
            }
        }

        /*
        1. Create proposal
        2. Voting on proposal by board members(add expiraton after first round of development)
        3. Check voting result
        4. execute proposal
        */

        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            title: String,
            proposal_type: ProposalType,
            approach: Option<AcceptedApproach>, // the list of task should be in specified order(only depend on previous task)
            deadline_extension_info: Option<DeadLineExtensionInfo>,
        ) -> Result<u64> {
            let id = self.next_proposal_id;
            let caller = self.env().caller();
            let proposal_span = ProposalSpan {
                start_time: self.env().block_timestamp(),
                end_time: self.env().block_timestamp() + self.proposal_duration,
            };

            match proposal_type {
                ProposalType::ProposeApproach => {
                    if let Some(mut approach) = approach {
                        if approach.project_start_date < proposal_span.end_time {
                            return Err(Error::CannotStartProjectBeforeVoting);
                        }

                        // create node
                        let mut task_by_id: Mapping<u16, Task> = Mapping::new();
                        let mut task_list: Vec<Task> = Vec::new(); // contains all task to be sent to project after passing

                        for task in approach.tasks {
                            if task.end_time > approach.project_completion_date {
                                return Err(Error::TaskDeadlineCannotCrossPojectDeadLine);
                            }
                            create_task_nodes(&mut task_by_id, &mut task_list, task);
                        }
                        // update task list with graph node informations
                        approach.tasks = task_list;

                        let proposal_info = ProposalInfo {
                            proposal_type: ProposalType::ProposeApproach,
                            proposal_id: id,
                            proposer: self.env().caller(),
                            proposal_span: proposal_span,
                            title: title,
                            status: Status::Open,
                        };
                        self.proposals.insert(id, &proposal_info);
                        // self.open_proposals_id.push(id);
                        self.approach_info.insert(id, &approach);

                        let votes = Votes {
                            yes: 0,
                            no: 0,
                            abstain: 0,
                        };
                        let voting_status = VotingStatus {
                            proposal_id: id,
                            voters: Vec::new(),
                            votes: votes,
                        };
                        self.voting_result.insert(id, &voting_status);
                        self.next_proposal_id += 1;
                    } else {
                        return Err(Error::ApproachNotDefined);
                    }
                }
                ProposalType::ExtendDeadLine => {
                    if let Some(deadline_info) = deadline_extension_info {
                        let project_instance: ProjectRef =
                            ink::env::call::FromAccountId::from_account_id(self.project_address);
                        // check only task node members
                        match project_instance.try_check_voters_for_deadline_extension(
                            deadline_info.task_id,
                            caller,
                            true,
                        ) {
                            Ok(_) => {
                                match project_instance.is_possible_deadline(
                                    deadline_info.task_id,
                                    deadline_info.deadline,
                                ) {
                                    Ok(_) => {
                                        let proposal_info = ProposalInfo {
                                            proposal_type: ProposalType::ExtendDeadLine,
                                            proposal_id: id,
                                            proposer: self.env().caller(),
                                            proposal_span: proposal_span,
                                            title: title,
                                            status: Status::Open,
                                        };
                                        self.proposals.insert(id, &proposal_info);
                                        // self.open_proposals_id.push(id);
                                        self.deadline_extension_info.insert(id, &deadline_info);

                                        let votes = Votes {
                                            yes: 0,
                                            no: 0,
                                            abstain: 0,
                                        };
                                        let voting_status = VotingStatus {
                                            proposal_id: id,
                                            voters: Vec::new(),
                                            votes: votes,
                                        };
                                        self.voting_result.insert(id, &voting_status);
                                        self.next_proposal_id += 1;
                                    }
                                    Err(err) => return Err(err),
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(Error::DeadLineNotDefined);
                    }
                } // _ => return Err(Error::IncorrectProposalType)
            }

            Ok(id)
        }

        // check proposal open status
        // check already voted
        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u64, vote: Vote) -> Result<()> {
            let caller = self.env().caller();
            self.update_proposal_open_status(proposal_id).unwrap();

            let proposal = self.proposals.get(proposal_id).unwrap();
            let mut _curr_voting_result = self.voting_result.get(proposal_id).unwrap();
            let project_address = self.project_address;

            let project_instance: ProjectRef =
                ink::env::call::FromAccountId::from_account_id(project_address);

            match proposal.status {
                Status::Open => match proposal.proposal_type {
                    ProposalType::ProposeApproach => {
                        if project_instance.check_member(caller) {
                            let updated_voting_result =
                                update_voting_result(caller, _curr_voting_result, vote);

                            match updated_voting_result {
                                Ok(updated_result) => {
                                    self.voting_result
                                        .insert(proposal.proposal_id, &updated_result);
                                }
                                Err(err) => return Err(err),
                            }
                        }
                    }
                    ProposalType::ExtendDeadLine => {
                        let deadline_extension_info =
                            self.deadline_extension_info.get(proposal_id).unwrap();
                        match project_instance.try_check_voters_for_deadline_extension(
                            deadline_extension_info.task_id,
                            caller,
                            false,
                        ) {
                            Ok(status) => {
                                if status {
                                    let updated_voting_result =
                                        update_voting_result(caller, _curr_voting_result, vote);

                                    match updated_voting_result {
                                        Ok(updated_result) => {
                                            self.voting_result
                                                .insert(proposal.proposal_id, &updated_result);
                                        }
                                        Err(err) => return Err(err),
                                    }
                                } else {
                                    return Err(Error::UnAuthorized);
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    }
                },
                Status::Rejected => return Err(Error::ProposalRejected),
                Status::Passed => {
                    // self.execute_proposal(proposal.proposal_id);
                    return Err(Error::ProposalClosed);
                }
                Status::Executed => return Err(Error::ProposalAlreadyExecuted),
            }

            Ok(())
        }

        fn update_proposal_open_status(&mut self, proposal_id: u64) -> Result<()> {
            let mut proposal = self.proposals.get(proposal_id).unwrap();
            let curr_voting_result = self.voting_result.get(proposal_id).unwrap();
            let project_address = self.project_address;

            let project_instance: ProjectRef =
                ink::env::call::FromAccountId::from_account_id(project_address);

            if self.env().block_timestamp() > proposal.proposal_span.end_time {
                // Implement better voting result algo
                let total_voters = project_instance.total_members();
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
                            // call execute function to execute the call from here
                        } else {
                            proposal.status = Status::Rejected;
                        }
                        self.proposals.insert(proposal.proposal_id, &proposal);
                        return Ok(());
                    }
                    _ => {}
                }
            }
            Ok(())
        }

        // passed proposal will get executed
        // can be called by anyone
        // call respective functions to execute the proposal
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u64) -> Result<()> {
            /*
                1.Check proposal status
                2. call respective functions according to proposaltype
            */
            // if voting complete and status is still unchanged(in the case of no vote after expiration)
            self.update_proposal_open_status(proposal_id).unwrap();
            let proposal = self.proposals.get(proposal_id);
            let project_address = self.project_address;

            let mut project_instance: ProjectRef =
                ink::env::call::FromAccountId::from_account_id(project_address);

            if let Some(mut proposal) = proposal {
                match proposal.status {
                    Status::Open => {
                        return Err(Error::ProposalRunning);
                    }
                    Status::Rejected => {
                        return Err(Error::ProposalRejected);
                    }
                    // contains code for executing passed proposals
                    Status::Passed => {
                        match proposal.proposal_type {
                            ProposalType::ProposeApproach => {
                                let approach =
                                    self.approach_info.get(proposal.proposal_id).unwrap();

                                project_instance
                                    .try_accept_proposed_approach(approach)
                                    .unwrap();
                                proposal.status = Status::Executed;
                                self.proposals.insert(proposal.proposal_id, &proposal);
                            }
                            ProposalType::ExtendDeadLine => {
                                let extension_info = self
                                    .deadline_extension_info
                                    .get(proposal.proposal_id)
                                    .unwrap();
                                project_instance
                                    .try_extend_task_deadline(
                                        extension_info.task_id,
                                        extension_info.deadline,
                                    )
                                    .unwrap();
                            } // _ => {
                              //     return Err(Error::UnAuthorized);
                              // }
                        }
                    }
                    Status::Executed => {
                        return Err(Error::ProposalAlreadyExecuted);
                    }
                }
            } else {
                return Err(Error::ProposalNotFound);
            }

            Ok(())
        }

        

        // Queries      Queries     Queries     Queries     Queries     Queries
        // Queries      Queries     Queries     Queries     Queries     Queries

        #[ink(message)]
        pub fn show_address(&self) -> AccountId {
            self.env().account_id()
        }
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
        pub fn get_approach_info(&self, proposal_id: u64) -> Result<AcceptedApproach> {
            let approach_info = self.approach_info.get(proposal_id);
            if let Some(info) = approach_info {
                return Ok(info);
            } else {
                return Err(Error::ApproachNotRequiredForThisProposal);
            }
        }
        #[ink(message)]
        pub fn get_deadline_extension_info(
            &self,
            proposal_id: u64,
        ) -> Result<DeadLineExtensionInfo> {
            let deadline_info = self.deadline_extension_info.get(proposal_id);
            if let Some(info) = deadline_info {
                return Ok(info);
            } else {
                return Err(Error::DeadLineInfoNotRequiredForThisProposal);
            }
        }
    }
    // Helper Functions     Helper Functions    Helper Functions    Helper Functions
    // Helper Functions     Helper Functions    Helper Functions    Helper Functions

    fn create_task_nodes(
        task_by_id: &mut Mapping<u16, Task>,
        task_list: &mut Vec<Task>,
        mut task: Task,
    ) {
        let mut start_after_map: Mapping<u16, bool> = Mapping::new();
        let user_start_after = task.clone().start_after;

        // if user_start_after.is_empty() {
        //     independent_tasks.push(task.clone().task_id);
        //     task_by_id.insert(task.task_id, &task.clone());
        // }

        user_start_after.iter().map(|id| {
            start_after_map.insert(id, &true);
        });

        let mut actual_start_after: Vec<u16> = Vec::new();
        for node in user_start_after {
            if !find_child_node(task_by_id, node, &start_after_map) {
                actual_start_after.push(node);
                let node_task = task_by_id.get(node);
                if let Some(mut node_task) = node_task {
                    node_task.children_task.push(task.task_id);
                    task_by_id.insert(node_task.task_id, &node_task);
                }
            }
        }
        task.start_after = actual_start_after;
        task_by_id.insert(task.task_id, &task.clone());
        task_list.push(task);
    }

    // bfs, check if any child is also in the element
    fn find_child_node(
        task_by_id: &Mapping<u16, Task>,
        node: u16,
        elements: &Mapping<u16, bool>,
    ) -> bool {
        let mut vis: Mapping<u16, bool> = Mapping::new();
        let mut queue: Vec<u16> = Vec::new();
        queue.push(node);
        vis.insert(node, &true);

        while !queue.is_empty() {
            let u = queue[0];
            queue.remove(0);
            let u_task_node = task_by_id.get(u);

            if let Some(u_task) = u_task_node {
                let neighbour = u_task.children_task;

                for id in neighbour {
                    if vis.get(id).is_none() {
                        if elements.get(id).is_some() {
                            return true;
                        } else {
                            vis.insert(id, &true);
                            queue.push(id);
                        }
                    }
                }
            }
        }
        false
    }

    fn update_voting_result(
        caller: AccountId,
        mut curr_voting_result: VotingStatus,
        vote: Vote,
    ) -> Result<VotingStatus> {
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

            Ok(curr_voting_result)
        }
    }
}
