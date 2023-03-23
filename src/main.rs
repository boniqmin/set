mod set;
use gloo_timers::callback::Timeout;
use set::{Board, Card};
use yew::prelude::*;

// mod rendercards;
// use crate::rendercards::CardImg;

// #[function_component(App)]
// fn app() -> Html {
// }

pub enum Msg {
    CardSelected(usize),
    Reset,
    Expand,
    FullSelection,
}

// fn app() -> Html {
//     html!{
//         <>
//             <Board
//         </>
//     }
// }

#[derive(PartialEq, Properties)]
struct CardProp {
    card: Card,
    selected: bool,
}

#[function_component(CardImg)]
fn card_img(card: &CardProp) -> Html {
    let shapes = if card.card.amount == 0 {
        html! {shape_svg(&card.card, 0.,0.,1.)}
    } else if card.card.amount == 1 {
        html! {  <>
            {shape_svg(&card.card, 0.,0.,0.5)}
            {shape_svg(&card.card, 100.,100.,0.5)}
            </>
        }
    } else if card.card.amount == 2 {
        html! { <>
            {shape_svg(&card.card, 50.,0.,0.5)}
            {shape_svg(&card.card, 0.,100.,0.5)}
            {shape_svg(&card.card, 100.,100.,0.5)}
            </>
        }
    } else {
        panic!("Invalid amount code")
    };

    let shadow = if card.selected {
        // TODO: does this still do anything?
        "shadow-in"
    } else {
        "shadow-out"
    };
    let size;
    if card.selected {
        size = "200";
    } else {
        size = "180";
    }

    html! {<svg height={size} width={size} viewBox="0 0 200 200"> // set height and width to change card size
    <rect class={shadow} x="10" y="10" rx="20" ry="20" width="180" height="180"
    style="fill:WhiteSmoke;stroke:SlateGray;stroke-width:5" />{shapes}
    {
        if card.selected {
            html!{<path fill-rule="evenodd" d="M-10,-10  h220 v220 h-220 z
            M7,30 v140 a 23 23 0 0 0 23 23 h140 a 23 23 0 0 0 23 -23
             v-140 a 23 23 0 0 0 -23 -23 h-140 a 23 23 0 0 0 -23 23 z" class="shadow-pressed" stroke="WhiteSmoke"
            fill="WhiteSmoke" />} // creates the shape that is inside the square but outside the rounded rect
            // so that we can cast a shadow on the rect
        }
        else {
            html!{}
        }
    }
    </svg>}
}

