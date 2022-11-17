
use std::{io, collections::HashMap, hash::Hash}; 

fn main() {

    println!("please enter coin_name");
    let mut buffer = String::new() ; 

    std::io::stdin().read_line(&mut buffer).expect("msg"); 

    let res = CoinsbasePrices::fetch_responce(); 


    let mut my_coin_storage:HashMap<String,CoinsPrices> = HashMap::new() ; 

    let trim_buffer = buffer.trim().to_owned();

    println!("{}", trim_buffer);

    let mut coin_storage  = CoinsbasePrices::new(res, my_coin_storage.clone()); 

    //  

    let coin_price = CoinsbasePrices::fetch_coin(trim_buffer, &mut coin_storage.coins).unwrap();

    dbg!(coin_price);



    
   
}

use serde_derive::{Deserialize, Serialize};
    
#[derive(Debug, Deserialize, Serialize,Clone)]

struct CoinsPrices {
id: String,
name: String,
min_size: String,
}


#[derive(Debug, Deserialize, Serialize,Clone)]
struct CoinsbasePrices {
data: Vec<CoinsPrices>,

} 

#[derive(Debug, Deserialize, Serialize,Clone)]

struct  CoinStorage {
    coins:HashMap<String, CoinsPrices>
}

impl CoinStorage{

    fn new( ) -> Self{

        Self {coins: HashMap::new()}
    }
}


impl CoinsbasePrices {

    // self -> instance Coinbaseprices
    // Self -> type

    // Associated fn 
    fn fetch_coin( coin_name:String, storage:&mut HashMap<String,CoinsPrices>) ->Option<&CoinsPrices> {

        
        if storage.contains_key(&coin_name) {

            println!("key is there");
                 return Some(&storage.get(&coin_name).unwrap());
        }else {
            println!("key is not there");

            return None;
    
        }
    
        
    
    }
    
    

    fn new(responce:String, mut map_storage: HashMap<String,CoinsPrices>) -> CoinStorage {

        let coin_base_price: CoinsbasePrices = serde_json::from_str(responce.as_str()).unwrap();
        let storage = CoinStorage::new(); 

        for coin_prices in coin_base_price.data.clone(){

            let key = coin_prices.name.clone() ; 
            map_storage.insert(key, coin_prices.clone());

        }
        
        // println!("{:?}", map_storage); 
         CoinStorage { coins: map_storage }

    }





    //  Methods
    fn print_coin_prices(&self) {

        println!("{:?}", self.data)
    }

    fn fetch_responce() -> String {

        let responce = ureq::get("https://api.coinbase.com/v2/currencies")
    .call()
    .unwrap()
    .into_string()
    .unwrap();

    responce

    }

}

#[derive(Debug)]
enum Result {

    Ok(i32), 
    Err(String)
}

impl Result {

    fn new() ->Self {

        Self::Err("ti".to_owned())
    } 

    fn print_result(&self)  {
        println!("{:?}", self);
    }
}

fn ret_coin(coin:&str, database: CoinsbasePrices ) -> CoinsPrices {


    let data = database.data.clone(); 

    for user_coin in data {

        if coin == user_coin.name.as_str() {
            
          return    user_coin;
        } 
    }

    CoinsPrices { id: "()".to_owned(), name: "".to_owned(), min_size: "()".to_string() }
    
}
