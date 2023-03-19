#![cfg_attr(not(feature = "std"), no_std)]

pub use self::org::{Designation, Org, OrgRef};

#[ink::contract]
mod org {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use pproposal::PproposalRef;
    use project::{BusinessIdea, Error, ProjectRef, Result};

    #[derive(Debug, PartialEq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    //add different posts related to car company
    pub enum Designation {
        // BoardMember,
        Owner,
        Member, // change it later
        Manager,
        Secretariate,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    //Todo: projectwise rating storage
    pub struct MemberInfo {
        name: String,
        company_id: u16,
        designation: Designation, // add designation enums
        rating: Option<u8>,
    }
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct OutsiderInfo {
        name: String,
        account_address: AccountId,
    }

    #[ink(storage)]
    pub struct Org {
        org_name: String,
        org_owner: AccountId,
        secretariate: Vec<AccountId>, // store board members list(Added by proposal or something)
        secretariate_count: u64,
        member_count: u64,
        org_members: Mapping<AccountId, MemberInfo>, //also contains board members
        next_project_id: u32,
        /// store created (project address -> manager address)
        projects_manager: Mapping<AccountId, AccountId>,
        /// Store projects by id, to iterate to find all the projects
        project_list_by_id: Mapping<u32, AccountId>,
        // Don't need to store this because can call project fn using address
        // project: ProjectRef,
        // project_address: AccountId,
        project_codehash: Hash,
        project_proposal_codehash: Hash,
        proposal_address: Option<AccountId>,
    }

    impl Org {
        #[ink(constructor)]
        pub fn new(
            org_name: String,
            name: String,
            company_id: u16,
            rating: Option<u8>,
            project_codehash: Hash,
            project_proposal_codehash: Hash,
        ) -> Self {
            let creator_info = MemberInfo {
                name,
                company_id,
                designation: Designation::Owner,
                rating,
            };

            let creator = Self::env().caller();
            let mut org_members = Mapping::new();
            org_members.insert(creator, &creator_info);

            Self {
                org_name: org_name,
                org_owner: creator,
                secretariate: Vec::new(),
                secretariate_count: 0,
                member_count: 1,
                org_members,
                next_project_id: 1,
                projects_manager: Mapping::new(),
                project_list_by_id: Mapping::new(),
                project_codehash,
                project_proposal_codehash,
                proposal_address: None,
            }
        }

