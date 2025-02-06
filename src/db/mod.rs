use errors::DBError;
use futures::TryStreamExt;
use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client, Collection};

pub mod db_groups;
pub mod errors;
pub mod m_group;

#[derive(Clone, Debug)]
pub struct DB {
    pub group_collection: Collection<Document>,
    pub expense_collection: Collection<Document>,
    pub repeating_expense_collection: Collection<Document>,
    pub expense_category_collection: Collection<Document>,
    pub budget_collection: Collection<Document>,
    pub budget_category_collection: Collection<Document>,
    pub budget_view_collection: Collection<Document>,
    pub saving_goal_collection: Collection<Document>,
}

type Result<T> = std::result::Result<T, DBError>;

// Basis
impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let group_collection = database.collection::<Document>("groups");
        let expense_collection = database.collection::<Document>("expenses");
        let repeating_expense_collection = database.collection::<Document>("repeating_expenses");
        let expense_category_collection = database.collection::<Document>("expense_categories");
        let budget_collection = database.collection::<Document>("budgets");
        let budget_category_collection = database.collection::<Document>("budget_categories");
        let budget_view_collection = database.collection::<Document>("budget_views");
        let saving_goal_collection = database.collection::<Document>("saving_goals");

        tracing::info!("Database connection established successfully.");

        Ok(Self {
            group_collection,
            expense_collection,
            repeating_expense_collection,
            expense_category_collection,
            budget_collection,
            budget_category_collection,
            budget_view_collection,
            saving_goal_collection,
        })
    }
}

// Groups
impl DB {
    pub async fn list_groups(&self) -> Result<Vec<m_group::ListingGroup>> {
        let mut cursor = self.group_collection.find(Document::new()).await?;
        let mut listing_groups = vec![];
        while let Some(result) = cursor.try_next().await? {
            if let Ok(group) =
                bson::from_bson::<m_group::ListingGroup>(bson::Bson::Document(result))
            {
                listing_groups.push(m_group::ListingGroup {
                    id: group.id,
                    name: group.name,
                    status: group.status,
                    group_type: group.group_type,
                });
            }
        }
        Ok(listing_groups)
    }

    pub async fn insert_one(&self, group: m_group::Group) -> Result<()> {
        self.group_collection
            .insert_one(bson::to_document(&group)?)
            .await?;
        Ok(())
    }
}
