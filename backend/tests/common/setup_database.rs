use backend::database::{
    sql_helpers::get_pool,
    types::{comment::Comment, image::Image, post::Post, user::User},
    user_controller,
};
use futures::future::join_all;
use my_sqlx_crud::traits::Crud;

pub struct DbState {
    pub user: User,
    pub friends: Vec<User>,
    pub posts: Vec<Post>,
    pub images: Vec<Image>,
    pub comments: Vec<Comment>,
}

pub async fn create_friend_pair() -> (User, User) {
    let pool = get_pool().await;
    let mut user1 = User::new(vec![]).create(&pool).await.unwrap();
    let mut user2 = User::new(vec![]).create(&pool).await.unwrap();

    user1 = User {
        id: user1.id,
        friends: vec![user2.id],
        timestamp: user1.timestamp,
    }
    .update(&pool)
    .await
    .unwrap();

    user2 = User {
        id: user2.id,
        friends: vec![user1.id],
        timestamp: user2.timestamp,
    }
    .update(&pool)
    .await
    .unwrap();

    (user1, user2)
}

pub async fn create_simple_friendgroup() -> DbState {
    /*
       Sets up the database with the following:
       A User with 5 friends.
       5 friends with only the User as their friend.
       1 post per user.
       2 comments per post.
       1 image per post.
    */
    let friend_count = 5;
    let comments_per_post = 3;

    // ---
    // Create friends
    // ---

    let pool = get_pool().await;
    let mut friends: Vec<User> =
        join_all((0..friend_count).map(|_| User::new(vec![]).create(&pool)))
            .await
            .into_iter()
            .map(|f| f.unwrap())
            .collect();

    // ---
    // Create user
    // ---

    let user = User::new(friends.iter().map(|f| f.id).collect())
        .create(&pool)
        .await
        .unwrap();

    // ---
    // Add user to friendlists of all the other users
    // ---

    friends = join_all(friends.iter().map(|f| async {
        let friend_mutated = User {
            id: f.id,
            friends: vec![user.id],
            timestamp: f.timestamp,
        };
        return friend_mutated.update(&pool).await.unwrap();
    }))
    .await;

    // ---
    // For each friend create their image, comments, and post.
    // Aggregate all posts in the mutable all_posts Vec.
    // ---

    let mut friend_posts: Vec<Post> = Vec::new();
    let mut friend_images: Vec<Image> = Vec::new();
    let mut friend_comments: Vec<Comment> = Vec::new();

    for friend in friends.iter() {
        let image = Image::new(vec![1, 2, 3]).create(&pool).await.unwrap();
        let comments: Vec<Comment> = join_all((0..comments_per_post).map(|_| async {
            Comment::new(friend.id, String::new())
                .create(&pool)
                .await
                .unwrap()
        }))
        .await;
        let post = Post::new(friend.id, image.id, comments.iter().map(|c| c.id).collect())
            .create(&pool)
            .await
            .unwrap();

        friend_posts.push(post);
        friend_images.push(image);
        friend_comments.extend(comments);
    }

    // ---
    // Create the user image, comments and post.
    // Add the post to the mutable all_posts Vec.
    // ---

    let user_image = Image::new(vec![1, 2, 3]).create(&pool).await.unwrap();
    let user_comments: Vec<Comment> = join_all(friends.iter().chain(vec![&user]).map(|f| async {
        Comment::new(f.id, String::new())
            .create(&pool)
            .await
            .unwrap()
    }))
    .await;
    let user_post = Post::new(
        user.id,
        user_image.id,
        user_comments.iter().map(|c| c.id).collect(),
    )
    .create(&pool)
    .await
    .unwrap();

    DbState {
        user: user,
        friends: friends,
        posts: [vec![user_post], friend_posts].concat(),
        images: [vec![user_image], friend_images].concat(),
        comments: friend_comments,
    }
}
