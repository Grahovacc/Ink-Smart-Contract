#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod my_first_contract {
    #[ink(storage)]
    pub struct MyFirstContract {
        value: bool,
    }

    impl MyFirstContract {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
             Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let my_first_contract = MyFirstContract::default();
            assert_eq!(my_first_contract.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut my_first_contract = MyFirstContract::new(false);
            assert_eq!(my_first_contract.get(), false);
            my_first_contract.flip();
            assert_eq!(my_first_contract.get(), true);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = MyFirstContractRef::default();
            let contract = client 
                .instantiate("my_first_contract", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<MyFirstContract>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = MyFirstContractRef::new(false);
            let contract = client
                .instantiate("my_first_contract", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<MyFirstContract>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed ");

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
