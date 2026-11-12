//! The Kobana art in the terminal: the logo, the stork and Kadu, behind
//! `kobana art`, `kobana oi` and `kobana kadu <TEXTO>`.
//!
//! These commands are the one exception to two rules of [`crate::ui`]: they
//! paint in 24-bit color and `anim` redraws the screen. Both happen only
//! because the person asked for a picture, the picture carries no information
//! and it steps down on its own:
//!
//! | Where it runs | What it gets |
//! |---|---|
//! | A terminal that says it takes real color (`COLORTERM`) | the picture |
//! | Any other terminal with color | the picture, in 256 colors |
//! | No color (`--no-color`, `NO_COLOR`, a pipe, `KOBANA_ACCESSIBLE_COLORS`) | the picture's silhouette |
//! | `KOBANA_SCREEN_READER` | a sentence saying what it is; nothing moves |
//! | A window too small | cropped, with a warning; the animation stands still |

use std::borrow::Cow;
use std::ffi::OsString;
use std::io::{IsTerminal, Write};
use std::time::Duration;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use kobana::error::KobanaError;
use kobana::{tr, trf};
use unicode_width::UnicodeWidthStr;

use crate::ui::{self, Stream};

const KADU: &str = include_str!("../assets/art/kadu.ans");
const FRAMES: &str = include_str!("../assets/art/kadu-anim.ans");
const LOGO: &str = include_str!("../assets/art/kobana-logo.txt");
const STORK: &str = include_str!("../assets/art/cegonha.ans");

/// The product whose name also makes Kadu speak: `kobana kadu "Boleto pago!"`
pub const SPEAKER: &str = "kadu";
/// The row of [`KADU`] with his mouth, where a speech bubble's tail points
const MOUTH_ROW: usize = 14;
/// The widest a speech bubble's text gets
const BUBBLE_COLUMNS: usize = 60;
/// Any narrower beside Kadu and the text reads badly, so the bubble goes alone
const MIN_BUBBLE_COLUMNS: usize = 12;
/// The least room between Kadu and his bubble, which the tail crosses
const TAIL_COLUMNS: usize = 3;
/// What a bubble takes besides its text: the borders and their padding
const BUBBLE_FRAME_COLUMNS: usize = 4;

/// What a picture can count on where it is going
struct Screen {
    /// Colors may be written, and the palette may have hues
    color: bool,
    /// The terminal says it takes 24-bit color
    true_color: bool,
    /// stdout is a terminal of this size (columns, rows)
    size: Option<(usize, usize)>,
}

impl Screen {
    fn detect() -> Self {
        let size = if std::io::stdout().is_terminal() { crossterm::terminal::size().ok() } else { None };
        Self {
            color: ui::color(Stream::Stdout) && !ui::accessible_colors(),
            true_color: cfg!(windows)
                || std::env::var("COLORTERM").is_ok_and(|v| matches!(v.to_ascii_lowercase().as_str(), "truecolor" | "24bit")),
            // Some terminals answer 0×0 when they do not know
            size: size.filter(|(columns, rows)| *columns > 0 && *rows > 0).map(|(columns, rows)| (columns as usize, rows as usize)),
        }
    }

    /// 24-bit art in the colors this terminal takes
    fn colors<'a>(&self, art: &'a str) -> Cow<'a, str> {
        if self.true_color {
            art.into()
        } else {
            to_256(art).into()
        }
    }
}

#[derive(Clone, Copy)]
enum Piece {
    Kadu,
    Logo,
    Stork,
}

