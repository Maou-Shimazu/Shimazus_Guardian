use sqlx::{Pool, Sqlite};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Moderation {
    Mute,
    Ban,
    Unmute,
    Unban,
    Kick,
    Warn,
    UpdatedWarn,
    Warnings,
    Lock,
    Unlock,
}

pub async fn new_case(
    pool: Pool<Sqlite>,
    action: Moderation,
    moderator: u64,
    reason: &str,
    userid: u64,
) -> Result<i64, sqlx::Error> {
    let act = match action {
        Moderation::Mute => "Mute",
        Moderation::Ban => "Ban",
        Moderation::Unmute => "Unmute",
        Moderation::Unban => "Unban",
        Moderation::Kick => "Kick",
        Moderation::Warn => "Warn",
        Moderation::UpdatedWarn => "UpdatedWarn",
        Moderation::Warnings => "Warnings",
        Moderation::Lock => "Lock",
        Moderation::Unlock => "Unlock",
    };
    Ok(sqlx::query(&format!(
        r#"
    INSERT INTO cases ( action, moderator_id, reason, userid )
    VALUES ( "{act}", {moderator}, "{reason}", {userid} )
            "#,
    ))
    .execute(&mut pool.acquire().await?)
    .await?
    .last_insert_rowid())
}
