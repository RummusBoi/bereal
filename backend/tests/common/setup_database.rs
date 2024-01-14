use backend::database::{
    sql_helpers::get_pool,
    types::{comment::Comment, image::Image, post::Post, user::User},
};
use my_sqlx_crud::traits::Crud;

pub struct DbState {
    pub user: User,
    pub friends: Vec<User>,
    pub posts: Vec<Post>,
    pub images: Vec<Image>,
    pub comments: Vec<Comment>,
}

pub fn create_simple_friendgroup() -> DbState {
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

    let rt = tokio::runtime::Runtime::new().unwrap();

    let pool = rt.block_on(get_pool());

    // ---
    // Create friends
    // ---

    let mut friends: Vec<User> = (0..friend_count)
        .map(|_| rt.block_on(User::new(vec![]).create(&pool)).unwrap())
        .collect();

    // ---
    // Create user
    // ---

    let user = rt
        .block_on(User::new(friends.iter().map(|f| f.id).collect()).create(&pool))
        .unwrap();

    // ---
    // Add user to friendlists of all the other users
    // ---

    friends = friends
        .iter()
        .map(|f| {
            let friend_mutated = User {
                id: f.id,
                friends: vec![user.id],
                timestamp: f.timestamp,
            };
            return rt.block_on(friend_mutated.update(&pool)).unwrap();
        })
        .collect();

    // ---
    // For each friend create their image, comments, and post.
    // Aggregate all posts in the mutable all_posts Vec.
    // ---

    let mut friend_posts: Vec<Post> = Vec::new();
    let mut friend_images: Vec<Image> = Vec::new();
    let mut friend_comments: Vec<Comment> = Vec::new();

    for friend in friends.iter() {
        let image = rt.block_on(Image::new().create(&pool)).unwrap();
        let comments: Vec<Comment> = (0..comments_per_post)
            .map(|_| rt.block_on(Comment::new(friend.id).create(&pool)).unwrap())
            .collect();
        let post = rt
            .block_on(
                Post::new(friend.id, image.id, comments.iter().map(|c| c.id).collect())
                    .create(&pool),
            )
            .unwrap();

        friend_posts.push(post);
        friend_images.push(image);
        friend_comments.extend(comments);
    }

    // ---
    // Create the user image, comments and post.
    // Add the post to the mutable all_posts Vec.
    // ---

    let user_image = rt.block_on(Image::new().create(&pool)).unwrap();
    let user_comments: Vec<Comment> = friends
        .iter()
        .chain(vec![&user])
        .map(|f| rt.block_on(Comment::new(f.id).create(&pool)).unwrap())
        .collect();
    let user_post = rt
        .block_on(
            Post::new(
                user.id,
                user_image.id,
                user_comments.iter().map(|c| c.id).collect(),
            )
            .create(&pool),
        )
        .unwrap();

    DbState {
        user: user,
        friends: friends,
        posts: [vec![user_post], friend_posts].concat(),
        images: [vec![user_image], friend_images].concat(),
        comments: friend_comments,
    }
}
