use yew::{html, Html};
pub fn app_info(name: String) -> Html {
    html!{
        <div class="uk-container">
        <p> { name } {"商品名"} </p>
        </div>
    }
}