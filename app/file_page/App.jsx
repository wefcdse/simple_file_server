import { For, Show, Suspense, createEffect, createResource, createSignal, mergeProps } from 'solid-js'
import './App.css'
import Api from "../public_src/api"
import Media from './media_display'
import { to_style } from '../public_src/utils'

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

const [info] = createResource(path, get_info);

async function get_info(path) {
  let info = await (await fetch(`${Api.info_root}${path}`)).json();
  return info
}




function App() {


  const back_link = () => {
    let b = path.split("/");
    b.pop()
    let c = b.reduce((a, b) => `${a}/${b}`, "/?path=")
    return c
  };

  let size = () => {
    let size = info()?.size
    if (size > 1024 * 1024 * 1024) {
      return Math.floor(size / (1024 * 1024 * 1024) * 10) / 10 + " Gb"
    } else if (size > 1024 * 1024) {
      return Math.floor(size / (1024 * 1024) * 10) / 10 + " Mb"
    } else if (size > 1024) {
      return Math.floor(size / (1024) * 10) / 10 + " Kb"
    } else {
      return size;
    }
  }

  const download_link = () => Api.file_root + path

  return (
    <>
      <div className='top_bar'>


        <a id='back_link' href={back_link()}>back</a>
        {/* 
        <button

          onClick={
            () => {
              let b = path.split("/");
              let name = b.pop()
              console.log(name)

            }
          }
        >
          disp
        </button> */}



      </div>


      <div id='display_name'>
        {info()?.file_name}

      </div>

      <br />

      <div id='download_link'>
        <a href={download_link()} download
          style={
            {
              "padding-right": "100PX",
              // "font-size": "large"
            }
          }
        >
          Download
        </a>
      </div >

      <br />

      <div id='display_size'>
        <span style={to_style({ paddingLeft: "3rem" })}> size: {size}</span>
      </div >





      <br />

      <Suspense>
        <Media path={path} {...(info())}>
          {info()}
        </Media>
        {/* <span>{JSON.stringify(info())}</span> */}
      </Suspense>



    </>
  )
}

export default App
