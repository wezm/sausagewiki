use std::fmt::Debug;

use diff;
use diff::Result::*;

use super::chunk::Chunk;

#[derive(Debug, PartialEq)]
pub enum Output<Item: Debug + PartialEq + Copy> {
    Resolved(Vec<Item>),
    Conflict(Vec<Item>, Vec<Item>, Vec<Item>),
}

fn choose_left<Item: Copy>(operations: &[diff::Result<Item>]) -> Vec<Item> {
    operations
        .iter()
        .filter_map(|x| match x {
            &Both(y, _) => Some(y),
            &Left(y) => Some(y),
            &Right(_) => None,
        })
        .collect()
}

fn choose_right<Item: Copy>(operations: &[diff::Result<Item>]) -> Vec<Item> {
    operations
        .iter()
        .filter_map(|x| match x {
            &Both(_, y) => Some(y),
            &Left(_) => None,
            &Right(y) => Some(y),
        })
        .collect()
}

fn no_change<Item>(operations: &[diff::Result<Item>]) -> bool {
    operations
        .iter()
        .all(|x| match x {
            &Both(..) => true,
            _ => false,
        })
}

pub fn resolve<'a, Item: 'a + Debug + PartialEq + Copy>(chunk: Chunk<'a, Item>) -> Output<Item> {
    if chunk.0 == chunk.1 {
        // Either nothing changed or both sides made the same change
        return Output::Resolved(choose_right(chunk.0));
    }

    if no_change(chunk.0) {
        return Output::Resolved(choose_right(chunk.1));
    }

    if no_change(chunk.1) {
        return Output::Resolved(choose_right(chunk.0));
    }

    return Output::Conflict(
        choose_right(chunk.0),
        choose_left(chunk.0),
        choose_right(chunk.1),
    );
}

#[cfg(test)]
mod test {
    use diff::Result::*;
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            Output::Resolved(vec![]),
            resolve::<i32>(Chunk(&[], &[]))
        );
    }

    #[test]
    fn same() {
        assert_eq!(
            Output::Resolved(vec![
                1
            ]),
            resolve::<i32>(Chunk(
                &[Both(1, 1)],
                &[Both(1, 1)]
            ))
        );
    }

    #[test]
    fn only_left() {
        assert_eq!(
            Output::Resolved(vec![
                2
            ]),
            resolve::<i32>(Chunk(
                &[
                    Left(1),
                    Right(2)
                ],
                &[]
            ))
        );
    }

    #[test]
    fn false_conflict() {
        assert_eq!(
            Output::Resolved(vec![
                2
            ]),
            resolve::<i32>(Chunk(
                &[
                    Left(1),
                    Right(2)
                ],
                &[
                    Left(1),
                    Right(2)
                ],
            ))
        );
    }

    #[test]
    fn real_conflict() {
        assert_eq!(
            Output::Conflict(
                vec![2],
                vec![1],
                vec![3],
            ),
            resolve::<i32>(Chunk(
                &[
                    Left(1),
                    Right(2)
                ],
                &[
                    Left(1),
                    Right(3)
                ],
            ))
        );
    }
}
