use yew::prelude::*;
use crate::video::Video;

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>
}

#[function_component(VideosList)]
pub fn videos_list(props: &VideosListProps) -> Html {
    props.videos.iter().map(|video| {
        let on_video_select = {
            let on_click = props.on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}