        #[ink(message)]
        pub fn update_proposal_address(
            &mut self,
            proposal_contract_address: AccountId,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller == self.org_owner {
                let already_stored = self.proposal_address;
                if let Some(_already) = already_stored {
                    return Err(Error::ProposalAddressAlreadyExist);
                } else {
                    self.proposal_address = Some(proposal_contract_address);
                }
            } else {
                return Err(Error::UnAuthorized);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn add_member_in_orgainisation(
            &mut self,
            member_address: AccountId,
            name: String,
            company_id: u16,
            rating: Option<u8>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller == self.org_owner {
                let member_info = MemberInfo {
                    name,
                    company_id,
                    designation: Designation::Member,
                    rating,
                };
                if let Some(_info) = self.org_members.get(member_address) {
                    return Err(Error::AlreadyMember);
                } else {
                    self.org_members.insert(member_address, &member_info);
                    self.member_count += 1;
                }
            } else {
                return Err(Error::UnAuthorized);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn add_secretariate(
            &mut self,
            secretariate_address: AccountId,
            name: String,
            company_id: u16,
            rating: Option<u8>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller == self.org_owner {
                let secretariate_info = MemberInfo {
                    name,
                    company_id,
                    designation: Designation::Secretariate,
                    rating,
                };
                if let Some(info) = self.org_members.get(secretariate_address) {
                    if info.designation == Designation::Secretariate {
                        return Err(Error::AlreadySecretariate);
                    } else {
                        self.org_members
                            .insert(secretariate_address, &secretariate_info);
                        self.secretariate_count += 1;
                        self.secretariate.push(secretariate_address);
                    }
                } else {
                    self.org_members
                        .insert(secretariate_address, &secretariate_info);
                    self.member_count += 1;
                    self.secretariate_count += 1;
                    self.secretariate.push(secretariate_address);
                }
            } else {
                return Err(Error::UnAuthorized);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn create_project(
            &mut self,
            title: String,
            info_list: Vec<String>,
            fund_allocated: u128, // store it in project
            strength: u64,
            proposer: AccountId,
        ) -> Result<u32> {
            // caller should be proposal contract
            let creator = self.env().caller();
            let id = self.next_project_id;
            let proposal_address = self.proposal_address;

            if let Some(proposal_address) = proposal_address {
                if creator == proposal_address {
                    let version: u8 = 99; // for random salt creation
                    let salt = version.to_be_bytes();

                    // check if it checks ASTAR token balance
                    let organisation_balance = self.env().balance();
                    if organisation_balance < fund_allocated {
                        return Err(Error::InsufficientFundInOrganisation);
                    }
                    let business_idea = BusinessIdea {
                        title,
                        info_count: info_list.len() as u128,
                        info_list,
                    };

                    let mut project_instance =
                        ProjectRef::new(id, business_idea, creator, fund_allocated, strength,proposal_address)
                            .endowment(fund_allocated)
                            .code_hash(self.project_codehash)
                            .salt_bytes(salt)
                            .instantiate();
                    let project_address = project_instance.show_address();

                    let pprosal_endowment: u128 = 0;
                    let project_proposal_instance = PproposalRef::new(project_address)
                        .endowment(pprosal_endowment)
                        .code_hash(self.project_proposal_codehash)
                        .salt_bytes(salt)
                        .instantiate();
                    let project_proposal_address = project_proposal_instance.show_address();

                    project_instance.update_proposal_address(project_proposal_address);

                    self.projects_manager.insert(project_address, &proposer);
                    self.project_list_by_id.insert(id, &project_address);
                    self.next_project_id += 1;

                    Ok(id)
                } else {
                    return Err(Error::UnAuthorized);
                }
            } else {
                return Err(Error::ProposalAddressNotFound);
            }
        }

        /// call to show interest in project
        #[ink(message)]
        pub fn apply_for_project(
            &mut self,
            project_id: u32,
            outsider: Option<OutsiderInfo>,
            resume_url: String,
        ) -> Result<()> {
            // check caller is member of org
            let caller = self.env().caller();

            if let Some(info) = self.org_members.get(caller) {
                if let Some(address) = self.project_list_by_id.get(project_id) {
                    let mut project_instance: ProjectRef =
                        ink::env::call::FromAccountId::from_account_id(address);
                    if let Some(outsider_info) = outsider {
                        // Handle error
                        let response = project_instance.try_update_interested_list(
                            outsider_info.account_address,
                            outsider_info.name,
                            resume_url,
                        );
                        match response {
                            Ok(()) => return Ok(()),
                            Err(er) => return Err(er),
                        }
                    } else {
                        let response =
                            project_instance.try_update_interested_list(caller, info.name, resume_url);
                        match response {
                            Ok(()) => return Ok(()),
                            Err(er) => return Err(er),
                        }
                    }
                } else {
                    return Err(Error::ProjectNotFound);
                }
            }

            Ok(())
        }

        // IMPORTANT IMPORTANT IMPORTANT IMPORTANT
        // only being called when proposal passed for fund
        #[ink(message)]
        pub fn transfer_fund(&mut self, id: u32, amount: u128) -> Result<()> {
            // check boundary contions and also caller should be proposal address only
            if Some(self.env().caller()) == self.proposal_address {
                let project_address = self.project_list_by_id.get(id);
                if let Some(addr) = project_address {
                    if self.env().balance() < amount {
                        return Err(Error::InsufficientFundInOrganisation)
                    }
                    let transfer_response = self.env().transfer(addr, amount);
                    match transfer_response {
                        Ok(_) => return Ok(()),
                        _ => return Err(Error::CannotTransferFund),
                    }
                } else {
                    return Err(Error::ProjectNotFound);
                }
            } else {
                return Err(Error::UnAuthorized);
            }
        }

        // QUERIES  QUERIES QUERIES QUERIES QUERIES QUERIES  QUERIES QUERIES QUERIES QUERIES
        // QUERIES  QUERIES QUERIES QUERIES QUERIES QUERIES  QUERIES QUERIES QUERIES QUERIES
        // QUERIES  QUERIES QUERIES QUERIES QUERIES QUERIES  QUERIES QUERIES QUERIES QUERIES

        #[ink(message)]
        pub fn get_project_address(&self, id: u32) -> Result<AccountId> {
            let project_address = self.project_list_by_id.get(id);
            if let Some(addr) = project_address {
                Ok(addr)
            } else {
                return Err(Error::ProjectNotFound);
            }
        }

        #[ink(message)]
        pub fn check_member(&self, address: AccountId,secretariate: bool) -> Result<bool> {
            let member_info = self.org_members.get(address);
            if let Some(info) = member_info{
                if secretariate {
                    if info.designation == Designation::Secretariate {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }else{
                    Ok(true)
                }
            }else{
                return Err(Error::MemberNotFound)
            }
        }
        // send list of all secretariates
        #[ink(message)]
        pub fn secretariate_list(&self) -> Vec<AccountId> {
            self.secretariate.clone()
        }
        #[ink(message)]
        pub fn total_projects(&self) -> u32 {
            self.next_project_id - 1
        }
        #[ink(message)]
        pub fn total_secretariate(&self) -> u64 {
            self.secretariate_count.clone()
        }
        #[ink(message)]
        pub fn total_members(&self) -> u64 {
            self.member_count.clone()
        }
        #[ink(message)]
        pub fn get_member_info(&self, member_address: AccountId) -> Result<MemberInfo> {
            let member_info = self.org_members.get(member_address);
            if let Some(info) = member_info {
                Ok(info)
            } else {
                return Err(Error::MemberNotFound);
            }
        }
        #[ink(message)]
        pub fn get_org_owner_info(&self) -> MemberInfo {
            self.org_members.get(self.org_owner).unwrap()
        }
        #[ink(message)]
        pub fn get_org_name(&self) -> String {
            self.org_name.clone()
        }
    }
}
