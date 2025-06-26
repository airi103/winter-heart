use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::str::FromStr;
use std::{fmt, str};

struct User {
    id: u32,
    inventory: HashMap<Item, u32>,
}

pub enum Item {
    SnowGlobe,
    WoodenWand,
}

/// Retrieves emoji corresponding to the item
impl Item {
    pub fn emoji(&self) -> &'static str {
        match self {
            Item::SnowGlobe => "<:snow_globe:1387751058893307904>",
            Item::WoodenWand => "<:wooden_wand:1387911636249088272>",
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::SnowGlobe => write!(f, "Snow Globe"),
            Item::WoodenWand => write!(f, "Wooden Wand"),
        }
    }
}

impl str::FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Snow Globe" => Ok(Item::SnowGlobe),
            "Wooden Wand" => Ok(Item::WoodenWand),
            _ => Err(()),
        }
    }
}

pub async fn initialize_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let connection_options: SqliteConnectOptions = SqliteConnectOptions::new()
        .filename("winter_heart.sqlite")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connection_options).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn update_inventory(
    user_id: u64,
    item: Item,
    delta: u32,
    pool: &Pool<Sqlite>,
) -> Result<(), sqlx::Error> {
    // insert user id into users if not already present
    sqlx::query(r#"INSERT OR IGNORE INTO users (id) VALUES (?)"#)
        .bind(user_id as i64)
        .execute(pool)
        .await?;

    sqlx::query(r#"INSERT OR IGNORE INTO items (user_id, item, quantity) VALUES (?, ?, ?) ON CONFLICT(user_id, item) DO UPDATE SET quantity = quantity + excluded.quantity"#).bind(user_id as i64).bind(item.to_string()).bind(delta).execute(pool).await?;

    Ok(())
}

pub async fn read_inventory(
    user_id: u64,
    pool: &Pool<Sqlite>,
) -> Result<Vec<(Item, u32)>, sqlx::Error> {
    let rows = sqlx::query(r#"SELECT item, quantity from items WHERE user_id = ?"#)
        .bind(user_id as i64)
        .fetch_all(pool)
        .await?;

    let result: Vec<(Item, u32)> = rows
        .iter()
        .map(|row| {
            let item_str: &str = row.get("item");
            let item: Item = Item::from_str(item_str).expect("Failed to parse item");
            let quantity: u32 = row.get("quantity");
            (item, quantity)
        })
        .collect();

    Ok(result)
}
