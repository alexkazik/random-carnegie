use crate::{Data, Players};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use std::collections::BTreeMap;
use yew::virtual_dom::{VNode, VText};
use yew::{function_component, html, AttrValue, Html, Properties, ToHtml};
use yew_bootstrap::icons::BI;
use yewdux::mrc::Mrc;

pub(crate) struct Cards(Vec<u8>);

impl Default for Cards {
    fn default() -> Self {
        Self((0..20).collect())
    }
}

impl Cards {
    pub(crate) fn rand(&mut self, seed: u64) {
        self.0.sort();
        let mut rng = Pcg64Mcg::seed_from_u64(seed ^ 0x4362256e);
        self.0.shuffle(&mut rng);
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum City {
    // WEST
    Boise,
    Denver,     //3
    LosAngeles, //3
    Portland,
    Reno,
    SaltLakeCity,
    SanFrancisco,
    SantaFe,
    // MIDWEST
    Chicago,
    Cincinnati,
    Duluth,
    Fargo,
    KansasCity, //3
    Omaha,
    StLouis, //3
    StPaul,
    // EAST
    Albany, //3
    Boston, //3
    NewYork,
    Pittsburgh, //3
    Washington, //3
    // SOUTH
    Atlanta, //3
    Charleston,
    Dallas,
    Houston, //3
    Memphis,
    NewOrleans,
    SanAntonio,
}

impl ToHtml for City {
    fn to_html(&self) -> Html {
        VNode::VText(VText {
            text: AttrValue::Static(match self {
                City::Boise => "Boise",
                City::Denver => "Denver",
                City::LosAngeles => "Los Angeles",
                City::Portland => "Portland",
                City::Reno => "Reno",
                City::SaltLakeCity => "Salt Lake City",
                City::SanFrancisco => "San Francisco",
                City::SantaFe => "Santa Fe",
                City::Chicago => "Chicago",
                City::Cincinnati => "Cincinnati",
                City::Duluth => "Duluth",
                City::Fargo => "Fargo",
                City::KansasCity => "Kansas City",
                City::Omaha => "Omaha",
                City::StLouis => "St Louis",
                City::StPaul => "St Paul",
                City::Albany => "Albany",
                City::Boston => "Boston",
                City::NewYork => "New York",
                City::Pittsburgh => "Pittsburgh",
                City::Washington => "Washington",
                City::Atlanta => "Atlanta",
                City::Charleston => "Charleston",
                City::Dallas => "Dallas",
                City::Houston => "Houston",
                City::Memphis => "Memphis",
                City::NewOrleans => "New Orleans",
                City::SanAntonio => "San Antonio",
            }),
        })
    }
}

impl City {
    fn spaces(self) -> usize {
        match self {
            City::NewYork | City::Chicago | City::NewOrleans | City::SanFrancisco => 5,

            City::Albany
            | City::Boston
            | City::Pittsburgh
            | City::Washington
            | City::KansasCity
            | City::StLouis
            | City::Atlanta
            | City::Houston
            | City::Denver
            | City::LosAngeles => 3,

            City::Cincinnati
            | City::Duluth
            | City::Fargo
            | City::Omaha
            | City::StPaul
            | City::Charleston
            | City::Dallas
            | City::Memphis
            | City::SanAntonio
            | City::Boise
            | City::Portland
            | City::Reno
            | City::SaltLakeCity
            | City::SantaFe => 1,
        }
    }

    fn class(self) -> &'static str {
        match self {
            City::Boise
            | City::Denver
            | City::LosAngeles
            | City::Portland
            | City::Reno
            | City::SaltLakeCity
            | City::SanFrancisco
            | City::SantaFe => "west",
            City::Chicago
            | City::Cincinnati
            | City::Duluth
            | City::Fargo
            | City::KansasCity
            | City::Omaha
            | City::StLouis
            | City::StPaul => "midwest",
            City::Albany | City::Boston | City::NewYork | City::Pittsburgh | City::Washington => {
                "east"
            }
            City::Atlanta
            | City::Charleston
            | City::Dallas
            | City::Houston
            | City::Memphis
            | City::NewOrleans
            | City::SanAntonio => "south",
        }
    }
}

static CARDS: [&[City]; 20] = [
    &[City::SaltLakeCity, City::Reno],
    &[City::StLouis, City::Chicago],
    &[City::Boston, City::Washington],
    &[City::NewOrleans, City::Houston],
    &[City::SanFrancisco, City::LosAngeles],
    &[
        City::Cincinnati,
        City::Duluth,
        City::StLouis,
        City::KansasCity,
    ],
    &[
        City::Albany,
        City::NewYork,
        City::Washington,
        City::Pittsburgh,
    ],
    &[City::NewOrleans, City::Atlanta],
    &[City::Boston, City::NewYork],
    &[City::Chicago, City::Omaha],
    &[City::Fargo, City::StPaul],
    &[City::Pittsburgh, City::NewYork],
    &[City::SanAntonio, City::Memphis, City::Dallas],
    &[City::Portland, City::Boise, City::Denver, City::LosAngeles],
    &[
        City::NewYork,
        City::Chicago,
        City::NewOrleans,
        City::SanFrancisco,
    ],
    &[City::Pittsburgh, City::Boston, City::Albany],
    &[City::SanFrancisco, City::SantaFe],
    &[
        City::NewOrleans,
        City::Atlanta,
        City::Houston,
        City::Charleston,
    ],
    &[City::SanFrancisco, City::Denver],
    &[City::KansasCity, City::Chicago],
];

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub data: Mrc<Data>,
}

#[function_component]
pub(crate) fn SetupPane(props: &Props) -> Html {
    let data = props.data.borrow();

    let mut disks = match data.players {
        Players::All => 0,
        Players::Four => 0,
        Players::Three => 9,
        Players::Two => 18,
    };

    let mut donations = vec![vec![false; 4]; 5];
    let mut cities = BTreeMap::new();

    if disks > 0 {
        'outer: for idx in data.cards.0.iter().cloned() {
            let cs = CARDS[idx as usize];
            let x = (idx / 5) as usize;
            let y = (idx % 5) as usize;
            if !donations[y][x] {
                donations[y][x] = true;
                disks -= 1;
                if disks == 0 {
                    break;
                }
            }
            for c in cs {
                let ce = cities.entry(c).or_insert(0);
                if *ce < c.spaces() {
                    *ce += 1;
                    disks -= 1;
                    if disks == 0 {
                        break 'outer;
                    }
                }
            }
        }

        let hd = donations.into_iter().map(|row| {
            let row = row
                .into_iter()
                .map(|e| html! {<td>{if e {{BI::CIRCLE_FILL}} else {BI::CIRCLE}}</td>});
            html! {<tr>{for row}</tr>}
        });
        let hc = cities.into_iter().map(|(ci, co)| {
            if co == 1 {
                html! {<><span class={"hidden"}>{"2 × "}</span><span class={ci.class()}>{ci}</span><span class={"hidden"}>{" × 0"}</span><br/></>}
            } else {
                html! {<>{co}{" × "}<span class={ci.class()}>{ci}</span><span class={"hidden"}>{" × 0"}</span><br/></>}
            }
        });

        html! {
            <>
            <table align="center"><thead><th colspan="4">{"Blocked donations"}</th></thead><tbody>{for hd}</tbody></table>
            <table align="center"><thead><th colspan="4">{"Blocked cities"}</th></thead><tbody>{for hc}</tbody></table>
            </>
        }
    } else {
        Default::default()
    }
}