impl Piece {
    /// What a screen reader gets instead of the picture
    fn description(self) -> &'static str {
        match self {
            Self::Kadu => tr!(pt: "Kadu, o agente da Kobana.", en: "Kadu, Kobana's agent.", es: "Kadu, el agente de Kobana."),
            Self::Logo => tr!(pt: "O logo da Kobana.", en: "The Kobana logo.", es: "El logo de Kobana."),
            Self::Stork => tr!(pt: "A cegonha da Kobana.", en: "The Kobana stork.", es: "La cigüeña de Kobana."),
        }
    }

    fn render(self, screen: &Screen) -> Cow<'static, str> {
        let art = match self {
            Self::Kadu => KADU,
            Self::Logo => return LOGO.into(),
            Self::Stork => STORK,
        };
        if screen.color {
            screen.colors(art)
        } else {
            strip_colors(art).into()
        }
    }
}

/// `kobana art [kadu|anim|logo|cegonha]`
pub async fn handle_art(matches: &clap::ArgMatches) -> Result<(), KobanaError> {
    let screen = Screen::detect();
    match matches.subcommand_name() {
        Some("kadu") => draw(Piece::Kadu, &screen),
        Some("anim") => animate(&screen).await,
        Some("logo") => draw(Piece::Logo, &screen),
        _ => draw(Piece::Stork, &screen),
    }
}

/// `kobana oi`: Kadu sways, smiles and blinks if the window has room for it,
/// and otherwise says hi from a speech bubble
pub async fn handle_oi() -> Result<(), KobanaError> {
    let hello = tr!(
        pt: "Oi! Eu sou o Kadu, o agente da Kobana. Veja o que dá para fazer com kobana --help.",
        en: "Hi! I'm Kadu, Kobana's agent. See what you can do with kobana --help.",
        es: "¡Hola! Soy Kadu, el agente de Kobana. Mira lo que puedes hacer con kobana --help.",
    );
    if ui::screen_reader() {
        return emit(&format!("{hello}\n"));
    }
    let screen = Screen::detect();
    if screen.color && Frames::parse(FRAMES).fit(&screen) {
        play(&screen).await;
        return emit(&format!("{hello}\n"));
    }
    emit(&speech(hello, &screen))
}

/// The text of `kobana kadu "Boleto pago!"`, when that is what a command line
/// clap refused as an unknown command really is.
///
/// `kobana kadu` is a product, and its command tree stays exactly what the
/// spec makes it: every line it knows runs as ever, and only a refused one
/// gets here. Even then the person's typo must stay an error, so this is
/// speech only for a person at a terminal (a script or an agent gets the error
/// it always got), only on the first word after `kadu` (`kadu tasks lista` is
/// a typo), only with the options that apply, and not when that word is nearly
/// a command (`kadu taks list`), unless quotes made it a sentence.
pub fn speech_in(err: &clap::Error, args: impl IntoIterator<Item = OsString>, terminal: bool) -> Option<String> {
    if !terminal || err.kind() != ErrorKind::InvalidSubcommand {
        return None;
    }
    let global = |arg: clap::Arg| arg.global(true);
    let matches = clap::Command::new("kobana")
        .arg(global(clap::Arg::new("lang").long("lang")))
        .arg(global(clap::Arg::new("no-color").long("no-color").action(clap::ArgAction::SetTrue)))
        .subcommand(clap::Command::new(SPEAKER).arg(clap::Arg::new("text").required(true).num_args(1..)))
        .try_get_matches_from(args)
        .ok()?;
    let (SPEAKER, speaker) = matches.subcommand()? else {
        return None;
    };
    let words: Vec<&str> = speaker.get_many::<String>("text")?.map(String::as_str).collect();
    let refused = match err.get(ContextKind::InvalidSubcommand)? {
        ContextValue::String(refused) => refused.as_str(),
        _ => return None,
    };
    if refused != words[0] || nearly_a_command(err, words[0]) {
        return None;
    }
    let text = words.join(" ");
    (!text.trim().is_empty()).then_some(text)
}

