use crate::cache::errors::Error;
use crate::cache::TextCache;
use crate::blurses::Blurses;

fn init_cache() -> TextCache {
    let mut text = Vec::new();
    text.push("I have seen the dark universe yawning;".to_string());
    text.push("Where, the black_planets roll.. without aim.".to_string());
    text.push("Where they,,roll in their horror unheeded".to_string());
    text.push("Without knowledge, or lustre, or_name.".to_string());
    text.push("                      - H.P. Lovecraft".to_string());

    let history = Vec::new();

    TextCache::new(text, history)
}

fn init_blurses() -> Blurses {
    Blurses::default()
}

#[test]
fn test_next_nth_occurrence_of_char() {
    let cache = init_cache();
    let blurses = init_blurses();
    let cursor = blurses.get_cursor_position();

    assert_eq!(cursor, (1, 1));

    let mut res = cache
        .next_nth_occurrence_of_char(&'h', 1, cursor)
        .unwrap();

    assert_eq!(res, (3, 1));

    res = cache
        .next_nth_occurrence_of_char(&'W', 1, cursor)
        .unwrap();

    assert_eq!(res, (1, 2));

    res = cache
        .next_nth_occurrence_of_char(&'W', 2, cursor)
        .unwrap();

    assert_eq!(res, (1, 3));

    let err = cache.next_nth_occurrence_of_char(&'I', 2, cursor);
    match err {
        Err(e) => assert_eq!(e, Error::CharNotFound),
        _ => ()
    }
}

#[test]
fn test_is_match() {
    let cache = init_cache();
    let mut ch = '_'.to_string();

    assert_eq!(true, cache.is_match(&ch, r"[a-zA-Z_]{1}"));

    ch = ' '.to_string();

    assert_eq!(true, cache.is_match(&ch, r"[a-zA-Z_ ]{1}"));

    ch = ','.to_string();

    assert_eq!(true, cache.is_match(&ch, r"[^a-zA-Z_ ]{1}"));
}

#[test]
fn test_re_first_match_position() {
    let cache = init_cache();
    let blurses = init_blurses();
    let cursor = blurses.get_cursor_position();

    let mut re = r"\w+;";
    let mut pos = cache.re_first_match_position(re, 0, cursor).unwrap();
    assert_eq!(pos, (31, 1));

    re = r"\w+,";
    pos = cache.re_first_match_position(re, 0, cursor).unwrap();
    assert_eq!(pos, (1, 2));

    re = r"I\s+";
    pos = cache.re_first_match_position(re, 0, cursor).unwrap();
    assert_eq!(pos, (1, 1));

    let res = cache.re_first_match_position(re, 1, cursor);
    match res {
        Err(e) => assert_eq!(e, Error::PatternNotFound),
        _ => (),
    }
}


#[test]
fn test_compute_next_char() {
    let cache = init_cache();
    let mut blurses = init_blurses();
    let mut cursor = blurses.get_cursor_position();

    let mut next_char = cache.compute_next_char(cursor).unwrap();
    assert_eq!(next_char, ' ');

    blurses.cursor_set_position(1, cache.get_line(1).len());
    cursor = blurses.get_cursor_position();
    assert_eq!(cursor, (38, 1));

    next_char = cache.compute_next_char(cursor).unwrap();
    assert_eq!(next_char, 'W');

    let line_count = cache.line_count();

    blurses.cursor_set_position(line_count, cache.get_line(line_count).len());
    cursor = blurses.get_cursor_position();
    assert_eq!(cursor, (38, 5));

    let err = cache.compute_next_char(cursor);
    match err {
        Err(e) => assert_eq!(e, Error::EndOfText),
        _ => ()
    }
}


#[test]
fn test_distance_to_pattern_from_cursor() {
    let cache = init_cache();
    let mut blurses = init_blurses();
    let cursor = blurses.get_cursor_position();
    let dist_a = cache.distance_to_pattern_from_cursor(r"dark", 0, cursor).unwrap();
    let dist_b = cache.distance_to_pattern_from_cursor(r"horror", 0, cursor).unwrap();

    assert_eq!(dist_a < dist_b, true);
    
    let dist_c = cache.distance_to_pattern_from_cursor(r"blessed_dude_bro", 0, cursor);
    match dist_c {
        Err(e) => assert_eq!(e, Error::PatternNotFound),
        _ => ()
    }
}
