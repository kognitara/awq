use std::path::Path;

use crossterm::style::Stylize;
use sqlx::{Row, SqliteConnection};

use crate::vcs::{db::AWQ_DB_PATH, ko, locale, tt};

#[async_recursion::async_recursion]
pub async fn print_merkle_tree(
    conn: &mut SqliteConnection,
    tree_hash: &str,
    prefix: &str,
) -> Result<(), anyhow::Error> {
    let rows = sqlx::query(
        r#"
        SELECT name, hash, mode 
        FROM nodes 
        WHERE parent_tree_hash = ? 
        ORDER BY 
            ((mode & 61440) = 16384) DESC, 
            name ASC
        "#,
    )
    .bind(tree_hash)
    .fetch_all(&mut *conn)
    .await?;

    let len = rows.len();
    for (i, row) in rows.into_iter().enumerate() {
        let name: String = row.get(0);
        let hash: String = row.get(1);
        let mode: i64 = row.get(2);

        let is_last = i == len - 1;
        let connector = if is_last { "└──" } else { "├──" };
        let is_dir = (mode & 0o170000) == 0o040000;
        let short_hash = &hash[0..7]; // Affiche les 7 premiers caractères du hash

        if is_dir {
            // Affichage d'un sous-arbre (Dossier)
            println!(
                "[ {} ] {} {} {}",
                short_hash.yellow(),
                prefix,
                connector,
                name.blue().bold()
            );

            let new_prefix = if is_last {
                format!("{}     ", prefix)
            } else {
                format!("{} │   ", prefix)
            };

            print_merkle_tree(conn, &hash, &new_prefix).await?;
        } else {
            // Affichage d'une feuille (Fichier/Blob)
            println!(
                "[ {} ] {} {} {}",
                short_hash.yellow(),
                prefix,
                connector,
                name.white()
            );
        }
    }
    Ok(())
}

pub async fn awq_tree() -> Result<(), anyhow::Error> {
    if Path::new(AWQ_DB_PATH).exists().eq(&false) {
        ko(&tt(&locale(), "not-awq-db-found"));
        return Ok(());
    }
    let pool = crate::vcs::db::conn().await;
    let mut conn = pool.acquire().await?;

    // Récupération de l'état du HEAD
    let head = crate::vcs::commit::get_head_state(&pool).await?;

    match head {
        Some(h) => {
            let root_hash = &h.tree_hash[0..7];

            // Affichage de la racine du dépôt
            println!("[ {} ]  {}", root_hash.red().bold(), ".".white().bold());

            // Lancement de l'exploration de l'arbre
            print_merkle_tree(&mut conn, &h.tree_hash, "").await?;
        }
        None => {
            crate::vcs::ko("Repository is empty. Nothing to tree!");
        }
    }
    Ok(())
}
