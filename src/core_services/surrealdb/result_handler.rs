use serde::de::DeserializeOwned;
use surrealdb::opt::RecordId;
use surrealdb::Response;
use crate::entities::errors::Errors;

pub struct UniformSurrealResult<T> (pub Vec<T>) where T: Clone + Sized + Into<RecordId>;

impl<T> TryFrom<&mut Response> for UniformSurrealResult<T> where T: Clone + Sized + Into<RecordId> + DeserializeOwned {
    type Error = Errors;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let mut result: Vec<T> = vec![];
        for i in 0 .. value.num_statements() {
            let query_result: Option<T> = value.take(i).map_err(|e| Errors::InternalServerError(
                format!("Error happen while taking the result for statement {}: {:?}", i, e)))?;
            if let None = query_result {
                return Err(Errors::InternalServerError(format!("The query statement {} has response empty", &i)));
            }

            result.push(query_result.unwrap());
        }

        Ok(Self(result))
    }
}
