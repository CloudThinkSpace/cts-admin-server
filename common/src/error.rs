use sea_orm::DbErr;
use tracing::info;

pub enum CtsError {
    Sql(String),
    Server(String),
    Request(String),
    DataAlreadyExists(String),
    DataNotExists(String),
    Custom(String),
}

impl CtsError {
    pub fn into<T>(self) -> Result<T, CtsError> {
        match &self {
            CtsError::Sql(data) => {
                info!("SqlError:{}", data);
            }
            CtsError::Server(data) => {
                info!("Server:{}", data);
            }
            CtsError::Request(data) => {
                info!("RequestError:{}", data);
            }
            CtsError::Custom(data) => {
                info!("Custom:{}", data);
            }
            CtsError::DataAlreadyExists(data) => {
                info!("DataAlreadyExists:{}", data);
            }
            CtsError::DataNotExists(data) => {
                info!("DataAlreadyExists:{}", data);
            }
        }
        Err(self)
    }
}

impl From<DbErr> for CtsError {
    fn from(value: DbErr) -> Self {
        info!("{}", value.to_string());
        CtsError::Sql(value.to_string())
    }
}