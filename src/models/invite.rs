use rand::{thread_rng, Rng};
use diesel;
use diesel::ExpressionMethods;

use models::schema::invites;
use models::user::{User, self};
use database;
use models;
use error;

#[derive(Queryable, Identifiable, Debug, Associations)]
#[belongs_to(User)]
pub struct Invite {
    pub id: i64,
    pub user_id: Option<i64>,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    pub invite_key: String,
}

impl Invite {
    pub fn create_from(nup: NewInvite) -> Result<i64, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::invites::dsl::*;
        diesel::insert(&nup).into(invites).returning(id)
            .get_result(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn update(&self, update: &UpdateInvite) -> Result<usize, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::invites::dsl::*;
        diesel::update(invites.filter(id.eq(self.id))).set(update)
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn get_user(&self) -> Result<Option<User>, error::FurryError> {
        match self.user_id {
            Some(id) => user::find(id),
            None => Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name="invites"]
pub struct NewInvite {
    pub invite_key: String,
}

impl NewInvite {
    pub fn new() -> NewInvite {
        let mut rng = thread_rng();
        let key = rng.gen_ascii_chars().take(50).collect::<String>();

        NewInvite {
            invite_key: key,
        }
    }
}

#[derive(Clone, Debug)]
#[derive(AsChangeset)]
#[table_name="invites"]
pub struct UpdateInvite {
    pub id: i64,
    pub user_id: i64,
}

impl UpdateInvite {
    pub fn create_for(inv: &Invite, id: i64) -> UpdateInvite {
        UpdateInvite {
            id: inv.id,
            user_id: id,
        }
    }
}

pub fn find_by_user_id(uid: i64) -> Result<Option<Invite>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::invites::dsl::*;

    invites.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::invite::Invite>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_key(key: &str) -> Result<Option<Invite>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::invites::dsl::*;

    invites.limit(1).filter(invite_key.eq(key))
         .get_result::<models::invite::Invite>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_key_free(key: &str) -> Result<Option<Invite>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::invites::dsl::*;

    let null : Option<i64> = None;
    invites.limit(1).filter(invite_key.eq(key)).filter(user_id.eq(null))
         .get_result::<models::invite::Invite>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn all() -> Result<Vec<Invite>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::invites::dsl::*;

    invites.order(created_at.desc())
         .get_results::<models::invite::Invite>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}

