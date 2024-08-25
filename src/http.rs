use serde::Serialize;

type ResponseData<T: Serialize> = Option<T>;

#[derive(Serialize)]
pub struct HttpResponse<T> {
  message: String,
  code: String,
  data: ResponseData<T>,
}

impl<T> HttpResponse<T>
where
  T: Serialize,
{
  pub fn new(message: String, code: String, data: ResponseData<T>) -> Self {
    Self { message, code, data }
  }
}