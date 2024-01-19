# Canned queries for the backend

### What is a canned query?
This directory contains canned queries. A canned query is a fixed SQL procedure/query with a few variables.

The queries in this directory are intended for use with our structs representing posts, images, and so on. 

Oversimplified example:

```Rust
let result = post_queries::apply_deletion_operations(post_id, &pool);
```

In this example, the canned query is deleting a post. Upon post deletion, we must check that the post,
image, and comments tied to the post are also deleted. The `apply_deletion_operations(...)` thus makes
method calls to `Comment::delete()`, `Post::delete()`, and `Image::delete()`.

### ACID properties
The ACID properties are properties that make sure the data in a database is valid, i.e. not "fucked". An 
example of fuckedness is if an image exists without a post containing it. Another is if there are comments
on a post, dated before the post's creation date.

These canned queries have the responsibility of ensuring these ACID properties are present. Thus, if you
begin making calls outside of these canned queries, you are no longer guaranteed that the database is valid.

ACID is atomicity, consistency, isolation, and durability. The ACID properties are already enforced
by the PostgreSQL database.

However, if we consider deleting a post and all its comments, images, etc. as a 
transaction, then, without our canned queries, we do not have atomicity yet. This is because we are simply 
calling `delete()` etc. on our tables individually, meaning we might end up deleting the post successfully,
but getting an error when deleting the comments tied to the post. I.e. we have completed part of our transaction,
but not all. This will then fuck up our database.

In the same way, sure, the database without these canned queries is consistent in some way. But we would like
to keep a couple of new invariants, i.e. all comments must have a post, and so on. Thus, in order for the database
to remain consistent w.r.t. these new invariants, we must enforce it in our own way. To see why, consider the 
same example as above. We would not have a consistent database anymore. This is because at this point, our rule 
"all comments must be tied to a post" is now violated. So in order to enforce our new rules, we use these canned queries.
