#![cfg_attr(not(feature = "std"), no_std)]

pub use ink_lang as ink;

#[ink::contract]
mod quiz_winner {

    #[ink(storage)]
    pub struct QuizWinner {
        correct_answer: String,
        participants: ink_prelude::collections::HashMap<AccountId, String>,
        reward: Balance,
        winner: Option<AccountId>,
    }

    impl QuizWinner {
        #[ink(constructor)]
        pub fn new(correct_answer: String, reward: Balance) -> Self {
            Self {
                correct_answer,
                participants: ink_prelude::collections::HashMap::new(),
                reward,
                winner: None,
            }
        }

        #[ink(message)]
        pub fn submit_answer(&mut self, answer: String) {
            let caller = self.env().caller();
            // Store the participant's answer
            self.participants.insert(caller, answer);
        }

        #[ink(message)]
        pub fn check_winner(&mut self) {
            if self.winner.is_some() {
                return; // Winner already determined
            }

            // Iterate through all participants to find the winner
            for (account, answer) in self.participants.iter() {
                if answer == &self.correct_answer {
                    self.winner = Some(*account);
                    break;
                }
            }
        }

        #[ink(message)]
        pub fn claim_reward(&mut self) -> Result<(), &'static str> {
            let caller = self.env().caller();

            if let Some(winner) = self.winner {
                if caller == winner {
                    // Transfer the reward to the winner
                    let _ = self.env().transfer(caller, self.reward);
                    Ok(())
                } else {
                    Err("You are not the winner!")
                }
            } else {
                Err("No winner determined yet!")
            }
        }

        #[ink(message)]
        pub fn get_winner(&self) -> Option<AccountId> {
            self.winner
        }

        #[ink(message)]
        pub fn get_correct_answer(&self) -> String {
            self.correct_answer.clone()
        }

        #[ink(message)]
        pub fn get_participant_answer(&self, account: AccountId) -> Option<String> {
            self.participants.get(&account).cloned()
        }
    }
}
