pub struct Post {
  id:u32,
  //author id will reference User.id
  author_id: String,
  title: String,
  img: String,
  content:String,
}