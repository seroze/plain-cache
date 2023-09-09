

/** ```rust,ignore 
 * Of course this is a library but we can have main too 
 */

 /***
  * asdfa sdf adf a
  */

mod cache; 

//   use crate::cache; 
use cache::Cache; 

fn main(){
    println!("Testing main");

    let mut cache = Cache::new(100);

    // insert key, value into cache 
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    cache.insert("key3", "value3");

    // get values from cache 
    println!("Value for key 'key1': {:?}", cache.get(&"key1"));
    println!("Value for key 'key2': {:?}", cache.get(&"key2"));
    println!("Value for key 'key3': {:?}", cache.get(&"key3"));

    // print cache len 
    println!("Cache len: {}", cache.len());

    // remove key from cache 
    cache.remove(&"key2");

    // get updated values from cache 
    println!("Value for key 'key1': {:?}", cache.get(&"key1"));
    println!("Value for key 'key2': {:?}", cache.get(&"key2"));
    println!("Value for key 'key3': {:?}", cache.get(&"key3"));
}