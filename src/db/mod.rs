use bson::doc;
use bson::oid::ObjectId;
use errors::DBError;
use futures::TryStreamExt;
use m_group::CreateGroup;
use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client, Collection};

pub mod errors;
pub mod m_group;

#[derive(Clone, Debug)]
pub struct DB {
    group_collection: Collection<Document>,
    expense_collection: Collection<Document>,
    repeating_expense_collection: Collection<Document>,
    expense_category_collection: Collection<Document>,
    budget_collection: Collection<Document>,
    budget_category_collection: Collection<Document>,
    budget_view_collection: Collection<Document>,
    saving_goal_collection: Collection<Document>,
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
        let mut cursor = self
            .group_collection
            .find(doc! {"status": "Active"})
            .await?;
        let mut listing_groups = vec![];
        while let Some(result) = cursor.try_next().await? {
            if let Ok(group) =
                bson::from_bson::<m_group::ListingGroup>(bson::Bson::Document(result))
            {
                listing_groups.push(group);
            }
        }
        Ok(listing_groups)
    }

    pub async fn create_group(&self, create_group: CreateGroup) -> Result<String> {
        let group = m_group::BsonGroup {
            id: None,
            name: create_group.name,
            admins: create_group
                .admins
                .iter()
                .map(|id| ObjectId::parse_str(id).unwrap())
                .collect(),
            members: create_group
                .members
                .iter()
                .map(|id| ObjectId::parse_str(id).unwrap())
                .collect(),
            status: m_group::GroupStatus::Active,
            group_type: create_group.group_type,
            created_at: chrono::Utc::now().into(),
            updated_at: None,
            disbanded_at: None,
        };
        let res = self
            .group_collection
            .insert_one(bson::to_document(&group)?)
            .await?;
        Ok(res.inserted_id.as_object_id().unwrap().to_hex())
    }

    pub async fn get_group(&self, group_id: String) -> Result<m_group::JsonGroup> {
        let group_id = ObjectId::parse_str(&group_id).unwrap();
        let group = self
            .group_collection
            .find_one(doc! {"_id": group_id})
            .await?;
        match group {
            Some(group) => match bson::from_bson::<m_group::BsonGroup>(bson::Bson::Document(group))
            {
                Ok(group) => return Ok(group.into()),
                Err(e) => {
                    tracing::error!("Error deserializing group: {}", e);
                    return Err(DBError::DeserializationError(e));
                }
            },
            None => Err(DBError::NotFoundError(
                errors::ItemType::Group,
                group_id.to_hex(),
            )),
        }
    }

    pub async fn update_group(
        &self,
        group_id: String,
        group: m_group::UpdateGroup,
    ) -> Result<String> {
        match self.is_group_active(&group_id).await {
            Err(e) => Err(e),
            Ok(false) => Err(DBError::NotFoundError(errors::ItemType::Group, group_id)),
            Ok(true) => {
                let group_id = ObjectId::parse_str(&group_id).unwrap();
                let res = self
                    .group_collection
                    .update_one(
                        doc! {"_id": group_id},
                        doc! {"$set": {"name": &group.name, "groupType": bson::to_bson(&group.group_type)?, "updatedAt": bson::Bson::DateTime(chrono::Utc::now().into())}},
                    )
                    .await?;
                if res.modified_count == 0 {
                    return Err(DBError::NotFoundError(
                        errors::ItemType::Group,
                        group_id.to_hex(),
                    ));
                }
                Ok(group_id.to_hex())
            }
        }
    }

    pub async fn disband_group(&self, group_id: String) -> Result<()> {
        match self.is_group_active(&group_id).await {
            Ok(false) => Err(DBError::NotFoundError(errors::ItemType::Group, group_id)),
            Ok(true) => {
                let group_id = ObjectId::parse_str(&group_id).unwrap();
                let res = self
                    .group_collection
                    .update_one(
                        doc! {"_id": group_id},
                        doc! {"$set": {"status": "Disbanded", "disbandedAt": bson::Bson::DateTime(chrono::Utc::now().into())}},
                    )
                    .await?;
                if res.modified_count == 0 {
                    return Err(DBError::NotFoundError(
                        errors::ItemType::Group,
                        group_id.to_hex(),
                    ));
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn assign_members(
        &self,
        group_id: String,
        user_id: &String,
        members: Vec<String>,
    ) -> Result<()> {
        match self.is_user_member(&group_id, user_id).await {
            Ok(true) => {
                return Err(DBError::MongoDuplicateError(
                    "User is already a member".into(),
                ))
            }
            Ok(false) => {
                let group_id = ObjectId::parse_str(&group_id).unwrap();
                let res = self
                    .group_collection
                    .update_one(doc! {"_id": group_id}, doc! {"$push": {"members": {"$each": members.iter().filter_map(|id| ObjectId::parse_str(id).ok()).collect::<Vec<ObjectId>>()}}})
                    .await?;
                if res.modified_count == 0 {
                    return Err(DBError::NotFoundError(
                        errors::ItemType::Group,
                        group_id.to_hex(),
                    ));
                }
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    pub async fn is_group_active(&self, group_id: &String) -> Result<bool> {
        let group_id = ObjectId::parse_str(group_id).unwrap();
        let group = self
            .group_collection
            .find_one(doc! {"_id": group_id, "status": "Active"})
            .await?;
        Ok(group.is_some())
    }

    pub async fn is_user_member(&self, group_id: &String, user_id: &String) -> Result<bool> {
        if let Ok(is_admin) = self.is_user_admin(&group_id, &user_id).await {
            if is_admin {
                return Ok(true); // Admins are also members
            }
        }
        let group_id = ObjectId::parse_str(group_id).unwrap();
        let user_id = ObjectId::parse_str(user_id).unwrap();
        let group = self
            .group_collection
            .find_one(doc! {"_id": group_id, "members": user_id})
            .await?;
        Ok(group.is_some())
    }

    pub async fn is_user_admin(&self, group_id: &String, user_id: &String) -> Result<bool> {
        let group_id = ObjectId::parse_str(group_id).unwrap();
        let user_id = ObjectId::parse_str(user_id).unwrap();
        let group = self
            .group_collection
            .find_one(doc! {"_id": group_id, "admins": user_id})
            .await?;
        Ok(group.is_some())
    }

    pub async fn list_group_memerships_for_user(
        &self,
        user_id: String,
    ) -> Result<Vec<m_group::GroupMembership>> {
        let mut memberships: Vec<m_group::GroupMembership> = vec![];
        let user_id = ObjectId::parse_str(&user_id).unwrap();
        let mut cursor = self
            .group_collection
            .find(doc! {"$or": [{"members": user_id}, {"admins": user_id}]})
            .await?;

        while let Some(group) = cursor.try_next().await? {
            let role = if group
                .get_array("admins")?
                .contains(&bson::Bson::ObjectId(user_id.clone()))
            {
                m_group::GroupRole::Admin
            } else {
                m_group::GroupRole::Member
            };
            memberships.push(m_group::GroupMembership {
                group_id: group.get_object_id("_id")?,
                role,
            });
        }
        Ok(vec![])
    }
}
