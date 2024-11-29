//! The [Ana.so](https://ana.so) Algorithm

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]

/// Data used for scoring. Pass into [`score_post`].
pub struct ScoringData {
    /// Unix timestamp in seconds that the post was submitted.
    pub time_posted: i64,
    /// Number of likes the post has received in total.
    pub likes: i64,
}

/// The static score value to be associated with a post.
///
/// This is made to be run on a post's data whenever
/// a like is added or removed, and put in a score column
/// on the post table.
///
/// As an example (though not the real code):
///
/// ```
/// # use anaso_algorithm::*;
/// # struct PostData {time_posted:i64,likes:i64}
/// # fn query(s:&str,x:i64)->PostData{PostData{time_posted:0,likes:0}}
/// # let id=0;
/// // Get the relevant data for calculating the score
/// let post: PostData = query("SELECT * FROM post WHERE id=?", id);
///
/// // Calculate the new score
/// let score = score_post(ScoringData {
///     time_posted: post.time_posted,
///     likes: post.likes
///     // ...
/// });
///
/// // Update with score
/// query("UPDATE post SET score=? WHERE id=?", id);
/// ```
///
/// When getting posts for a particular page, they will be
/// ordered by this value.
///
/// As an example (though not the real query):
///
/// ```sql
/// SELECT * FROM post ORDER BY score LIMIT=20;
/// ```
pub fn score_post(data: ScoringData) -> i64 {
    const TWELVE_HOURS: i64 = 43200;

    // Number we pass into log must be at least one.
    let likes_normalized = (data.likes + 1).max(1);

    // Likes get a diminishing return
    let likes_logged = (likes_normalized as f64).log10() as i64;

    data.time_posted / TWELVE_HOURS + likes_logged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