/// Whether `word` is a slip on the command clap suggests. clap's own test is
/// too generous here, where a false alarm refuses a sentence: to it, `Boleto`
/// looks like `roles`. A person sees a bubble with their typo at once, so this
/// only needs to catch the plain ones, within a letter or two.
fn nearly_a_command(err: &clap::Error, word: &str) -> bool {
    let Some(ContextValue::Strings(suggested)) = err.get(ContextKind::SuggestedSubcommand) else {
        return false;
    };
    let word = word.to_lowercase();
    let slips = if word.chars().count() < 6 { 1 } else { 2 };
    !word.contains(char::is_whitespace) && suggested.iter().any(|command| strsim::levenshtein(&word, command) <= slips)
}

/// `kobana kadu <TEXTO>`, with the text [`speech_in`] found
pub fn handle_speech(text: &str) -> Result<(), KobanaError> {
    let text = ui::terminal_safe(text.trim());
    if ui::screen_reader() {
        return emit(&trf!(pt: "Kadu diz: {text}\n", en: "Kadu says: {text}\n", es: "Kadu dice: {text}\n"));
    }
    emit(&speech(&text, &Screen::detect()))
}

fn draw(piece: Piece, screen: &Screen) -> Result<(), KobanaError> {
    if ui::screen_reader() {
        return emit(&format!("{}\n", piece.description()));
    }
    let art = piece.render(screen);
    let needed = width(&art);
    match screen.size.map(|(columns, _)| columns).filter(|columns| *columns < needed) {
        Some(columns) => {
            ui::warn(&trf!(
                pt: "esta arte pede {needed} colunas e a janela tem {columns}: ela saiu cortada",
                en: "this art takes {needed} columns and the window has {columns}: it came out cropped",
                es: "esta imagen pide {needed} columnas y la ventana tiene {columns}: salió cortada",
            ));
            emit(&crop(&art, columns))
        }
        None => emit(&art),
    }
}

/// Writes to stdout; a reader that left (`kobana art | head`) is no failure
fn emit(text: &str) -> Result<(), KobanaError> {
    let mut out = std::io::stdout();
    match out.write_all(text.as_bytes()).and_then(|()| out.flush()) {
        Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => Err(e.into()),
        _ => Ok(()),
    }
}

// ---------------------------------------------------------------- pictures

/// Kadu with a speech bubble beside his mouth. A low window shows him from the
/// top down, as far as it fits, so the head and the bubble stay on screen; a
/// window too narrow for both gets the bubble alone, so no text is lost.
fn speech(text: &str, screen: &Screen) -> String {
    let art = Piece::Kadu.render(screen);
    let picture: Vec<&str> = art.lines().collect();
    let shown = screen.size.map_or(picture.len(), |(_, rows)| picture.len().min(rows.saturating_sub(2)));
    let picture = &picture[..shown];
    let room = |taken: usize| {
        let columns = screen.size.map_or(usize::MAX, |(columns, _)| columns);
        columns.saturating_sub(taken + BUBBLE_FRAME_COLUMNS).min(BUBBLE_COLUMNS)
    };

    // As near the mouth as the rows beside the bubble allow. A tall bubble
    // reaches the wider shoulders and moves away, which makes it narrower and
    // taller; the column only grows and has a limit, so this ends.
    let mut column = 0;
    let (lines, top) = loop {
        let lines = wrap(text, room(column + TAIL_COLUMNS).max(1));
        let height = lines.len() + 2;
        let top = (MOUTH_ROW - 1).min(shown.saturating_sub(height));
        let needed = picture.iter().skip(top).take(height).map(|row| filled(row)).max().unwrap_or(0);
        if needed <= column {
            break (lines, top);
        }
        column = needed;
    };
    if room(column + TAIL_COLUMNS) < MIN_BUBBLE_COLUMNS {
        let alone = bubble(&wrap(text, room(0).max(1)), false);
        return alone.iter().map(|line| format!("{line}\n")).collect();
    }

    let bubble = bubble(&lines, true);
    let mut out = String::new();
    for i in 0..shown.max(top + bubble.len()) {
        let row = picture.get(i).copied().unwrap_or("");
        match i.checked_sub(top).and_then(|at| bubble.get(at)) {
            Some(piece) => {
                let used = filled(row);
                let gap = column + TAIL_COLUMNS - used;
                // The row of the bubble's first line: the tail, from his face to it
                let gap = if i == top + 1 { format!(" {}", "─".repeat(gap - 1)) } else { " ".repeat(gap) };
                out.push_str(&format!("{}{gap}{piece}\n", crop_line(row, used)));
            }
            None => out.push_str(&format!("{row}\n")),
        }
    }
    out
}

