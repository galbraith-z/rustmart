use crate::types::CartProduct;
use yew::prelude::*;

pub struct Navbar {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
}

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let cart_value = self
            .props
            .cart_products
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

        html! {
            <div class="navbar">
                <div class="navbar_title">{"RUSTMART Mtg"}</div>
                <div class="navbar_cart_value">
                    {format!("${:.2}", cart_value)}
                    <button class="clear_cart_button" onClick="window.location.reload();">
                        <i class="fa-solid fa-arrow-rotate-right"></i>
                    </button>
                </div>
                <div>
                    <button class="back_button" onClick="window.history.back();">
                        <i class="fa-solid fa-arrow-left"></i>
                    </button>
                </div>
            </div>
        }
    }
}