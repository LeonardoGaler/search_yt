use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{function_component, html, use_state, Callback, Properties, UseStateHandle};
fn main() {
   yew::start_app::<App>();
}

#[derive(Clone)]
struct Video {
    id: String,
    name: String,
}

#[function_component(App)]
fn app() -> Html {
    let video :UseStateHandle<Option<Video>> = use_state(|| None);
    let on_search = {
        let video = video.clone();
        Callback::from(move |text_to_search| {
        let video_id = search_youtube(text_to_search);
        video.set(Some(Video{
            id: video_id,
            name: "name".to_string()
        }))
        })
    };

    let video_selection = match (*video).clone(){
        Some(video) => html!{
            <VideoPlayer video_title={video.name} video_id={video.id}/>
        },
        None => html!{}
    };
    html! {
        <main>
        <VideoControls on_search={on_search}/>
        {video_selection}
        <VideoPlayer video_title="{video.unwrap().name}" video_id="{video.unwrap().id}"/>
        </main>
    }
}

fn search_youtube(text_to_search: String) -> String {
    web_sys::console::log_1(&text_to_search.into());
    String::from("dQw4w9WgXcQ")
}

#[derive(Properties, PartialEq)]
struct VideoControlsProps {
    on_search: Callback<String>,
}

#[function_component(VideoControls)]
fn video_controls(props: &VideoControlsProps) -> Html {
    let text_to_search = use_state(|| String::new());
    let handle_input = {
        let text_to_search = text_to_search.clone();
        Callback::from(move |input_event| {
        let text = get_value_from_input_event(input_event);
        text_to_search.set(text);
        })
    };
    
    let on_search_pressed = { 
        let on_search = props.on_search.clone();
        Callback::from(move |_| on_search.emit(text_to_search.to_string()))
    };
    html! {
        <div>
            <h1>{ "Search a word" }</h1>
            <input type="text" oninput={handle_input}/>
            <div><button onclick={on_search_pressed}>{"SEARCH"}</button></div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps{
    video_id: String,
    video_title: String,
}

#[function_component(VideoPlayer)]
fn video_player(props: &VideoSectionProps) -> Html {
    let yt_url = format!("https://www.youtube.com/embed/{}", props.video_id);
    html! {
        <div>
            <iframe with="1920" height="1080" src={yt_url}></iframe>
        </div>
    }
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}