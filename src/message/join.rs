use actix_web::actix::Message;

pub struct Join {
  pub id: usize,
  pub name: String,
  pub user: String,
}

impl Message for Join {
  type Result = ();
}