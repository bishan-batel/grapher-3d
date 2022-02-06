import {useEffect, useState} from "react";

const useCrate = () => {
    const [mod, setMod] = useState();

    useEffect(() => {
        (async () => {
            // setMod(await import("./crate"))
        })();
    })
    return mod;
}