import { For, Show, createEffect, createResource, createSignal, mergeProps } from 'solid-js'
import './App.css'
import Api from "../public_src/api"
import Entry from './entry';
import SideBar from '../public_src/sidebar';

const path = (() => {
  let url = new URL(window.location.href);
  let _path = url.searchParams.get("path");
  if (_path == null) {
    _path = "";
  } else if (_path.startsWith("/")) {
    _path = _path.slice(1, _path.length)
  }
  return _path;
})()

const [files] = createResource(path, get_info);

async function get_info(path) {
  let info = await (await fetch(`${Api.info_root}${path}`)).json();
  const compare_entry = (a, b) => {
    // dir < file
    if (a.is_dir && b.is_file) {
      return -1;
    } else if (a.is_file && b.is_dir) {
      return 1;
    }

    if (a.file_name < b.file_name) {
      return -1
    } else if (a.file_name > b.file_name) {
      return 1;
    }
    return 0
  }
  info.entries.sort(compare_entry);
  return info;
}


function App() {

  let side_bar;

  const back_link = () => {
    let b = path.split("/");
    b.pop()
    let c = b.reduce((a, b) => `${a}/${b}`, "/?path=")
    return c
  };



  return (
    <>
      <div className='top_bar'>

        <button
          id='open_side_bar'
          onClick={
            () => {
              // show_side_bar(true)
              side_bar.setShow(true)
              // console.log("12", files().entries)
              // console.log("13", side_bar)
            }
          }
        >side bar</button>

        <Show when={path !== ""}>
          <a href={back_link()}>back</a>
        </Show>

        {/* <button
          onClick={
            () => {
              let b = path.split("/");
              b.pop()
              let c = b.reduce((a, b) => `${a}/${b}`, "/?path=")
              console.log(b);
              console.log(c);
            }
          }
        >
          back
        </button> */}


      </div>


      <SideBar ref={side_bar} if_show={false} />

      <div className='file_display'>

        <For
          each={files()?.entries}
        >
          {
            (e) => {
              return <>
                <Entry path={path} {...e}></Entry>
              </>
            }
          }
        </For>

      </div>
    </>
  )
}

export default App
