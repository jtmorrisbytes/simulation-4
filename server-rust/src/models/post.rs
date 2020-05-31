pub struct Post {
    pub id: u32,
    //author id will reference User.id
    pub author_id: String,
    pub title: String,
    pub img: String,
    pub content: String,
}
