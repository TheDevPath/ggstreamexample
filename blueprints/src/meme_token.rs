use scrypto::prelude::*;

blueprint! {
 struct MemeToken {
 }

 impl MemeToken {
   pub fn instantiate_meme_token() -> ComponentAddress {
     Self {}
     .instantiate()
     .globalize()
   }
 }
}
