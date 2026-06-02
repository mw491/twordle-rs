use crate::pick_word;
use crate::Route;
use chrono::{DateTime, Utc};
// use gloo_console::log;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

const EMPTY_LETTER: &str = "\u{00A0}"; // character shown to take up the space of a box without a letter
const LETTERS_ROW1: [&str; 10] = ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
const LETTERS_ROW2: [&str; 9] = ["a", "s", "d", "f", "g", "h", "j", "k", "l"];
const LETTERS_ROW3: [&str; 7] = ["z", "x", "c", "v", "b", "n", "m"];

pub struct Twordle {
    title: String,
    wordle: String,
    solved: bool,
    resigned: bool,
    game_started: bool,
    time_started: DateTime<Utc>,
    time_finished: Option<DateTime<Utc>>,
    show_modal: bool,
    typed_word: Vec<String>,
    typed_words: Vec<Vec<String>>,
    typed_words_indexes: Vec<usize>,
    green_letters: Vec<String>,
    yellow_letters: Vec<String>,
    gray_letters: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub game_type: GameType,
}

#[derive(Clone, Eq, PartialEq)]
pub enum GameType {
    Daily,
    Unlimited,
}

pub enum Msg {
    AddLetter(char),
    DeleteLetter,
    SubmitWord,
    Resign,
    PlayAgain,
}

