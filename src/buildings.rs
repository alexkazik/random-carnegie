use crate::buildings::building::Building;
use crate::{Data, Players};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use yew::{function_component, html, Html, Properties};
use yew_bootstrap::icons::BI;
use yewdux::functional::use_store;
use yewdux::mrc::Mrc;
use yewdux::store::Store;

mod building {
    use crate::buildings::Tiles;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub(super) struct Building(u8);

    impl Building {
        #[inline]
        pub(super) fn new(value: u8) -> Self {
            Self(value)
        }

        #[inline]
        pub(super) fn value(&self) -> u8 {
            self.0
        }

        #[inline]
        pub(super) fn row(&self) -> usize {
            (((self.0 - 1) >> 2) & 3) as usize
        }

        #[inline]
        pub(super) fn blue(&self) -> bool {
            (if self.0 <= 16 { self.0 - 1 } else { self.0 }) & 3 == 3
        }

        #[inline]
        pub(super) fn is_tile(&self, tiles: Tiles) -> bool {
            match tiles {
                Tiles::Base => self.0 <= 16,
                Tiles::Both => true,
                Tiles::Expansion => self.0 >= 17,
            }
        }
    }
}

pub(crate) struct Buildings(Vec<Building>);

impl Default for Buildings {
    fn default() -> Self {
        let mut b = Vec::with_capacity(64);
        for i in 1..=32 {
            b.push(Building::new(i));
            b.push(Building::new(i));
        }
        Self(b)
    }
}

impl Buildings {
    pub(crate) fn rand(&mut self, seed: u64) {
        self.0.sort();
        let mut rng = Pcg64Mcg::seed_from_u64(seed);
        self.0.shuffle(&mut rng);
    }
}

impl Display for Buildings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut r = Vec::from_iter(self.0.iter().take(32));
        r.sort();
        for (i, b) in r.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{:02}", b.value())?;
        }
        Ok(())
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub data: Mrc<Data>,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Deserialize, Serialize)]
enum Tiles {
    #[default]
    Base,
    Both,
    Expansion,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum LimitTypes {
    #[default]
    Four = 4,
    Five = 5,
    Six = 6,
    All = 8,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Deserialize, Serialize)]
enum Permanent {
    Zero,
    ZeroPlus,
    #[default]
    One,
    OnePlus,
    Two,
}

impl Permanent {
    fn min(&self) -> usize {
        match self {
            Permanent::Zero => 0,
            Permanent::ZeroPlus => 0,
            Permanent::One => 1,
            Permanent::OnePlus => 1,
            Permanent::Two => 2,
        }
    }

    fn max(&self) -> usize {
        match self {
            Permanent::Zero => 0,
            Permanent::ZeroPlus => 2,
            Permanent::One => 1,
            Permanent::OnePlus => 2,
            Permanent::Two => 2,
        }
    }
}

#[derive(Store, PartialEq, Default, Deserialize, Serialize, Clone)]
#[store(storage = "local")]
struct State {
    tiles: Tiles,
    limit: LimitTypes,
    permanent: Permanent,
}

