macro_rules! shuffle_tests {
    ($module:ident, $rng:ident, $word:ty) => {
        mod $module {
            use super::super::FisherYates;
            use crate::{Shuffle, ShuffleError};
            use moelle_a::$rng;

            struct FixedRng {
                word: $word,
                draws: usize,
            }

            impl moelle::Rng for FixedRng {
                type Word = $word;

                fn next(&mut self) -> $word {
                    self.draws += 1;
                    self.word
                }
            }

            #[test]
            fn empty_slice_returns_error_without_drawing() {
                let mut rng = FisherYates {
                    rng: &mut FixedRng { word: 0, draws: 0 },
                };
                assert!(matches!(
                    rng.shuffle::<i32>(&mut []),
                    Err(ShuffleError::EmptyArray)
                ));
                assert_eq!(rng.rng.draws, 0);
            }

            #[test]
            fn single_item_is_unchanged_without_drawing() {
                let mut rng = FisherYates {
                    rng: &mut FixedRng { word: 0, draws: 0 },
                };
                let mut items = [42];
                rng.shuffle(&mut items).unwrap();
                assert_eq!(items, [42]);
                assert_eq!(rng.rng.draws, 0);
            }

            #[test]
            fn selects_both_ends_of_each_range() {
                for (word, expected) in [(0, [1, 2, 3, 0]), (<$word>::MAX, [0, 1, 2, 3])] {
                    let mut rng = FisherYates {
                        rng: &mut FixedRng { word, draws: 0 },
                    };
                    let mut items = [0, 1, 2, 3];
                    rng.shuffle(&mut items).unwrap();
                    assert_eq!(items, expected);
                    assert_eq!(rng.rng.draws, items.len() - 1);
                }
            }

            #[test]
            fn preserves_all_elements() {
                let mut rng = FisherYates {
                    rng: &mut $rng::from(327984),
                };
                for len in 1..=32 {
                    let mut items: alloc::vec::Vec<_> = (0..len).collect();
                    rng.shuffle(&mut items).unwrap();
                    items.sort_unstable();
                    assert!(items.into_iter().eq(0..len));
                }
            }
        }
    };
}

shuffle_tests!(bits32, Sfc32, u32);
shuffle_tests!(bits64, Sfc64, u64);