/// The lines of a speech bubble; `tail` opens its left side to take the tail
fn bubble(lines: &[String], tail: bool) -> Vec<String> {
    let inner = lines.iter().map(|line| line.width()).max().unwrap_or(0);
    let rule = "─".repeat(inner + 2);
    let mut out = vec![format!("╭{rule}╮")];
    for (i, line) in lines.iter().enumerate() {
        let side = if tail && i == 0 { '┤' } else { '│' };
        out.push(format!("{side} {line}{} │", " ".repeat(inner - line.width())));
    }
    out.push(format!("╰{rule}╯"));
    out
}

/// `text` in lines of at most `limit` columns: its own line breaks stay, a
/// word goes to the next line whole, and only a word wider than `limit` breaks
fn wrap(text: &str, limit: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if !line.is_empty() && line.width() + 1 + word.width() > limit {
                lines.push(std::mem::take(&mut line));
            }
            if !line.is_empty() {
                line.push(' ');
            }
            for c in word.chars() {
                let mut buffer = [0; 4];
                if !line.is_empty() && line.width() + c.encode_utf8(&mut buffer).width() > limit {
                    lines.push(std::mem::take(&mut line));
                }
                line.push(c);
            }
        }
        lines.push(line);
    }
    lines
}

/// `art` without its escape sequences
fn strip_colors(art: &str) -> String {
    let mut out = String::with_capacity(art.len());
    let mut chars = art.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            // Up to the letter that ends the sequence
            chars.by_ref().find(char::is_ascii_alphabetic);
        } else {
            out.push(c);
        }
    }
    out
}

/// The columns `art` takes: every glyph in it is one column wide
fn width(art: &str) -> usize {
    strip_colors(art).lines().map(|line| line.chars().count()).max().unwrap_or(0)
}

/// The columns of a row up to its last glyph; past it the row is blank
fn filled(row: &str) -> usize {
    strip_colors(row).trim_end().chars().count()
}

/// `art` cut at `limit` columns
fn crop(art: &str, limit: usize) -> String {
    art.lines().map(|line| crop_line(line, limit) + "\n").collect()
}

/// `line` cut at `limit` columns. Escape sequences past the cut stay, so it
/// still ends with its colors reset.
fn crop_line(line: &str, limit: usize) -> String {
    let mut out = String::with_capacity(line.len());
    let mut shown = 0;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            out.push(c);
            for code in chars.by_ref() {
                out.push(code);
                if code.is_ascii_alphabetic() {
                    break;
                }
            }
        } else if shown < limit {
            out.push(c);
            shown += 1;
        }
    }
    out
}

/// The art's 24-bit colors (`38;2;r;g;b`, one per sequence) as the nearest of
/// the 256, which a terminal that lacks real color shows right instead of
/// misreading the numbers as other styles
fn to_256(art: &str) -> String {
    let mut out = String::with_capacity(art.len());
    let mut rest = art;
    while let Some((before, after)) = rest.split_once("\u{1b}[") {
        let end = after.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(after.len());
        let (params, tail) = after.split_at(end);
        out.push_str(before);
        out.push_str("\u{1b}[");
        let parts: Vec<&str> = params.split(';').collect();
        match parts[..] {
            [ground @ ("38" | "48"), "2", r, g, b] => {
                let channel = |value: &str| value.parse().unwrap_or(0);
                out.push_str(&format!("{ground};5;{}", nearest_256(channel(r), channel(g), channel(b))));
            }
            _ => out.push_str(params),
        }
        rest = tail;
    }
    out.push_str(rest);
    out
}

