use crate::buildings::{Buildings, BuildingsPane};
use crate::setup::{Cards, SetupPane};
use gloo_history::{BrowserHistory, History, HistoryListener};
use gloo_storage::errors::StorageError;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{html, Component, Context, Html, NodeRef};
use yew_bootstrap::component::{Button, ButtonSize};
use yew_bootstrap::icons::BI;
use yew_bootstrap::util::Color;
use yewdux::mrc::Mrc;

pub(crate) mod buildings;
pub(crate) mod setup;

#[derive(Default)]
pub(crate) struct Data {
    players: Players,
    buildings: Buildings,
    cards: Cards,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum Players {
    All = 0,
    #[default]
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Default, Deserialize, Serialize)]
struct State {
    players: Players,
}

impl State {
    const KEY: &'static str = "random_carnegie::State";
}

pub(crate) struct App {
    seed: u64,
    data: Mrc<Data>,
    edit_seed: bool,
    inp_seed: NodeRef,
    // Router
    browser_history: BrowserHistory,
    _history_listener: HistoryListener,
    base: String,
    redirect_counter: usize,
}

impl App {
    fn rand(&mut self) {
        self.seed = Into::<u64>::into(rand::random::<u32>()) % 100000000;
        self.set_seed(self.seed);
    }
    fn set_seed(&mut self, seed: u64) {
        let mut data = self.data.borrow_mut();
        data.buildings.rand(seed);
        data.cards.rand(seed);
    }
}

enum AppMsg {
    HistoryChanged,
    Rand,
    Players(Players),
    EditSeed,
    SetSeed,
    CancelSeed,
    UnusedKeyboardSeed,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let browser_history = BrowserHistory::new();
        ctx.link().send_message(AppMsg::HistoryChanged);
        let link_cloned = ctx.link().clone();
        let history_listener =
            browser_history.listen(move || link_cloned.send_message(AppMsg::HistoryChanged));

        let state = LocalStorage::get::<State>(State::KEY).unwrap_or_default();

