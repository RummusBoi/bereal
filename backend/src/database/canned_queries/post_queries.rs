use my_sqlx_crud::traits::Schema;

use crate::{database::{types::{post::Post, comment::Comment, image::Image}, sql_helpers::get_pool}, socket_handlers::types::AppError};

pub async fn apply_post_deletion_operations(post: Post) -> Result<String, AppError> {
    let pool = get_pool().await;
    let mut transaction = pool.begin().await?;

    // TODO: replace these loops with something more optimized (query_builder probably)
    // so that we don't make a billion sql queries and instead do all if it in one.
    for comment_id in &post.comments {
        let query_to_remove_comment = sqlx::query(Comment::delete_by_id_sql()).bind(comment_id);
        query_to_remove_comment.execute(&mut *transaction).await?;
    }

    let query_to_remove_image = sqlx::query(Image::delete_by_id_sql()).bind(post.image);
    query_to_remove_image.execute(&mut *transaction).await?;

    let query_to_remove_post = sqlx::query(Post::delete_by_id_sql()).bind(post.id);
    query_to_remove_post.execute(&mut *transaction).await?;

    transaction.commit().await?;
    Ok(format!("Removed post: {:?}, removed comments: {:?}, and removed image: {:?}", post.id, post.comments, post.image))
}