fn shape_svg(card: &Card, translate_x: f32, translate_y: f32, scale: f32) -> Html {
    let color_str = match card.color {
        0 => "red",
        1 => "green",
        2 => "blue",
        _ => {
            panic!("Invalid card")
        }
    };

    let diag_hatch = html! {
        <defs>
        <pattern id="diagonalHatchred" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
            <line x1="0" y1="0" x2="0" y2="10" style="stroke:red; stroke-width:4" />
        </pattern>
        <pattern id="diagonalHatchgreen" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
            <line x1="0" y1="0" x2="0" y2="10" style="stroke:green; stroke-width:4"/>
        </pattern>
        <pattern id="diagonalHatchblue" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
            <line x1="0" y1="0" x2="0" y2="10" style="stroke:blue; stroke-width:4" />
        </pattern>
        </defs>
    };
    // let style = format!("fill:{color_str}");
    let fill = match card.filling {
        0 => "WhiteSmoke".to_owned(),
        1 => format!("url(#diagonalHatch{color_str})"),
        2 => color_str.to_owned(), //format!("{color_str}"),
        _ => panic!("Invalid filling code"),
    };
    let style = format!("stroke-width:5;stroke:{color_str}"); //;{style}");
    let shape_svg = match card.shape {
        0 => {
            html! {<polygon points="100,50 150,150 50,150" stroke-linejoin={"round"} style={style} fill={fill}/>}
        }
        1 => {
            html! {<polygon points="50,50 150,50 150,150 50 150" stroke-linejoin="round" style={style} fill={fill}/>}
        }
        2 => html! {<circle cx="100" cy="100" r="50" style={style} fill={fill}/>},
        _ => {
            panic!("Invalid shape code")
        }
    };

    html! {<g transform={format!("translate({translate_x} {translate_y}) scale({scale} {scale})")}>
        {diag_hatch} // since this is constant, move somewhere?
        {shape_svg}
        // <text x={"100"} y={"100"}>{card.amount+1}</text>
    </g>}
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid = self
            .cards
            .iter()
            .enumerate()
            .map(move |(i, card)| {
                // let on_card_select = {
                //     let on_click = on_click.clone();
                //     let card = card.clone();
                //     Callback::from(move |_| on_click.emit(card.clone()))
                // };

                let on_card_click = ctx.link().callback(move |_| Msg::CardSelected(i));
                let class;
                if self.card_selection.is_selected(i) {
                    class = "grid-item selected"
                } else {
                    class = "grid-item unselected"
                }

                // html! {<button class={class} onclick={on_card_click}><img src={format!("cards/{card}.png")} alt={format!("{card}")}/></button>}
                html! {<button class={class} onclick={on_card_click}><CardImg card={card.clone()} selected={self.card_selection.is_selected(i)}/></button>}
            })
            .collect::<Html>();

        let reset_onclick = ctx.link().callback(|_| Msg::Reset);
        let expand_onclick = ctx.link().callback(|_| Msg::Expand);

        let sets_available = self.count_sets();
        let available_plural_s = match sets_available {
            1 => "",
            _ => "s",
        };
        let found_plural_s = match self.num_sets {
            1 => "",
            _ => "s",
        };

        html! {<>
        <h1> {"Play set!"}  </h1>
        <div class="infobox">{format!("{} set{} found", self.num_sets, found_plural_s)} </div>

        <div class="infobox">{format!("{} set{} available", sets_available, available_plural_s)} </div>
        <button onclick={reset_onclick} class="gamebutton">{"Reset"}</button>
        if self.times_expanded < 3 && !self.deck.is_empty() {  // 3 times expanded => 21 cards, so we always have a set (cap set problem)
            <button onclick={expand_onclick} class="gamebutton">{"Expand"}</button>
        }
        <div class="grid-container">{grid}</div>
        </>}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.finished {
            return false;
        }
        match msg {
            Msg::CardSelected(card) => {
                if self.card_selection.add_next_toggle(card).is_err() {
                    return false; // happens when player click card during the delay between selecting 3 cards and processing
                }

                if self.card_selection.is_full() {
                    let handle_full_selection = _ctx.link().callback(|()| Msg::FullSelection);
                    let timeout = Timeout::new(300, move || {
                        handle_full_selection.emit(());
                    });
                    timeout.forget();
                }
                // if self.deck.is_empty() {
                //     self.finished = true; // TODO: make screen for when game is finished
                // } // game is finished if deck is empty and there are no remaining sets in cards

                if self.cards.is_empty() {
                    self.finished = true;
                }
                true
            }
            Msg::Reset => {
                self.reset();
                true
            }

            Msg::Expand => {
                self.times_expanded += 1;
                self.expand();
                true
            }

            Msg::FullSelection => {
                if self.card_selection.is_set(&self.cards) {
                    self.num_sets += 1;
                    if self.times_expanded > 0 {
                        // self.card_selection.remove_cards(&mut self.cards);
                        self.remove_cards();
                        self.times_expanded -= 1;
                    } else {
                        // self.card_selection
                        //     .replace_cards_from_deck(&mut self.cards, &mut self.deck);
                        self.replace_cards();
                    }
                    // if self.deck.is_empty() {
                    //     self.num_sets = 0;
                    //     self.deck = Deck::new_shuffled();
                    // }  // TODO: potentially add "infinite mode" with this code
                }

                self.card_selection.clear();
                true
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::start_app::<Board>();
    yew::Renderer::<Board>::new().render();
}