        let mut app = App {
            seed: 0,
            data: Mrc::new(Data {
                players: state.players,
                ..Default::default()
            }),
            edit_seed: false,
            inp_seed: NodeRef::default(),
            browser_history,
            _history_listener: history_listener,
            base: yew_router::utils::fetch_base_url().unwrap_or_default(),
            redirect_counter: 0,
        };
        app.rand();
        app
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::HistoryChanged => {
                let loc = self.browser_history.location();
                self.seed = loc
                    .hash()
                    .trim_start_matches('#')
                    .parse()
                    .unwrap_or_else(|_| Into::<u64>::into(rand::random::<u32>()))
                    % 100000000;
                self.set_seed(self.seed);

                let new_path = format!("{}/#{:08}", self.base, self.seed);

                if format!("{}{}", loc.path(), loc.hash()) == new_path || self.redirect_counter > 3
                {
                    true
                } else {
                    self.browser_history.replace(&new_path);
                    self.redirect_counter += 1;
                    false
                }
            }
            AppMsg::Rand => {
                self.rand();
                let new_path = format!("{}#{:08}", self.base, self.seed);
                self.browser_history.push(new_path);
                self.redirect_counter = 0;
                false
            }
            AppMsg::Players(players) => {
                let mut data = self.data.borrow_mut();
                if data.players != players {
                    data.players = players;
                    let _: Result<(), StorageError> =
                        LocalStorage::set(State::KEY, State { players });
                    true
                } else {
                    false
                }
            }
            AppMsg::EditSeed => {
                let inp_seed = self.inp_seed.cast::<HtmlInputElement>().unwrap();
                inp_seed.set_class_name("seed_input");
                let _ = inp_seed.focus();
                self.edit_seed = true;
                true
            }
            AppMsg::UnusedKeyboardSeed => false,
            AppMsg::SetSeed => {
                self.edit_seed = false;
                let inp_seed = self.inp_seed.cast::<HtmlInputElement>().unwrap();
                let _ = inp_seed.blur();
                inp_seed.set_class_name("seed_input_hidden");

                if let Ok(new_seed) = inp_seed.value().parse() {
                    self.seed = new_seed;
                    let new_path = format!("{}#{:08}", self.base, self.seed);
                    self.browser_history.push(new_path);
                    self.redirect_counter = 0;
                }

                true
            }
            AppMsg::CancelSeed => {
                self.edit_seed = false;
                let inp_seed = self.inp_seed.cast::<HtmlInputElement>().unwrap();
                let _ = inp_seed.blur();
                inp_seed.set_class_name("seed_input_hidden");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn key_to_message(e: KeyboardEvent) -> AppMsg {
            match e.key().as_str() {
                "Escape" => AppMsg::CancelSeed,
                "Enter" => AppMsg::SetSeed,
                _ => AppMsg::UnusedKeyboardSeed,
            }
        }

        let data = self.data.borrow();
        html! {
            <div class="app-wrap">
                <nav class="navbar sticky-top bg-body-tertiary">
                    <div class="container-fluid">
                        <a class="navbar-brand">
                            {"Unofficial Carnegie Randomizer"}
                        </a>
                    </div>
                </nav>
                <main class="container py-4" style="text-align: center">
                    <div class="d-grid gap-3">
                        <div>
                            {"Seed: "}
                            <input
                                class="seed_input_hidden"
                                type="text"
                                value={format!("{:08}",self.seed)}
                                maxlength=8
                                size=12
                                inputmode="numeric"
                                pattern="\\d*"
                                onkeyup={ctx.link().callback(key_to_message)}
                                ref={&self.inp_seed}
                            />
                            if self.edit_seed {
                                {" "}
                                <Button style={Color::Success} size={ButtonSize::Small} onclick={ctx.link().callback(|_| AppMsg::SetSeed)}>
                                    {BI::CHECK}
                                </Button>
                                {" "}
                                <Button style={Color::Secondary} size={ButtonSize::Small} onclick={ctx.link().callback(|_| AppMsg::CancelSeed)}>
                                    {BI::X}
                                </Button>
                            }else{
                                <a onclick={ctx.link().callback(|_| AppMsg::EditSeed)}>{format!("{:08}",self.seed)}</a>
                                {" "}
                                <Button size={ButtonSize::Small} onclick={ctx.link().callback(|_| AppMsg::Rand)}>
                                    {BI::ARROW_CLOCKWISE}
                                </Button>
                            }
                        </div>
                        <div>
                            {"Show for: "}
                            <div class="btn-group" role="group">
                                <input
                                    type="radio"
                                    class="btn-check"
                                    name="players"
                                    id="players0"
                                    autocomplete="off"
                                    checked={data.players == Players::All}
                                    onchange={ctx.link().callback(|_| AppMsg::Players(Players::All))}
                                />
                                <label class="btn btn-outline-primary" for="players0">{"All"}</label>

                                <input
                                    type="radio"
                                    class="btn-check"
                                    name="players"
                                    id="players1"
                                    autocomplete="off"
                                    checked={data.players == Players::Four}
                                    onchange={ctx.link().callback(|_| AppMsg::Players(Players::Four))}
                                />
                                <label class="btn btn-outline-primary" for="players1">{"4p"}</label>

                                <input
                                    type="radio"
                                    class="btn-check"
                                    name="players"
                                    id="players2"
                                    autocomplete="off"
                                    checked={data.players == Players::Three}
                                    onchange={ctx.link().callback(|_| AppMsg::Players(Players::Three))}
                                />
                                <label class="btn btn-outline-primary" for="players2">{"3p"}</label>

                                <input
                                    type="radio"
                                    class="btn-check"
                                    name="players"
                                    id="players3"
                                    autocomplete="off"
                                    checked={data.players == Players::Two}
                                    onchange={ctx.link().callback(|_| AppMsg::Players(Players::Two))}
                                />
                                <label class="btn btn-outline-primary" for="players3">{"2p"}</label>
                            </div>
                        </div>
                        <BuildingsPane data={self.data.clone()} />
                        <SetupPane data={self.data.clone()} />
                    </div>
                </main>
                <nav class="navbar sticky-bottom bg-body-tertiary">
                    <div class="container-fluid">
                        <b class="mb-0">
                            {"Written by Alex."}
                        </b>

                        <div class="ms-auto">
                            {"Version: "}{env!("CARGO_PKG_VERSION")}
                            <a
                                href="https://github.com/alexkazik/random-carnegie"
                                target="_blank"
                                class="btn btn-dark btn-sm ms-4"
                            >
                                {BI::GITHUB}{" Source"}
                            </a>
                        </div>
                    </div>
                </nav>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
