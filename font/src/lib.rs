#![recursion_limit = "512"]
use serde_derive::{Deserialize, Serialize};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{route::Route,service::RouteService,Switch};

pub struct Model {
    link: ComponentLink<Self>,
    route_service: RouteService<()>,
    route: Route<()>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
    Nope,
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/a/{anything}"]
    A(String),
    #[to = "/b/{anything}/{number}"]
    B { anything: String, number: u32 },
    #[to = "/c"]
    C,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);
        Model {
            link,
            route_service,
            route,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
                // This might be derived in the future
                let route_string = match route {
                    AppRoute::A(s) => format!("/a/{}", s),
                    AppRoute::B { anything, number } => format!("/b/{}/{}", anything, number),
                    AppRoute::C => "/c".to_string(),
                };
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: (), 
                };
            }
            Msg::Nope => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="todomvc-wrapper">
                { self.nav() }
                { self.apps() }
                <div>
                <nav class="menu",>
                    <button onclick=&self.change_route(AppRoute::A("lorem".to_string())) > {"A"} </button>
                    <button onclick=&self.change_route(AppRoute::B{anything: "hello".to_string(), number: 42}) > {"B"} </button>
                    <button onclick=&self.change_route(AppRoute::C) > {"C"} </button>
                </nav>
                <div>
                {
                    match AppRoute::switch(self.route.clone()) {
                        Some(AppRoute::A(thing)) => VNode::from(thing.as_str()),
                        Some(AppRoute::B{anything, number}) => html!{<div> {anything} {number} </div>},
                        Some(AppRoute::C) => VNode::from("C"),
                        None => VNode::from("404")
                    }
                }
                </div>
            </div>
            </div>
        }
    }
}

impl Model {
    fn nav(&self) -> Html {
        html! {
            <nav class="uk-navbar-container" uk-navbar={ true }>
            <div class="uk-navbar-left">
                <ul class="uk-navbar-nav">
                    <li class="uk-active"><a href="#">{ "开源市场|云原生市场" }</a></li>
                    <li>
                        <a href="#">{ "友情链接" }</a>
                        <div class="uk-navbar-dropdown">
                            <ul class="uk-nav uk-navbar-dropdown-nav">
                                <li class="uk-active"><a href="#">{ "云原生实验室" }</a></li>
                                <li><a href="#">{ "sealos" }</a></li>
                                <li><a href="#">{ "kuboard" }</a></li>
                            </ul>
                        </div>
                    </li>
                    <li><a href="#">{ "签约作者" }</a></li>
                </ul>
            </div>
            <div class="uk-navbar-right">
                <p uk-icon="icon: github; ratio: 2.3" style="margin-right:10px;"/>
            </div>
            </nav>
        }
    }

    fn apps(&self) -> Html {
        html! {
            <ul uk-accordion={ true } class="uk-container">
            <li class="uk-open">
                <a class="uk-accordion-title" href="#">{ "云内核 kubernetes" }</a>
                <div class="uk-accordion-content">
                { self.apps_table() }
                </div>
            </li>
            <li>
                <a class="uk-accordion-title" href="#">{ "云驱动" }</a>
                <div class="uk-accordion-content">
                { self.apps_table() }
                </div>
            </li>
            <li>
                <a class="uk-accordion-title" href="#">{ "中间件" }</a>
                <div class="uk-accordion-content">
                { self.apps_table() }
                </div>
            </li>
            </ul>
        }
    }

    fn apps_table(&self) -> Html {
        html! {
            <table class="uk-table">
            <thead>
                <tr>
                    <th>{ "名称" }</th>
                    <th>{ "价格" }</th>
                    <th>{"描述"}</th>
                    <th>{"使用次数"}</th>
                    <th>{"评分"}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{ "kubernetes离线包" }</td>
                    <td>{"50"}</td>
                    <td>{"一键安装kubernetes高可用集群"}</td>
                    <td>{"2020"}</td>
                    <td><p uk-icon="star" /><p uk-icon="star" /><p uk-icon="star" /><p uk-icon="star" /></td>
                </tr>
                <tr>
                    <td>{ "ARM kubernetes离线包" }</td>
                    <td>{"99"}</td>
                    <td>{"ARM版 一键安装kubernetes高可用集群"}</td>
                    <td>{"2020"}</td>
                    <td><p uk-icon="star" /><p uk-icon="star" /><p uk-icon="star" /></td>
                </tr>
            </tbody>
            </table>
        }
    }
}
