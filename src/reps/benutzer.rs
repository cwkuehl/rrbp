
pub async fn get_all(con: &DbCon) -> Result<Vec<ContactRequest>, Error> {
  let result = con.run(|c| {
      requests::table
          .load::<ContactRequest>(c)
  }).await;
  return result;
}