/// The nearest of the xterm palette: its 6×6×6 cube or its 24 greys
fn nearest_256(r: u8, g: u8, b: u8) -> u8 {
    const LEVELS: [i32; 6] = [0, 95, 135, 175, 215, 255];
    let distance = |x: i32, y: i32, z: i32| (x - r as i32).pow(2) + (y - g as i32).pow(2) + (z - b as i32).pow(2);
    let step = |value: u8| (0..6).min_by_key(|&i| (LEVELS[i] - value as i32).abs()).unwrap_or(0);
    let (x, y, z) = (step(r), step(g), step(b));
    let cube = distance(LEVELS[x], LEVELS[y], LEVELS[z]);

    let average = (r as i32 + g as i32 + b as i32) / 3;
    let grey_step = ((average - 3).max(0) / 10).min(23);
    let grey = 8 + 10 * grey_step;

    if distance(grey, grey, grey) < cube {
        232 + grey_step as u8
    } else {
        (16 + 36 * x + 6 * y + z) as u8
    }
}

// ---------------------------------------------------------------- animation

/// `kadu-anim.ans`: a header `KADU <rows> <frames> <fps>`, the order the
/// frames play in, and then every line of every frame
struct Frames<'a> {
    rows: usize,
    delay: Duration,
    sequence: Vec<usize>,
    lines: Vec<&'a str>,
}

impl<'a> Frames<'a> {
    /// The text is embedded, and `the_frames_are_complete` checks its shape
    fn parse(text: &'a str) -> Self {
        let number = |value: &str| value.parse::<usize>().expect("embedded frames");
        let mut lines = text.lines();
        let header: Vec<usize> = lines.next().expect("embedded frames").split_whitespace().skip(1).map(number).collect();
        let sequence = lines.next().expect("embedded frames").split_whitespace().map(number).collect();
        Self {
            rows: header[0],
            delay: Duration::from_secs(1) / header[2] as u32,
            sequence,
            lines: lines.collect(),
        }
    }

    fn columns(&self) -> usize {
        self.lines[..self.rows].iter().map(|line| width(line)).max().unwrap_or(0)
    }

    /// One row more than the picture: the line the cursor ends on. A window
    /// any lower scrolls, and the frames would be drawn over each other.
    fn rows_needed(&self) -> usize {
        self.rows + 1
    }

    fn fit(&self, screen: &Screen) -> bool {
        screen.size.is_some_and(|(columns, rows)| columns >= self.columns() && rows >= self.rows_needed())
    }
}

async fn animate(screen: &Screen) -> Result<(), KobanaError> {
    if ui::screen_reader() {
        return draw(Piece::Kadu, screen);
    }
    let frames = Frames::parse(FRAMES);
    if screen.color && frames.fit(screen) {
        play(screen).await;
        return Ok(());
    }
    // Without color or a terminal, a still Kadu is simply what was asked for;
    // the window is worth a word, since the person can do something about it
    if let (true, Some((columns, rows))) = (screen.color, screen.size) {
        let (needed_columns, needed_rows) = (frames.columns(), frames.rows_needed());
        ui::warn(&trf!(
            pt: "a animação pede uma janela de {needed_columns}×{needed_rows} e esta tem {columns}×{rows}: o Kadu ficou parado",
            en: "the animation takes a {needed_columns}×{needed_rows} window and this one is {columns}×{rows}: Kadu stood still",
            es: "la animación pide una ventana de {needed_columns}×{needed_rows} y esta tiene {columns}×{rows}: Kadu se quedó quieto",
        ));
    }
    draw(Piece::Kadu, screen)
}

