import { Match, Suspense, Switch, createEffect, createResource, createSignal, mergeProps } from "solid-js"
import Api from "../public_src/api";
import "./media_display.css"
export default function Media(props) {
    let extend_name = () => props.extend_name;
    let link = () => Api.file_root + props.path;
    let small = () => props.size < 1024 * 20;

    const texts = ["txt", "json", "gitignore", "md", "bat", "rs", "toml", "js", "html", "css", "jsx"]
    const videoes = ["mp4", "avi"]
    const audios = ["flac", "mp3", "m4a"]
    const pictures = ["png", "jpg"]
    // let get_content = async ([extend_name, link]) => {
    //     if (texts.includes(extend_name)) {
    //         console.log("ok")
    //         console.log(extend_name)
    //         let text = await (await fetch(`${link}`)).text()
    //         return text
    //     }
    // }
    // let [res1] = createResource(() => [extend_name(), link()], get_content);



    return <>
        {/* <button
            onClick={() => {
                console.log(extend_name())
                console.log(props.path)
                console.log(link())
                console.log(small())
            }}
        >
            d
        </button> */}
        <Suspense>
        </Suspense>

        <Switch>

            <Match when={texts.includes(extend_name?.()) && small()}>
                <DisplayText link={link()}></DisplayText>
            </Match>

            <Match when={texts.includes(extend_name?.())}>
                <div id="text_disp">
                    <h2>too big to display</h2>
                </div>
            </Match>

            <Match when={videoes.includes(extend_name?.())}>
                <div id="video_disp">
                    <video controls style={{
                        "border-radius": "1rem",
                        "width": "90%",
                    }}
                        src={link()}></video>
                </div>
            </Match>
            <Match when={pictures.includes(extend_name?.())}>
                <div id="picture_disp">
                    <img controls style={{
                        "width": "90%",
                    }}
                        src={link()}></img>
                </div>
            </Match>

            <Match when={audios.includes(extend_name?.())}>
                <div id="audio_disp">
                    <audio controls
                        style={{
                            "border-radius": "1rem",
                            "width": "90%",
                        }}
                        src={link()}
                    ></audio>
                </div>
            </Match>

        </Switch >


    </>
}

function DisplayText(props) {
    let link = () => props.link
    let get_content = async (link) => {
        let text = await (await fetch(`${link}`)).text()
        // console.log(text)
        return text
    }
    let [res1] = createResource(link, get_content);
    return <>

        <div id="text_disp">
            <pre>
                {res1}
            </pre>
        </div>

    </>



}