#[function_component]
pub(crate) fn BuildingsPane(props: &Props) -> Html {
    let data = props.data.borrow();
    let (state, dispatch) = use_store::<State>();

    let set_base = dispatch.reduce_mut_callback(|state| state.tiles = Tiles::Base);
    let set_both = dispatch.reduce_mut_callback(|state| state.tiles = Tiles::Both);
    let set_expansion = dispatch.reduce_mut_callback(|state| state.tiles = Tiles::Expansion);

    let mut bs = vec![BTreeMap::new(); 4];
    let mut count = 0;
    let limit = state.limit as u8 as usize;
    let blue_min = state.permanent.min();
    let blue_max = state.permanent.max();
    let mut blues_missing = blue_min * 4;
    for b in data
        .buildings
        .0
        .iter()
        .copied()
        .filter(|b| b.is_tile(state.tiles))
    {
        let b_is_blue = b.blue();
        let row = &mut bs[b.row()];
        let row_types = row.len();
        let row_len = row.values().map(|x: &Vec<usize>| x.len()).sum::<usize>();
        let row_blue = row.keys().copied().filter(Building::blue).count();
        if row_len < 8 {
            if let Some(e) = row.get_mut(&b) {
                if count == 32 - blues_missing {
                    continue;
                }
                if row_len - row_blue + blue_min >= 8 {
                    continue;
                }
                e.push(count);
            } else {
                if row_types == limit {
                    continue;
                }
                if b_is_blue {
                    if row_blue == blue_max {
                        continue;
                    }
                    if row_blue < blue_min && blues_missing > 0 {
                        blues_missing -= 1;
                    }
                } else {
                    if count == 32 - blues_missing {
                        continue;
                    }
                    if row_types - row_blue + blue_min >= limit {
                        continue;
                    }
                    if row_len - row_blue + blue_min >= 8 {
                        continue;
                    }
                }

                row.insert(b, vec![count]);
            }
            count += 1;
            if count == 32 {
                break;
            }
        }
    }

    let buildings = match data.players {
        Players::All => 32,
        Players::Four => 7,
        Players::Three => 3,
        Players::Two => 1,
    };

    let bs = bs.iter().map(|rx| {
        let rx = rx.iter().map(|(cx, cc)| {
            let blue =cx.blue();
            let cc = cc.iter().filter(|x|**x&buildings!=buildings).count();
            if cc > 0 {
                let cc = if cc == 1 {BI::LAYERS_HALF}else{BI::LAYERS_FILL};
                html! {
                <td><small>{cc}{" "}</small><span style={if blue {"color: blue"}else{""}}>{cx.value()}</span></td>
            }
            }else{
                html!{<td/>}
            }
        });
        html! {<tr>{for rx}</tr>}
    });

    html! {
        <>
        <div>
            {"Department tiles: "}
            <div class="btn-group" role="group">
                <input
                    type="radio"
                    class="btn-check"
                    name="tiles"
                    id="tiles0"
                    autocomplete="off"
                    checked={state.tiles == Tiles::Base}
                    onchange={set_base}
                />
                <label class="btn btn-outline-primary" for="tiles0">{"Base"}</label>

                <input
                    type="radio"
                    class="btn-check"
                    name="tiles"
                    id="tiles1"
                    autocomplete="off"
                    checked={state.tiles == Tiles::Both}
                    onchange={set_both}
                />
                <label class="btn btn-outline-primary" for="tiles1">{"Base+Expansion"}</label>

                <input
                    type="radio"
                    class="btn-check"
                    name="tiles"
                    id="tiles2"
                    autocomplete="off"
                    checked={state.tiles == Tiles::Expansion}
                    onchange={set_expansion}
                />
                <label class="btn btn-outline-primary" for="tiles2">{"Expansion"}</label>
            </div>
        </div>
        if state.tiles == Tiles::Both {
            <div>
                {"Different Departments per row: "}
                <div class="btn-group" role="group">
                    <input
                        type="radio"
                        class="btn-check"
                        name="limit"
                        id="limit0"
                        autocomplete="off"
                        checked={state.limit == LimitTypes::Four}
                        onchange={dispatch.reduce_mut_callback(|state|state.limit=LimitTypes::Four)}
                    />
                    <label class="btn btn-outline-primary" for="limit0">{"4"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="limit"
                        id="limit1"
                        autocomplete="off"
                        checked={state.limit == LimitTypes::Five}
                        onchange={dispatch.reduce_mut_callback(|state|state.limit=LimitTypes::Five)}
                    />
                    <label class="btn btn-outline-primary" for="limit1">{"up to 5"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="limit"
                        id="limit2"
                        autocomplete="off"
                        checked={state.limit == LimitTypes::Six}
                        onchange={dispatch.reduce_mut_callback(|state|state.limit=LimitTypes::Six)}
                    />
                    <label class="btn btn-outline-primary" for="limit2">{"up to 6"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="limit"
                        id="limit3"
                        autocomplete="off"
                        checked={state.limit == LimitTypes::All}
                        onchange={dispatch.reduce_mut_callback(|state|state.limit=LimitTypes::All)}
                    />
                    <label class="btn btn-outline-primary" for="limit3">{"up to 8"}</label>
                </div>
            </div>
            <div>
                {"Permanent Departments per row: "}
                <div class="btn-group" role="group">
                    <input
                        type="radio"
                        class="btn-check"
                        name="permanent"
                        id="permanent0"
                        autocomplete="off"
                        checked={state.permanent == Permanent::Zero}
                        onchange={dispatch.reduce_mut_callback(|state|state.permanent=Permanent::Zero)}
                    />
                    <label class="btn btn-outline-primary" for="permanent0">{"0"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="permanent"
                        id="permanent0p"
                        autocomplete="off"
                        checked={state.permanent == Permanent::ZeroPlus}
                        onchange={dispatch.reduce_mut_callback(|state|state.permanent=Permanent::ZeroPlus)}
                    />
                    <label class="btn btn-outline-primary" for="permanent0p">{"0+"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="permanent"
                        id="permanent1"
                        autocomplete="off"
                        checked={state.permanent == Permanent::One}
                        onchange={dispatch.reduce_mut_callback(|state|state.permanent=Permanent::One)}
                    />
                    <label class="btn btn-outline-primary" for="permanent1">{"1"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="permanent"
                        id="permanent2"
                        autocomplete="off"
                        checked={state.permanent == Permanent::OnePlus}
                        onchange={dispatch.reduce_mut_callback(|state|state.permanent=Permanent::OnePlus)}
                    />
                    <label class="btn btn-outline-primary" for="permanent2">{"1+"}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="permanent"
                        id="permanent3"
                        autocomplete="off"
                        checked={state.permanent == Permanent::Two}
                        onchange={dispatch.reduce_mut_callback(|state|state.permanent=Permanent::Two)}
                    />
                    <label class="btn btn-outline-primary" for="permanent3">{"2"}</label>
                </div>
            </div>
        }
        <table align="center">
            {for bs}
        </table>
        </>
    }
}