/// Plays the frames once, writing only the lines that changed, or until an
/// interrupt. Always shows the cursor again. A write that fails means the
/// terminal left, and so there is nothing to report to.
async fn play(screen: &Screen) {
    let text = screen.colors(FRAMES);
    let frames = Frames::parse(&text);
    let rows = frames.rows;
    let mut out = std::io::stdout();

    // Room for the picture; back to its top, cursor hidden and position saved
    let setup = format!("{}\u{1b}[{rows}A\u{1b}[?25l\u{1b}7", "\n".repeat(rows));
    let _ = out.write_all(setup.as_bytes());

    let stop = interrupted();
    tokio::pin!(stop);
    let mut shown = vec![""; rows];
    for &frame in &frames.sequence {
        let mut chunk = String::new();
        for (i, line) in frames.lines[frame * rows..(frame + 1) * rows].iter().enumerate() {
            if *line != shown[i] {
                chunk.push_str("\u{1b}8");
                if i > 0 {
                    chunk.push_str(&format!("\u{1b}[{i}B"));
                }
                chunk.push_str(line);
                shown[i] = line;
            }
        }
        if out.write_all(chunk.as_bytes()).and_then(|()| out.flush()).is_err() {
            break;
        }
        tokio::select! {
            () = &mut stop => break,
            () = tokio::time::sleep(frames.delay) => {}
        }
    }

    let _ = writeln!(out, "\u{1b}8\u{1b}[{rows}B\u{1b}[0m\u{1b}[?25h");
    let _ = out.flush();
}