impl Component for Twordle {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut wordle = pick_word::gen();
        let mut title = "Twordle".to_string();
        if ctx.props().game_type == GameType::Unlimited {
            wordle = pick_word::unlimited();
            title = title + " Unlimited";
        }
        Self {
            title,
            wordle,
            solved: false,
            resigned: false,
            game_started: false,
            time_started: Utc::now(),
            time_finished: None,
            show_modal: false,
            typed_word: vec![
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            typed_words: vec![],
            typed_words_indexes: vec![],
            green_letters: vec![],
            yellow_letters: vec![],
            gray_letters: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::PlayAgain = msg {
            if ctx.props().game_type == GameType::Unlimited {
                self.reset_unlimited();
                return true;
            }
            return false;
        }
        if self.solved || self.resigned {
            return false;
        }
        match msg {
            Msg::AddLetter(character) => {
                if !self.game_started {
                    self.time_started = Utc::now();
                    self.game_started = true;
                }
                fn find_empty_string_index(vector: &[String]) -> Option<usize> {
                    vector.iter().position(|s| s.is_empty())
                }
                let i = find_empty_string_index(&self.typed_word);
                match i {
                    Some(i) => {
                        self.typed_word[i] = character.to_lowercase().to_string();
                        true
                    }
                    None => false,
                }
            }
            Msg::DeleteLetter => {
                fn find_non_empty_string_index(vector: &[String]) -> Option<usize> {
                    vector.iter().rposition(|s| !s.is_empty())
                }
                let i = find_non_empty_string_index(&self.typed_word);
                match i {
                    Some(i) => {
                        self.typed_word[i] = String::from("");
                        true
                    }
                    None => false,
                }
            }
            Msg::SubmitWord => {
                if !self.typed_word.iter().all(|letter| !letter.is_empty()) {
                    return false;
                }
                // self.typed_words.append(&mut vec![self.typed_word.clone()]);
                self.typed_words.insert(0, self.typed_word.clone());
                self.typed_words_indexes.insert(0, self.typed_words.len());
                for (i, letter) in self.typed_word.iter().enumerate() {
                    if *letter == self.wordle.chars().nth(i).unwrap().to_string() {
                        self.green_letters.append(&mut vec![letter.to_string()]);
                    } else if self.wordle.contains(letter) {
                        self.yellow_letters.append(&mut vec![letter.to_string()]);
                    } else {
                        self.gray_letters.append(&mut vec![letter.to_string()]);
                    }
                }
                if self.typed_word.join("") == self.wordle {
                    self.solved = true;
                    self.time_finished = Some(Utc::now());
                    self.show_modal = true;
                }
                self.typed_word = vec![
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ];
                true
            }
            Msg::Resign => {
                if self.game_started {
                    self.resigned = true;
                    self.time_finished = Some(Utc::now());
                    self.show_modal = true;
                    true
                } else {
                    false
                }
            }
            Msg::PlayAgain => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="h-[100dvh] max-h-[100dvh] py-3 text-5xl font-mono bg-neutral-900 text-white flex flex-col justify-between items-center touch-none">
                <div class="h-[10dvh] w-full flex flex-col items-center justify-center gap-1 px-5">
                    <div>{&self.title}</div>
                    {
                        if !self.solved && !self.resigned && self.game_started {
                            html!{<button onclick={ctx.link().callback(|_| Msg::Resign)} class="text-base bg-neutral-800 hover:bg-neutral-700 px-3 py-1 rounded">{"Resign"}</button>}
                        } else {
                            html!{}
                        }
                    }
                </div>
                <div class="flex flex-col gap-3 h-[70dvh] bg-neutral-800/25 p-5 rounded-md">
                    {
                        if !self.solved && !self.resigned {
                            html!{
                                <div class="flex gap-3 items-center">
                                    <div class="w-[10%] flex justify-center py-6 lg:py-1 px-9 lg:px-5 mr-3 rounded-md">
                                        {self.typed_words.len()+1}
                                    </div>

                                    <div class="w-[90%] flex gap-3 items-center">
                                    {
                                        self.typed_word.clone().into_iter().map(|letter| {
                                            if letter.is_empty() {
                                                html!{<div class="bg-neutral-800 py-7 lg:py-2 px-9 lg:px-5 rounded-md">{EMPTY_LETTER}</div>}
                                            } else {
                                                html!{<div class="bg-neutral-800 py-7 lg:py-2 px-9 lg:px-5 rounded-md">{letter.to_uppercase()}</div>}
                                            }
                                        }).collect::<Html>()
                                    }
                                    </div>
                                </div>
                            }
                        } else {
                            html!{}
                        }
                    }
                    <div class="flex flex-col gap-3 overflow-y-scroll scroll-top-0">
                        {
                            self.typed_words.clone().into_iter().enumerate().map(|(i, word)| {
                                html!{
                                    <div class="flex gap-3 items-center">
                                    <div class="w-[10%] flex justify-center py-6 lg:py-1 px-9 lg:px-5 mr-3 rounded-md">
                                        {self.typed_words_indexes[i]}
                                    </div>
                                    <div class="w-[90%] flex gap-3 items-center animate-list">
                                    {
                                        word.into_iter().enumerate().map(|(i, letter)| {
                                            html!(<div class={classes!("py-7", "lg:py-2", "px-9", "lg:px-5", "rounded-md", self.letter_colour(&letter, i))}>{letter.to_uppercase()}</div>)
                                        }).collect::<Html>()
                                    }
                                    </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // KEYBOARD
                <div class="flex flex-col items-center justify-end gap-3 text-keysmall lg:text-keybig h-[20dvh]">
                    <div class="flex gap-3">
                        {
                            LETTERS_ROW1.into_iter().map(|letter| {
                                html!(<button onclick={ctx.link().callback(|_| Msg::AddLetter(letter.chars().nth(0).unwrap()))} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "transition-colors", "duration-500", self.key_colour(&letter.to_string()))}>{letter.to_uppercase()}</button>)
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="flex gap-3">
                        {
                            LETTERS_ROW2.into_iter().map(|letter| {
                                html!(<button onclick={ctx.link().callback(|_| Msg::AddLetter(letter.chars().nth(0).unwrap()))} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "transition-colors", "duration-500", self.key_colour(&letter.to_string()))}>{letter.to_uppercase()}</button>)
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="flex gap-3">
                        <button onclick={ctx.link().callback(|_| Msg::SubmitWord)} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "bg-neutral-800")}>{"RET"}</button>
                        {
                            LETTERS_ROW3.into_iter().map(|letter| {
                                html!(<button onclick={ctx.link().callback(|_| Msg::AddLetter(letter.chars().nth(0).unwrap()))} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "transition-colors", "duration-500", self.key_colour(&letter.to_string()))}>{letter.to_uppercase()}</button>)
                            }).collect::<Html>()
                        }
                        <button onclick={ctx.link().callback(|_| Msg::DeleteLetter)} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "bg-neutral-800")}>{"DEL"}</button>
                    </div>
                </div>

                {
                    if self.show_modal && (self.solved || self.resigned) {
                        let title_text = if self.solved { "You Won!" } else { "Game Over" };
                        let time = self.format_time_from_now();
                        let turns = self.typed_words.len();
                        let score = self.generate_score() as i64;
                        let word = self.wordle.to_uppercase();
                        let navigator = ctx.link().navigator();
                        let (btn_text, btn_cb): (&str, Callback<MouseEvent>) = if ctx.props().game_type == GameType::Unlimited {
                            ("New Game", ctx.link().callback(|_: MouseEvent| Msg::PlayAgain))
                        } else {
                            ("Play Unlimited", Callback::from(move |_: MouseEvent| {
                                if let Some(nav) = navigator.as_ref() {
                                    nav.push(&Route::Unlimited);
                                }
                            }))
                        };
                        html!{
                            <div class="fixed inset-0 bg-black/70 z-50 flex items-center justify-center p-4">
                                <div class="bg-neutral-800 rounded-lg p-6 max-w-md w-full text-2xl flex flex-col gap-4">
                                    <div class="flex justify-between items-center">
                                        <div class="font-bold">{title_text}</div>
                                    </div>
                                    <div class="flex flex-col gap-1 text-left">
                                        <div>{format!("Time:   {}", time)}</div>
                                        <div>{format!("Turns:  {}", turns)}</div>
                                        // <div>{format!("Score:  {}", score)}</div>
                                        <div>{format!("Word:   {}", word)}</div>
                                    </div>
                                    <div class="flex flex-col gap-2">
                                        <button onclick={btn_cb} class="bg-green-700 py-3 rounded">{btn_text}</button>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }

            </main>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let onkeypress = ctx.link().batch_callback(handle_keypress);

            let window = window().expect("Window not found!?");

            EventListener::new(&window, "keydown", move |e: &Event| {
                if let Ok(e) = e.clone().dyn_into::<KeyboardEvent>() {
                    onkeypress.emit(e);
                }
            })
            .forget();
        }
    }
}

impl Twordle {
    fn letter_colour(&self, letter: &String, i: usize) -> String {
        if self.wordle.chars().nth(i).unwrap().to_string() == *letter {
            "bg-green-700".to_string()
        } else if self.wordle.contains(letter) {
            "bg-yellow-700".to_string()
        } else {
            "bg-gray-700".to_string()
        }
    }
    fn key_colour(&self, letter: &String) -> String {
        if self.green_letters.contains(letter) {
            "bg-green-700".to_string()
        } else if self.yellow_letters.contains(letter) {
            "bg-yellow-700".to_string()
        } else if self.gray_letters.contains(letter) {
            "bg-gray-700".to_string()
        } else {
            "bg-neutral-800".to_string()
        }
    }

    fn format_time_from_now(&self) -> String {
        let now = self.time_finished.unwrap_or_else(Utc::now);

        let duration = now - self.time_started;

        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds();
        let milleseconds = duration.num_milliseconds();

        if minutes == 0 {
            if seconds == 0 {
                return format!("{} milliseconds!", milleseconds);
            }
            format!("{} seconds", seconds)
        } else if minutes == 1 {
            if seconds == 0 {
                "1 minute".to_string()
            } else {
                format!("1 minute and {} seconds", seconds)
            }
        } else {
            if seconds == 0 {
                format!("{} minutes", minutes)
            } else {
                format!("{} minutes and {} seconds", minutes, seconds)
            }
        }
    }

    fn generate_score(&self) -> f64 {
        let time_taken = self.time_finished.unwrap_or_else(Utc::now) - self.time_started;
        let chances_used = self.typed_words.len();

        let rating = 100.0 - (time_taken.num_seconds() as f64 + chances_used as f64);
        rating.max(1.0).min(100.0) as f64
    }

    fn reset_unlimited(&mut self) {
        self.wordle = pick_word::unlimited();
        self.solved = false;
        self.resigned = false;
        self.game_started = false;
        self.time_started = Utc::now();
        self.time_finished = None;
        self.show_modal = false;
        self.typed_word = vec![
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ];
        self.typed_words = vec![];
        self.typed_words_indexes = vec![];
        self.green_letters = vec![];
        self.yellow_letters = vec![];
        self.gray_letters = vec![];
    }
}

fn handle_keypress(e: KeyboardEvent) -> Option<Msg> {
    if e.key() == "Backspace" {
        return Some(Msg::DeleteLetter);
    }
    if e.key() == "Enter" {
        return Some(Msg::SubmitWord);
    }
    if e.key().len() == 1 {
        if let Some(c) = e.key().chars().next() {
            if c.is_alphabetic() {
                return Some(Msg::AddLetter(c));
            }
        }
    }
    None
}
