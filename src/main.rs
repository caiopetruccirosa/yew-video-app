mod video;

use yew::prelude::*;

use crate::video::*;

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <videodetails::VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <videoslist::VideosList videos={get_videos()} on_click={on_video_select}/>
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}