/// Ctrl+C, or a polite `kill`
async fn interrupted() {
    #[cfg(unix)]
    if let Ok(mut terminate) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = terminate.recv() => {}
        }
        return;
    }
    let _ = tokio::signal::ctrl_c().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen(color: bool, size: Option<(usize, usize)>) -> Screen {
        Screen { color, true_color: true, size }
    }

    /// Every escape sequence in `art`, without the `ESC`
    fn sequences(art: &str) -> Vec<String> {
        art.split('\u{1b}')
            .skip(1)
            .map(|piece| {
                let end = piece.find(|c: char| c.is_ascii_alphabetic()).map_or(piece.len(), |i| i + 1);
                piece[..end].to_string()
            })
            .collect()
    }

    #[test]
    fn the_art_can_only_set_colors() {
        for art in [KADU, LOGO, STORK] {
            assert!(sequences(art).iter().all(|s| s.starts_with('[') && s.ends_with('m')));
        }
        // The frames also erase to the end of the line, so a shorter one leaves nothing behind
        assert!(sequences(FRAMES).iter().all(|s| s.starts_with('[') && (s.ends_with('m') || s == "[K")));
    }

    #[test]
    fn the_art_has_the_size_the_help_states() {
        let size = |art: &str| (width(art), art.lines().count());
        assert_eq!(size(KADU), (55, 44));
        assert_eq!(size(LOGO), (64, 6));
        assert_eq!(size(STORK), (96, 32));
    }

    #[test]
    fn the_frames_are_complete() {
        let frames = Frames::parse(FRAMES);
        let count: usize = FRAMES.split_whitespace().nth(2).unwrap().parse().unwrap();
        assert_eq!(frames.lines.len(), frames.rows * count);
        assert!(frames.sequence.iter().all(|frame| *frame < count));
        // A cycle is what the help says, about 6 s
        assert_eq!((frames.delay * frames.sequence.len() as u32).as_secs_f32().round(), 6.0);
        assert_eq!((frames.columns(), frames.rows_needed()), (48, 37));
    }

    #[test]
    fn the_animation_needs_a_window_it_fits() {
        let frames = Frames::parse(FRAMES);
        assert!(frames.fit(&screen(true, Some((48, 37)))));
        assert!(!frames.fit(&screen(true, Some((48, 36)))), "the last line would scroll");
        assert!(!frames.fit(&screen(true, Some((47, 50)))));
        assert!(!frames.fit(&screen(true, None)), "not a terminal");
    }

    #[test]
    fn without_color_no_piece_has_escapes() {
        for piece in [Piece::Kadu, Piece::Logo, Piece::Stork] {
            let art = piece.render(&screen(false, None));
            assert!(!art.contains('\u{1b}'));
            assert!(!art.trim().is_empty());
        }
    }

    #[test]
    fn without_real_color_the_art_takes_256() {
        assert_eq!(nearest_256(0, 0, 0), 16);
        assert_eq!(nearest_256(255, 255, 255), 231);
        assert_eq!(nearest_256(128, 128, 128), 244);
        assert_eq!(nearest_256(255, 0, 0), 196);

        assert_eq!(to_256("a\u{1b}[38;2;255;0;0mb\u{1b}[48;2;0;0;0mc\u{1b}[0m\u{1b}[K"), "a\u{1b}[38;5;196mb\u{1b}[48;5;16mc\u{1b}[0m\u{1b}[K");
        for art in [KADU, STORK, FRAMES] {
            let converted = to_256(art);
            assert!(!converted.contains("8;2;"));
            assert_eq!(strip_colors(&converted), strip_colors(art), "only the colors change");
        }
    }

    #[test]
    fn a_crop_cuts_the_columns_and_keeps_the_reset() {
        let art = "\u{1b}[31mabcd\u{1b}[0m\nef\n";
        assert_eq!(crop(art, 3), "\u{1b}[31mabc\u{1b}[0m\nef\n");
        assert_eq!(width(&crop(STORK, 80)), 80);
    }

    #[test]
    fn long_text_wraps_at_words_and_only_breaks_what_does_not_fit() {
        assert_eq!(wrap("um dois tres", 7), ["um dois", "tres"]);
        assert_eq!(wrap("abcdefgh ij", 5), ["abcde", "fgh", "ij"]);
        assert_eq!(wrap("a\n\nb", 5), ["a", "", "b"]);
        // Wide glyphs take two columns
        assert_eq!(wrap("日本語", 4), ["日本", "語"]);
    }

    #[test]
    fn the_bubble_sits_beside_the_mouth_with_its_tail() {
        let out = speech("Boleto pago!", &screen(false, Some((100, 50))));
        let rows: Vec<&str> = out.lines().collect();
        assert_eq!(rows.len(), 44, "{out}");
        assert!(rows[MOUTH_ROW - 1].ends_with("╭──────────────╮"), "{out}");
        assert!(rows[MOUTH_ROW].ends_with("─┤ Boleto pago! │"), "{out}");
        assert!(rows[MOUTH_ROW + 1].ends_with("╰──────────────╯"), "{out}");
        // Beside the head, so nearer than the shoulders would put it
        assert_eq!(rows[MOUTH_ROW - 1].find('╭').map(|at| rows[MOUTH_ROW - 1][..at].chars().count()), Some(42), "{out}");
    }

    #[test]
    fn the_bubble_never_covers_kadu_and_keeps_to_the_window() {
        let text = "palavra ".repeat(60);
        let plain = strip_colors(KADU);
        let picture: Vec<&str> = plain.lines().collect();
        for (columns, rows) in [(80, 24), (100, 30), (120, 50), (200, 60)] {
            let out = speech(&text, &screen(false, Some((columns, rows))));
            for (i, row) in out.lines().enumerate() {
                assert!(row.width() <= columns, "{columns}×{rows}: {row}");
                if let Some(original) = picture.get(i) {
                    assert!(row.starts_with(original.trim_end()), "{columns}×{rows} covers row {i}: {row}");
                }
            }
            assert_eq!(out.matches("palavra").count(), 60, "{columns}×{rows}: no word lost");
        }
        // Right beside the head, a short text still fits a window this narrow
        assert!(speech("Boleto pago!", &screen(false, Some((61, 30)))).contains("┤ Boleto pago! │"));
    }

    #[test]
    fn a_low_window_keeps_the_head_and_the_bubble_on_screen() {
        let out = speech("Boleto pago!", &screen(false, Some((80, 24))));
        assert_eq!(out.lines().count(), 22, "{out}");
        assert!(out.lines().next().unwrap().starts_with(strip_colors(KADU).lines().next().unwrap().trim_end()), "{out}");
        assert!(out.contains("Boleto pago!"), "{out}");

        // A long text pushes the bubble up instead of past the window
        let out = speech(&"palavra ".repeat(30), &screen(false, Some((80, 24))));
        assert_eq!(out.lines().count(), 22, "{out}");
    }

    #[test]
    fn a_window_too_narrow_for_both_gets_the_bubble_alone() {
        let out = speech("Boleto pago! Pode mandar o próximo.", &screen(false, Some((40, 24))));
        assert!(!out.contains('▀') && !out.contains('┤'), "{out}");
        assert!(out.contains("Boleto pago!"), "{out}");
        assert!(out.lines().all(|row| row.width() <= 40), "{out}");
    }

    #[test]
    fn the_colored_speech_is_the_same_drawing() {
        for size in [Some((100, 50)), Some((80, 24)), None] {
            let plain = speech("Boleto pago!", &screen(false, size));
            let colored = speech("Boleto pago!", &screen(true, size));
            assert!(colored.contains('\u{1b}'));
            assert_eq!(strip_colors(&colored), plain);
            // The bubble is out of the picture's colors
            let row = colored.lines().find(|row| row.contains("Boleto")).unwrap();
            assert!(row[..row.find('─').unwrap()].ends_with("\u{1b}[0m "), "{row:?}");
        }
    }

    #[test]
    fn only_a_person_gets_speech_and_a_typo_stays_an_error() {
        let products = crate::product::load_all().unwrap();
        let speech_of = |args: &[&str], terminal: bool| {
            let err = crate::commands::build_root_command(&products).try_get_matches_from(args).unwrap_err();
            speech_in(&err, args.iter().map(OsString::from), terminal)
        };
        let said = |text: &str| Some(text.to_string());

        assert_eq!(speech_of(&["kobana", "kadu", "Boleto pago!"], true), said("Boleto pago!"));
        assert_eq!(speech_of(&["kobana", "kadu", "Boleto", "pago"], true), said("Boleto pago"));
        assert_eq!(speech_of(&["kobana", "--lang", "en", "kadu", "Hello", "--no-color"], true), said("Hello"));
        assert_eq!(speech_of(&["kobana", "kadu", "oi"], true), said("oi"));
        assert_eq!(speech_of(&["kobana", "kadu", "taks list"], true), said("taks list"), "quotes make it a sentence");

        assert_eq!(speech_of(&["kobana", "kadu", "Boleto pago!"], false), None, "a script or an agent");
        assert_eq!(speech_of(&["kobana", "kadu", "taks", "list"], true), None, "nearly a command");
        assert_eq!(speech_of(&["kobana", "kadu", "webhoks"], true), None, "nearly a command");
        assert_eq!(speech_of(&["kobana", "kadu", "Task"], true), None, "nearly a command, in any case");
        assert_eq!(speech_of(&["kobana", "kadu", "tasks", "lista"], true), None, "a typo further on");
        assert_eq!(speech_of(&["kobana", "banking", "oi"], true), None, "another product");
        assert_eq!(speech_of(&["kobana", "kadu", "oi", "--dry-run"], true), None, "an option of the API commands");
        assert_eq!(speech_of(&["kobana", "kadu", "  "], true), None, "nothing to say");
        assert_eq!(speech_of(&["kobana", "oops"], true), None, "not under kadu");
    }

    #[test]
    fn the_text_cannot_drive_the_terminal() {
        let safe = ui::terminal_safe("a\u{1b}[2Jb\u{9b}c");
        assert!(!speech(&safe, &screen(false, None)).contains('\u{1b}'));
    }

    #[test]
    fn the_installer_shows_the_same_art() {
        let installer = include_str!("../../../scripts/install.sh");
        assert!(installer.contains(LOGO), "the banner of scripts/install.sh");
        assert!(installer.contains(&format!("-ge {}", width(STORK))), "the window scripts/install.sh checks for");
    }
}
