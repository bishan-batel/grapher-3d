import {useEffect, useState} from "react";

// @ts-ignore
export type Crate = typeof import('../crate-build');

export const useCrate = (): Crate | null => {
    const [mod, setMod] = useState<Crate | null>(null);

    useEffect(() => {
        (async () => {

            // Imports module
            let mod = await import('../crate-build/');

            // Calls default to load the WASM binary
            await mod.default();
            // Updates hook
            setMod(mod);
        })();
    }, []);

    return mod;
}