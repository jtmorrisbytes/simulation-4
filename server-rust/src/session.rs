/* store a value, using a unique, randomly generated key
  the store should be able to hold any type.

  it would be nice if it could use any backend,
  but it would be cool to start with a HashMap

  - the session should be able to expire after a certian elapsed time
  - it would be nice if we could update the session object directly, and
    have the updated values be reflected on whatever backend being used,
    but an update fn would be okay


  impl details:

  it would be nice for the user to be able
  to have several immutable references
  to the session store,
  but one mutable reference

  the user should aquire the mutable reference
  using a known function, then the data should
  be updated when the mutable reference ends

  Type K:
  
  Type K should be considered the unique identifier that
  should be used to query the backend for value V

  type V:
  
  Type V should be considered a generic, with any number of fields,
  or values


  if v is only known at runtime, dyn Trait???
*/

use std::collections::{HashMap};

// the example struct
#[derive(Debug)]
pub struct UserSession {
  user_id:i32
}
/* the session struct
   where K is the type of the session key
   and V is the type of the struct 
*/
// #[derive(Debug)]

pub struct HashMapBackend<V>(HashMap<String,Session<String,V>>);

pub trait Backend {
  fn new();
  fn with_key();
}
impl <V> Backend for HashMapBackend<V>{
  fn new(){}
  fn with_key(){}
}


// impl<B,V> SessionContainer<HashMap<String,Session<String,V>>>{
//   fn new(data:V) -> SessionContainer<HashMap<String,Session<String,V>>>{
//     let backend: HashMap<String,V> = HashMap::new();
//     let key:String = "some unique key".to_string();
//     let session : Session<String,V> = Session::with_key(key, data);
//     backend.insert(key,data);
//     SessionContainer {
//       backend:backend
//     }
//   }
// }



#[derive(Debug)]
pub struct Session<K,V> {
  session_id: K,
  // session_id:String
  /* when was the session created,
     according to the backend */
  // created: ? undecided type
  /* how long should the session last? */
  // max_age ? undecided type, most likely in 
  //           millis or nanos
  /* the data associated with the session.
     the data should not be mutated directly,
     but the user should be able to get
     a mutable reference to data that should
     enable the backend to get notified
     that an update potentially occurred.
  */
  data: V
  // backend:B
}
/* for the frontend, you should be able to get
  a mutable and immutable reference to the session
*/
pub trait ReadSession {
  // get an immutable reference to the session
  fn get();
  /* get a mutable reference to the session,
     ideally updating when the mutable reference
     drops
  */
  fn get_mut();
}
/*
  for the backend, other than choosing which backend
  it would be nice if most implementation
  details were taken care of, like explicitly
  creating, updating or checking if the session is valid
*/
trait UpdateSession {
  fn update();
}
trait CreateSession {
  fn new();
}



impl<V> Session<String,V> {
  fn new(data:V)->Session<String,V>{
    let key:String = "Some random session key".to_string();
    Session {session_id:key,data:data}
  }
  fn with_key(key:String,data:V)->Session<String,V>{
    Session {
      session_id:key,
      data:data
    }
  }
}



#[test]
pub fn test_session(){

  let sess_container:SessionContainer<String,UserSession> = SessionContainer::new(UserSession{user_id:0});

  println!("Create session result: {:?}",sess_container)
}