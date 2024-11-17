import { Show, createSignal, mergeProps } from "solid-js"
import "./sidebar.css"
import clickOutside from "./click-outside"

export default function SideBar(props) {

    let merged = mergeProps({ width: 200, if_show: true }, props)
    let [show, setShow] = createSignal(merged.if_show);
    props.ref({
        setShow: setShow
    })

    return <>
        {/* <Show when={show()}> */}
        <div
            id="sidebar"
            style={
                {
                    width: `${merged.width}px`
                }
            }
            classList={
                { hide: !show() }
            }
            use: clickOutside={
                () => {
                    setShow(false)
                    console.log(props.ref)
                    console.log("out")
                }
            }
        >
            <button
                className="full_width no_border_radius"
                onClick={() => {
                    setShow(false)
                }}
            >{'<<<'}</button>


            <div id="sidebar_link_home">
                <a href="/">
                    home
                </a>
            </div>



        </div>
        {/* </Show > */}
    </>
}