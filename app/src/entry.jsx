import Api from "../public_src/api"
import "./entry.css"

export default function Entry(props) {
    if (props.is_dir) {
        return <>
            <DirEntry {...props}></DirEntry>
        </>
    }
    else {
        return <>
            <FileEntry {...props}></FileEntry>
        </>
    }
}
function DirEntry(props) {
    // console.log(props)
    let link = () => "/?path=" + props.path + "/" + props.file_name

    return <>
        <div className="entry_bar">
            <a href={link()} className="name" title={props.name}>{props.name}</a>
            <span className="entry_type">dir</span>
        </div>
    </>
}

function FileEntry(props) {
    let size = () => {
        let size = props.size
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
    let link = () => "/file_page/?path=" + props.path + "/" + props.file_name
    let extend_name = () => props.extend_name

    return <>
        <div className="entry_bar">
            <a className="name" href={link()} title={props.name}>{props.name}</a>
            <a className="download_link" download href={Api.file_root + props.path + "/" + props.name}>download</a>
            <span className="entry_type">file</span>
            <span className="size">{size} </span>
            <span className="extend_name">{extend_name}</span>
        </div>
    </>
}