#![cfg_attr(not(feature = "std"), no_std)]

pub use self::project::{AcceptedApproach, BusinessIdea, Project, ProjectRef, Task,Result,Error};

#[ink::contract]
pub mod project {

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;

    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        // organisation err
        MemberNotFound,
        UnAuthorized,
        CannotInitialiseProject,
        CannotTransferFund,
        AlreadySecretariate,
        AlreadyMember,
        ProposalAddressNotFound,
        ProposalAddressAlreadyExist,
        ProjectNotFound,
        InsufficientFundInOrganisation,
        //project
        AlreadyApplied,
        ApproachAlreadyAccepted,
        NotAMember,
        ThisTaskIsNotAssignedToYou,
        TaskInReview,
        TaskCompleted,
        TaskAlreadyAssigned,
        TaskNotFound,
        AlreadyReviewed,
        // Proposal errors
        AlreadyVoted,
        ProposalClosed,
        ProposalRejected,
        ProposalAlreadyExecuted,
        ProposalNotFound,
        ProposalRunning,
        ApproachNotDefined,
        DeadLineNotDefined,
        IncorrectProposalType,
        CannotStartProjectBeforeVoting,
        ApproachNotRequiredForThisProposal,
        TaskDeadlineCannotCrossPojectDeadLine,
        DeadLineInfoNotRequiredForThisProposal,
        // org proposal
        ProjectInfoNotFound,
        QuorumInfoNotFound,
        SpanInfoNotFound,
        FundInfoNotFound,
        InvalidProjectInfo,
        ProjectIdNotFound,
    }
    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum CommunityFormationStatus {
        Running,
        Closed,
    }

    #[derive(Debug, PartialEq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum TaskStatus {
        NotYetStarted,
        Running(u8), // store progress (0 to 100)
        InReview,
        Completed
    }

    #[derive(Debug, Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct BusinessIdea {
        pub title: String,
        // strore total count of keypoints in the info
        pub info_count: u128,
        pub info_list: Vec<String>,
    }

    /*
        1. Proposal contract will pass task id,start after
    */

    #[derive(Debug, Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Task {
        pub task_id: u16,
        /// contains the task ids, this task will start after the completion of these tasks
        pub start_after: Vec<u16>, // filter it
        pub start_time: Option<Timestamp>,
        pub end_time: Timestamp,
        pub status: TaskStatus, // will be updated by assigned member
        pub performance_rating: Option<u8>,
        pub details: Vec<String>,
        pub children_task: Vec<u16>,
    }

    #[derive(Debug, Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    /// this is the approach accepted after voting on proposals
    pub struct AcceptedApproach {
        pub project_start_date: Timestamp,
        pub project_completion_date: Timestamp,
        pub tasks: Vec<Task>, // This task should contain the task in order -> task[j] should only depend on task[i] where i 0..j-1
    }

    #[derive(Debug, Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct DeadLine {
        pub project_start_date: Timestamp,
        pub project_completion_date: Timestamp,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    // keeps basic work info of insider and outsider employees
    pub struct MemberInfo {
        name: String,
        address: AccountId,
        resume_url: String,
    }

    #[ink(storage)]
    pub struct Project {
        ///unique id of the project
        id: u32,
        project_info: BusinessIdea,
        /// manager who proposed business plan(add change function later)
        manager: AccountId,
        curr_member_count: u64,
        /// store list of address of all members accepted in the project
        member_list: Vec<AccountId>,
        members_info: Mapping<AccountId, MemberInfo>,

        // /// store some id of the members(Maybe we can further convert it into some gov id)
        // members_id: Mapping<AccountId, u32>,
        /// stores the task assigned to the person(address -> vec of ids of all the tasks assigned)
        members_assigned_task_list: Mapping<AccountId, Vec<u16>>,
        members_in_task: Mapping<u16, Vec<AccountId>>, // take vote only from eligible members

        total_fund_allocated: u128,
        strength: u64, // change it to members required
        organisation_contract: AccountId,
        organisation_proposal_contract: AccountId,
        community_formation_status: CommunityFormationStatus,

        // mamanger will check and let them in the project(Make it more appropriate)
        interested_members: Vec<AccountId>,
        interested_members_info: Mapping<AccountId, MemberInfo>,

        // approach: Option<AcceptedApproach>,
        deadline: Option<DeadLine>,
        task_by_id: Mapping<u16, Task>,
        root_tasks: Vec<u16>,

        review_tasks: Vec<u16>,

        project_proposal_address: Option<AccountId>,
    }

    impl Project {
        // checkout accountid(you will be providing string address)
        #[ink(constructor)]
        #[ink(payable)] // It should not be here by ink documentation but without it ,not receiving tokens
        pub fn new(
            id: u32,
            business_idea: BusinessIdea,
            manager: AccountId,
            fund_allocated: u128,
            strength: u64,
            proposal_contract: AccountId
        ) -> Self {
            Project {
                id,
                project_info: business_idea,
                manager,
                curr_member_count: 0,
                member_list: Vec::new(),
                // members_id: Mapping::new(),
                members_info: Mapping::new(),
                members_assigned_task_list: Mapping::new(),
                members_in_task: Mapping::new(),
                total_fund_allocated:fund_allocated,
                strength,
                organisation_contract: Self::env().caller(),
                organisation_proposal_contract: proposal_contract,
                community_formation_status: CommunityFormationStatus::Running,
                interested_members: Vec::new(),
                interested_members_info: Mapping::new(),
                deadline: None,
                task_by_id: Mapping::new(),
                root_tasks: Vec::new(),
                review_tasks: Vec::new(),
                project_proposal_address: None,
            }
        }

        // called during project creation in organisation contract
        #[ink(message)]
        pub fn update_proposal_address(&mut self,proposal_contract_address: AccountId,) {
            let caller = self.env().caller();
            if caller == self.organisation_contract {
                self.project_proposal_address = Some(proposal_contract_address);
            }
        }
        // on asking for fund in between
        #[ink(message)]
        pub fn update_fund_allocated(&mut self,amount: u128) {
            let caller = self.env().caller();
            if caller == self.organisation_proposal_contract {
                self.total_fund_allocated = self.total_fund_allocated.saturating_add(amount);
            }
        }

        // called by organisation
        #[ink(message)]
        pub fn try_update_interested_list(
            &mut self,
            address: AccountId,
            name: String,
            resume_url: String,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller == self.organisation_contract {
                let member_info = MemberInfo {
                    name,
                    address,
                    resume_url,
                };
                if let Some(_info) = self.interested_members_info.get(address) {
                    return Err(Error::AlreadyApplied);
                }
                if let Some(_info) = self.members_info.get(address) {
                    return Err(Error::AlreadyMember);
                }
                self.interested_members.push(address);
                self.interested_members_info.insert(address, &member_info);
            } else {
                return Err(Error::UnAuthorized);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn try_add_member(&mut self, interested_member_address: AccountId) -> Result<()> {
            let caller = self.env().caller();

            if caller == self.manager && self.curr_member_count < self.strength {
                let member_info = self
                    .interested_members_info
                    .get(interested_member_address)
                    .unwrap();

                self.interested_members_info
                    .remove(interested_member_address);

                let index = self
                    .interested_members
                    .iter()
                    .position(|id| *id == interested_member_address)
                    .unwrap();
                self.interested_members.remove(index);

                self.member_list.push(interested_member_address);
                self.members_info
                    .insert(interested_member_address, &member_info);
            } else {
                return Err(Error::UnAuthorized);
            }

            Ok(())
        }

        
        // Discuss : how will the passed approach for project should be accepted.
        // Directly from proposal dao or manager will manually accept
        #[ink(message)]
        pub fn try_accept_proposed_approach(&mut self, approach: AcceptedApproach) -> Result<()> {
            let caller = self.env().caller();
            if let Some(_) = self.deadline {
                return Err(Error::ApproachAlreadyAccepted);
            }

            if Some(caller) == self.project_proposal_address {
                // self.approach = Some(approach.clone());
                let deadline = DeadLine{
                    project_start_date: approach.project_start_date,
                    project_completion_date: approach.project_completion_date
                };
                self.deadline = Some(deadline);

                for task in approach.tasks {
                    self.task_by_id.insert(task.task_id, &task);
                    if task.start_after.len() == 0 {
                        self.root_tasks.push(task.task_id);
                    }
                }
            } else {
                return Err(Error::UnAuthorized);
            }
            Ok(())
        }
        
        // we can provide one member one task or one member many task policy
        // one member many task
        #[ink(message)]
        pub fn try_assign_task_to_member(&mut self,task_id: u16, member_address: AccountId) -> Result<()> {

            let caller = self.env().caller();

            if caller != self.manager {
                return Err(Error::UnAuthorized)
            }
            let assigned_tasks = self.members_assigned_task_list.get(member_address);

            if let Some(mut assigned_tasks) = assigned_tasks {
                let already_assigned = assigned_tasks.iter().find(|id| **id == task_id);
                if already_assigned.is_some() {
                    return Err(Error::TaskAlreadyAssigned)
                }
                assigned_tasks.push(task_id);
                self.members_assigned_task_list.insert(member_address,&assigned_tasks);

                let mut members_in_task = self.members_in_task.get(task_id).unwrap();
                members_in_task.push(member_address);
                self.members_in_task.insert(task_id,&members_in_task);

            }
            
            Ok(())
        }

        #[ink(message)]
        pub fn try_update_task_progress(&mut self,task_id: u16, progress: u8) -> Result<()> {

            let caller = self.env().caller();

            let assigned_tasks = self.members_assigned_task_list.get(caller);

            if let Some(assigned_task) = assigned_tasks {
                let id = assigned_task.iter().find(|x| **x == task_id);
                if id.is_none() {
                    return Err(Error::ThisTaskIsNotAssignedToYou)
                }else{
                    let mut task = self.task_by_id.get(task_id).unwrap();
                    if task.status == TaskStatus::InReview {
                        return Err(Error::TaskInReview)
                    }else if task.status == TaskStatus::Completed {
                        return Err(Error::TaskCompleted)
                    }else{
                        if progress >= 100 {
                            task.status = TaskStatus::InReview;
                            self.review_tasks.push(task_id);
                        }else{
                            task.status = TaskStatus::Running(progress);
                        }
                        self.task_by_id.insert(task_id, &task);
                    }
                }
            }else{
                return Err(Error::ThisTaskIsNotAssignedToYou)
            }

            Ok(())
        }
        // manager will review the task
        #[ink(message)]
        pub fn review_task_and_rate(&mut self,task_id: u16,rating: u8) -> Result<()> {

            let caller = self.env().caller();
            if caller != self.manager {
                return Err(Error::UnAuthorized)
            }
            let mut task = self.task_by_id.get(task_id).unwrap();
            
            if task.status == TaskStatus::Completed {
                return Err(Error::AlreadyReviewed)
            }

            if task.status == TaskStatus::InReview {
                task.performance_rating = Some(rating);
                self.task_by_id.insert(task_id,&task);
                task.status = TaskStatus::Completed;
            }

            Ok(())
        }


        // check curr task and all children tasks(and their children) accordingly. These are the possible voters for deadline extension
        #[ink(message)]
        pub fn try_check_voters_for_deadline_extension(&self,task_id: u16, member: AccountId, check_task_only: bool) -> Result<bool> {
            
            let members = self.members_in_task.get(task_id);
            let mut _is_possible = false;

            if let Some(mut members) = members {
                if !check_task_only {
                    let children_task = self.task_by_id.get(task_id).unwrap().children_task;
    
                    for child in children_task {
                        if let Some(mut child_members) = self.members_in_task.get(child) {
                            members.append(&mut child_members);
                        }
                    }
                }
                if members.iter().find(|x| **x == member).is_some() {
                    Ok(true)
                }else{
                    Ok(false)
                }
            }else{
                return Err(Error::TaskNotFound)
            }
        }
        

        #[ink(message)]
        pub fn is_possible_deadline(&self,task_id: u16, completion_time: Timestamp) -> Result<bool> {

            let task = self.task_by_id.get(task_id);
            let mut _is_possible = false;
            if let Some(task) = task {
                _is_possible = self.check_neighbour_deadline(task,completion_time);
            }else{
                return Err(Error::TaskNotFound)
            }

            Ok(_is_possible)
        }
        // query all the children nodes if endtime of the task is not crossing the end time of neighbours
        // otherwise task extension is not possible,
        fn check_neighbour_deadline(&self,task: Task,endtime: Timestamp) -> bool {
            let children_task = task.children_task;
            let mut is_possible = true;
            for child in children_task {
                let child_task = self.task_by_id.get(child).unwrap();
                if child_task.end_time <= endtime {
                    is_possible = false;
                }
            }
            is_possible
        }
        
        #[ink(message)]
        pub fn try_extend_task_deadline(&mut self,task_id: u16, completion_time: Timestamp) -> Result<()> {

            let caller = self.env().caller();
            // everything already checked in proposal
            if Some(caller) == self.project_proposal_address {
                let mut task = self.task_by_id.get(task_id).unwrap();
                task.end_time = completion_time;
                self.task_by_id.insert(task_id, &task);
                self.build_deadline_graph(task_id);

            } else {
                return Err(Error::UnAuthorized);
            }
            Ok(())
        }

        /*
            1. it will take the task node and update all the subtasks
        */
        fn build_deadline_graph(&mut self, root_task: u16){

            // let mut vis: Mapping<u16, bool> = Mapping::new();
            let mut queue: Vec<u16> = Vec::new();
            queue.push(root_task);
            // vis.insert(root_task, &true);

            while !queue.is_empty() {
                let u = queue[0];
                queue.remove(0);
                let u_task_node = self.task_by_id.get(u);

                if let Some(u_task) = u_task_node {
                    // make all start time of neighbour node to the end time of the parent node
                    let start_time = u_task.end_time;
                    let neighbour = u_task.children_task;

                    for id in neighbour {
                        let child_task = self.task_by_id.get(id);
                        if let Some(mut child_task) = child_task {
                            if child_task.start_time.is_none() || child_task.start_time < Some(start_time) {
                                child_task.start_time = Some(start_time);
                                self.task_by_id.insert(child_task.task_id, &child_task);
                                queue.push(child_task.task_id);
                            }
                        }
                    }
                }
            }
        }

        // Queries      Queries     Queries    Queries   Queries      Queries      Queries       Queries
        // Queries      Queries     Queries    Queries   Queries      Queries      Queries       Queries

        #[ink(message)]
        pub fn show_address(&self) -> AccountId {
            self.env().account_id()
        }

        #[ink(message)]
        pub fn show_interested_members_list(&self) -> Vec<AccountId> {
            self.interested_members.clone()
        }

        #[ink(message)]
        pub fn show_members_list(&self) -> Vec<AccountId> {
            self.member_list.clone()
        }

        #[ink(message)]
        pub fn check_member(&self, address: AccountId) -> bool {
            let member_info = self.members_info.get(address);
            if member_info.is_some() {
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn total_members(&self) -> u64 {
            self.curr_member_count.clone()
        }
        #[ink(message)]
        pub fn get_task_members(&self,task_id: u16) -> Vec<AccountId> {
            let assigned_members_in_the_task = self.members_in_task.get(task_id);
            if let Some(assigned_members) = assigned_members_in_the_task {
                assigned_members
            }else{
                [].to_vec()
            }
        }
        #[ink(message)]
        pub fn get_review_task(&self) -> Vec<u16> {
            self.review_tasks.clone()
        }

    }
}
