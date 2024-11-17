const to_style = (obj) => {
    let out = {}
    for (let i in obj) {
        let j = i;
        i = i.replace("A", "-a"); i = i.replace("B", "-b"); i = i.replace("C", "-c"); i = i.replace("D", "-d"); i = i.replace("E", "-e"); i = i.replace("F", "-f"); i = i.replace("G", "-g"); i = i.replace("H", "-h"); i = i.replace("I", "-i"); i = i.replace("J", "-j"); i = i.replace("K", "-k"); i = i.replace("L", "-l"); i = i.replace("M", "-m"); i = i.replace("N", "-n"); i = i.replace("O", "-o"); i = i.replace("P", "-p"); i = i.replace("Q", "-q"); i = i.replace("R", "-r"); i = i.replace("S", "-s"); i = i.replace("T", "-t"); i = i.replace("U", "-u"); i = i.replace("V", "-v"); i = i.replace("W", "-w"); i = i.replace("X", "-x"); i = i.replace("Y", "-y"); i = i.replace("Z", "-z");
        out[i] = obj[j]
    }
    return out
}
export { to_style }